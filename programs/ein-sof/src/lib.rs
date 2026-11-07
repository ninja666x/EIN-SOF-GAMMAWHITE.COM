//! EIN SOF TOKEN — Solana Anchor -ohjelma
//! Copyright © 2026 Tuomas Johannes Karjalainen (s. 1971-09-07 13:30 UTC / 15:30 Turku, 070971-119X)

use anchor_lang::prelude::*;

// Paikkamerkki-ID. Aja `anchor keys sync` ensimmäisen `anchor build`:n jälkeen.
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// ============================================================
// VAKIOT
// ============================================================

pub const BASIS_POINTS: u64 = 10_000;
pub const EMPATHY_REJECTED: u16 = 2_000;   // < 0.20
pub const EMPATHY_RESTRICTED: u16 = 5_000; // < 0.50
pub const EMPATHY_ACCEPTED: u16 = 8_000;   // < 0.80
pub const DEMURRAGE_LAMBDA_BPS: u64 = 100; // 1.00 % / kk
pub const SECONDS_PER_MONTH: i64 = 30 * 24 * 3600;
pub const BAN_THRESHOLD: i32 = -100;

// ============================================================
// TILIT
// ============================================================

#[account]
#[derive(InitSpace)]
pub struct Identity {
    pub owner: Pubkey,
    /// Empatiapisteet basis pointeina (0–10 000)
    pub empathy_bps: u16,
    /// Mainepisteet (voi olla negatiivinen)
    pub reputation: i32,
    /// Onko porttikielto
    pub banned: bool,
    /// DNA:n SHA-256 -sitoumus (EI raakaa DNA:ta)
    pub dna_commitment: [u8; 32],
    /// Onko DNA-sitoumus julkinen
    pub dna_public: bool,
    /// Milloin empatiatesti on viimeksi suoritettu
    pub last_empathy_test: i64,
    pub bump: u8,
}

impl Identity {
    pub fn can_trade(&self) -> bool {
        !self.banned && self.empathy_bps >= EMPATHY_RESTRICTED
    }
    pub fn can_vote(&self) -> bool {
        !self.banned && self.empathy_bps >= EMPATHY_ACCEPTED
    }
    pub fn adjust_reputation(&mut self, delta: i32) -> Result<()> {
        self.reputation = self.reputation.saturating_add(delta);
        if self.reputation <= BAN_THRESHOLD {
            self.banned = true;
        }
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct EsofToken {
    pub mint: Pubkey,
    pub nominal: u64,        // lamports-yksikköinä
    pub created_at: i64,
    pub lambda_bps: u64,     // demurrage-kerroin
    pub bump: u8,
}

impl EsofToken {
    /// arvo(t) = nominal * e^(-λ t)
    /// Lasketaan likimääräisesti (integer-approksimaatio).
    pub fn value_at(&self, now: i64) -> u64 {
        let age = (now - self.created_at).max(0);
        let months = age / SECONDS_PER_MONTH;
        // Approksimaatio: nominal * (1 - λ)^months
        let lambda = self.lambda_bps as f64 / BASIS_POINTS as f64;
        let factor = (1.0 - lambda).powi(months as i32);
        (self.nominal as f64 * factor) as u64
    }
}

#[account]
#[derive(InitSpace)]
pub struct Dispute {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    #[max_len(256)]
    pub reason: String,
    pub votes_buyer: u16,
    pub votes_seller: u16,
    pub status: DisputeStatus,
    pub created_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum DisputeStatus {
    Open,
    ResolvedBuyer,
    ResolvedSeller,
    Tied,
}

#[account]
#[derive(InitSpace)]
pub struct VoteRecord {
    pub dispute: Pubkey,
    pub voter: Pubkey,
    pub side: VoteSide,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum VoteSide {
    Buyer,
    Seller,
}

// ============================================================
// INSTRUKTIOT
// ============================================================

#[program]
pub mod ein_sof {
    use super::*;

    /// Luo uuden identiteetin. DNA-sitoumus annetaan valmiiksi laskettuna.
    pub fn initialize_identity(
        ctx: Context<InitializeIdentity>,
        dna_commitment: [u8; 32],
        dna_public: bool,
    ) -> Result<()> {
        let id = &mut ctx.accounts.identity;
        id.owner = ctx.accounts.owner.key();
        id.empathy_bps = 0;
        id.reputation = 0;
        id.banned = false;
        id.dna_commitment = dna_commitment;
        id.dna_public = dna_public;
        id.last_empathy_test = 0;
        id.bump = ctx.bumps.identity;
        Ok(())
    }

    /// Päivittää empatiapisteen. Vain valtuutettu oraakkeli voi kutsua.
    pub fn update_empathy(
        ctx: Context<UpdateEmpathy>,
        new_score_bps: u16,
    ) -> Result<()> {
        require!(new_score_bps <= BASIS_POINTS as u16, EsofError::InvalidScore);
        let id = &mut ctx.accounts.identity;
        id.empathy_bps = new_score_bps;
        id.last_empathy_test = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// Todentaa, että annettu DNA-sitoumus vastaa tallennettua.
    /// (ZKP todistetaan off-chain; tämä on yksinkertainen vertailu.)
    pub fn verify_dna_commitment(
        ctx: Context<VerifyDna>,
        claimed_commitment: [u8; 32],
    ) -> Result<bool> {
        Ok(ctx.accounts.identity.dna_commitment == claimed_commitment)
    }

    /// Luo uuden E-SOF-tokenin.
    pub fn initialize_token(ctx: Context<InitializeToken>, nominal: u64) -> Result<()> {
        let t = &mut ctx.accounts.token;
        t.mint = ctx.accounts.mint.key();
        t.nominal = nominal;
        t.created_at = Clock::get()?.unix_timestamp;
        t.lambda_bps = DEMURRAGE_LAMBDA_BPS;
        t.bump = ctx.bumps.token;
        Ok(())
    }

    /// Avaa riidan.
    pub fn file_dispute(
        ctx: Context<FileDispute>,
        amount: u64,
        reason: String,
    ) -> Result<()> {
        require!(reason.len() <= 256, EsofError::ReasonTooLong);
        let d = &mut ctx.accounts.dispute;
        d.buyer = ctx.accounts.buyer.key();
        d.seller = ctx.accounts.seller.key();
        d.amount = amount;
        d.reason = reason;
        d.votes_buyer = 0;
        d.votes_seller = 0;
        d.status = DisputeStatus::Open;
        d.created_at = Clock::get()?.unix_timestamp;
        d.bump = ctx.bumps.dispute;
        Ok(())
    }

    /// Solmu äänestää riidassa.
    pub fn vote_dispute(ctx: Context<VoteDispute>, side: VoteSide) -> Result<()> {
        let voter_id = &ctx.accounts.voter_identity;
        require!(voter_id.can_vote(), EsofError::NotAuthorizedToVote);

        let d = &mut ctx.accounts.dispute;
        require!(d.status == DisputeStatus::Open, EsofError::DisputeClosed);

        match side {
            VoteSide::Buyer => d.votes_buyer += 1,
            VoteSide::Seller => d.votes_seller += 1,
        }

        let record = &mut ctx.accounts.vote_record;
        record.dispute = d.key();
        record.voter = voter_id.owner;
        record.side = side;
        record.bump = ctx.bumps.vote_record;
        Ok(())
    }

    /// Ratkaisee riidan ja päivittää maineet.
    pub fn resolve_dispute(ctx: Context<ResolveDispute>) -> Result<()> {
        let d = &mut ctx.accounts.dispute;
        require!(d.status == DisputeStatus::Open, EsofError::DisputeClosed);

        let buyer_id = &mut ctx.accounts.buyer_identity;
        let seller_id = &mut ctx.accounts.seller_identity;

        if d.votes_buyer > d.votes_seller {
            d.status = DisputeStatus::ResolvedBuyer;
            buyer_id.adjust_reputation(2)?;
            seller_id.adjust_reputation(-5)?;
        } else if d.votes_seller > d.votes_buyer {
            d.status = DisputeStatus::ResolvedSeller;
            buyer_id.adjust_reputation(-5)?;
            seller_id.adjust_reputation(2)?;
        } else {
            d.status = DisputeStatus::Tied;
        }
        Ok(())
    }
}

// ============================================================
// TILIKONTEKSTIT
// ============================================================

#[derive(Accounts)]
pub struct InitializeIdentity<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Identity::INIT_SPACE,
        seeds = [b"identity", owner.key().as_ref()],
        bump
    )]
    pub identity: Account<'info, Identity>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateEmpathy<'info> {
    #[account(mut, seeds = [b"identity", identity.owner.as_ref()], bump = identity.bump)]
    pub identity: Account<'info, Identity>,
    /// Oraakkelin allekirjoitus.
    /// TODO (turvallisuus): nyt KUKA TAHANSA allekirjoittaja voi asettaa pisteet.
    /// Ennen mainnetiä: lisää Config-tili ja tarkista oracle-whitelist.
    pub oracle: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerifyDna<'info> {
    #[account(seeds = [b"identity", identity.owner.as_ref()], bump = identity.bump)]
    pub identity: Account<'info, Identity>,
}

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(init, payer = payer, space = 8 + EsofToken::INIT_SPACE,
              seeds = [b"esof-token", mint.key().as_ref()], bump)]
    pub token: Account<'info, EsofToken>,
    /// CHECK: SPL-mint (oikea token); vain avain tallennetaan, dataa ei lueta
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FileDispute<'info> {
    #[account(init, payer = buyer, space = 8 + Dispute::INIT_SPACE,
              seeds = [b"dispute", buyer.key().as_ref(), seller.key().as_ref()],
              bump)]
    pub dispute: Account<'info, Dispute>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: vain Pubkey, ei dataa
    pub seller: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteDispute<'info> {
    #[account(mut)]
    pub dispute: Account<'info, Dispute>,
    #[account(seeds = [b"identity", voter_identity.owner.as_ref()],
              bump = voter_identity.bump)]
    pub voter_identity: Account<'info, Identity>,
    #[account(init, payer = voter, space = 8 + VoteRecord::INIT_SPACE,
              seeds = [b"vote", dispute.key().as_ref(), voter_identity.owner.as_ref()],
              bump)]
    pub vote_record: Account<'info, VoteRecord>,
    #[account(mut)]
    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResolveDispute<'info> {
    #[account(mut)]
    pub dispute: Account<'info, Dispute>,
    #[account(mut, seeds = [b"identity", dispute.buyer.as_ref()], bump = buyer_identity.bump)]
    pub buyer_identity: Account<'info, Identity>,
    #[account(mut, seeds = [b"identity", dispute.seller.as_ref()], bump = seller_identity.bump)]
    pub seller_identity: Account<'info, Identity>,
}

// ============================================================
// VIRHEET
// ============================================================

#[error_code]
pub enum EsofError {
    #[msg("Empatiapiste ei ole kelvollinen (0–10000).")]
    InvalidScore,
    #[msg("Vain :node-luokituksen identiteetti voi äänestää.")]
    NotAuthorizedToVote,
    #[msg("Riita on jo ratkaistu.")]
    DisputeClosed,
    #[msg("Syy on liian pitkä (max 256 merkkiä).")]
    ReasonTooLong,
}
