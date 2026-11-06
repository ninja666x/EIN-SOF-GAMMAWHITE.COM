//! EIN SOF — Paikallistalouden simulaatio 1000 agentilla
//! Copyright © 2026 Tuomas Johannes Karjalainen — 8:30 AM EET, Turku
//!
//! Simuloi:
//!   - 1000 työntekijää (biometria, stressi, hyvinvointi)
//!   - 50 sijoittajaa
//!   - 100 jaksoa
//!   - Demurrage, voitonjako, maine, bannaus

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

const N_WORKERS: usize = 1000;
const N_INVESTORS: usize = 50;
const N_PERIODS: usize = 100;
const DEMURRAGE_LAMBDA: f64 = 0.01;
const BAN_THRESHOLD: i32 = -100;

#[derive(Clone, Debug)]
struct Worker {
    id: u32,
    empathy: f64,        // 0.0–1.0
    reputation: i32,
    banned: bool,
    hrv_baseline: f64,
    hrv_current: f64,
    stress: f64,
    balance: f64,
}

impl Worker {
    fn wellbeing(&self) -> f64 {
        (self.hrv_current / self.hrv_baseline.max(0.001)) * (1.0 - self.stress)
    }

    fn can_trade(&self) -> bool {
        !self.banned && self.empathy >= 0.2
    }

    fn can_vote(&self) -> bool {
        !self.banned && self.empathy >= 0.8
    }

    fn adjust_rep(&mut self, delta: i32) {
        self.reputation = self.reputation.saturating_add(delta);
        if self.reputation <= BAN_THRESHOLD {
            self.banned = true;
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Investor {
    id: u32,
    capital: f64,
    balance: f64,
}

#[derive(Debug, Default)]
struct PeriodStats {
    profit_total: f64,
    worker_share_total: f64,
    investor_share_total: f64,
    wellness_avg: f64,
    new_bans: usize,
    trades: usize,
    losses: usize,
}

fn spawn_workers(rng: &mut StdRng) -> Vec<Worker> {
    (0..N_WORKERS).map(|i| {
        // Jakauma: 5 % psykopaatteja, 20 % matala, 60 % normaali, 15 % korkea
        let empathy = match rng.gen::<f64>() {
            x if x < 0.05 => rng.gen_range(0.0..0.2),   // psykopaatti
            x if x < 0.25 => rng.gen_range(0.2..0.5),   // rajoitettu
            x if x < 0.85 => rng.gen_range(0.5..0.8),   // hyväksytty
            _            => rng.gen_range(0.8..1.0),   // solmu
        };
        Worker {
            id: i as u32,
            empathy,
            reputation: 0,
            banned: false,
            hrv_baseline: 1.0,
            hrv_current: rng.gen_range(0.6..1.0),
            stress: rng.gen_range(0.0..0.8),
            balance: rng.gen_range(10.0..100.0),
        }
    }).collect()
}

fn spawn_investors(rng: &mut StdRng) -> Vec<Investor> {
    (0..N_INVESTORS).map(|i| Investor {
        id: i as u32,
        capital: rng.gen_range(1_000.0..100_000.0),
        balance: rng.gen_range(1_000.0..100_000.0),
    }).collect()
}

fn simulate_period(
    period: usize,
    workers: &mut Vec<Worker>,
    investors: &mut Vec<Investor>,
    rng: &mut StdRng,
) -> PeriodStats {
    // 1. Päivitä biometria
    for w in workers.iter_mut() {
        if w.banned { continue; }
        w.hrv_current = (w.hrv_current + rng.gen_range(-0.05..0.05)).clamp(0.3, 1.0);
        w.stress = (w.stress + rng.gen_range(-0.05..0.05)).clamp(0.0, 1.0);
    }

    // 2. Kauppasimulaatio (vain :accepted ja :node voivat käydä kauppaa)
    let trade_candidates: Vec<usize> = workers.iter()
        .enumerate()
        .filter(|(_, w)| w.can_trade())
        .map(|(i, _)| i)
        .collect();

    let mut stats = PeriodStats::default();
    let trades_n = trade_candidates.len().min(200);
    for _ in 0..trades_n {
        let i = trade_candidates[rng.gen_range(0..trade_candidates.len())];
        let j = trade_candidates[rng.gen_range(0..trade_candidates.len())];
        if i == j { continue; }
        let amount = rng.gen_range(1.0..20.0);
        if workers[i].balance >= amount {
            workers[i].balance -= amount;
            workers[j].balance += amount;
            workers[i].adjust_rep(1);
            workers[j].adjust_rep(1);
            stats.trades += 1;
        }
    }

    // 3. Jakson tilitys: satunnainen liikevaihto ja kulut
    let revenue = rng.gen_range(5_000.0..20_000.0);
    let costs   = rng.gen_range(3_000.0..25_000.0);
    let profit = revenue - costs;

    let wellness_sum: f64 = workers.iter()
        .filter(|w| !w.banned)
        .map(|w| w.wellbeing())
        .sum();
    let active = workers.iter().filter(|w| !w.banned).count().max(1);
    let wellness = wellness_sum / active as f64;
    let adjusted = profit * wellness;

    let (worker_share, investor_share) = if adjusted >= 0.0 {
        (adjusted * 0.5, adjusted * 0.5)
    } else {
        (0.0, adjusted) // 100 % tappio sijoittajille
    };

    // 4. Jaa työntekijöille
    let mut active_workers: Vec<&mut Worker> = workers.iter_mut()
        .filter(|w| !w.banned)
        .collect();
    let per_worker = if active_workers.is_empty() {
        0.0
    } else {
        worker_share / active_workers.len() as f64
    };
    for w in active_workers.iter_mut() {
        w.balance += per_worker;
    }

    // 5. Jaa sijoittajille (tai laskuta tappio)
    let n_inv = investors.len().max(1);
    let per_inv = investor_share / n_inv as f64;
    for inv in investors.iter_mut() {
        inv.balance += per_inv;
    }

    // 6. Demurrage (raha vanhenee)
    let factor = (-DEMURRAGE_LAMBDA * 1.0).exp(); // 1 kk
    for w in workers.iter_mut() { w.balance *= factor; }
    for inv in investors.iter_mut() { inv.balance *= factor; }

    // 7. Reklamaatiot: satunnaisesti 5 % kaupoista epäonnistuu → maine
    let reklamations = (trades_n as f64 * 0.05) as usize;
    for _ in 0..reklamations {
        let i = rng.gen_range(0..workers.len());
        if workers[i].banned { continue; }
        workers[i].adjust_rep(-5);
        if workers[i].reputation <= BAN_THRESHOLD {
            stats.new_bans += 1;
        }
    }

    if adjusted < 0.0 { stats.losses += 1; }

    stats.profit_total = profit;
    stats.worker_share_total = worker_share;
    stats.investor_share_total = investor_share;
    stats.wellness_avg = wellness;

    if period % 20 == 0 || period == N_PERIODS - 1 {
        println!(
            "Jakso {:>3}: profit={:>9.2}  wellness={:.3}  \
             bans={:>3}  trades={:>4}  worker_avg={:.2}",
            period, profit, wellness,
            workers.iter().filter(|w| w.banned).count(),
            stats.trades,
            workers.iter().map(|w| w.balance).sum::<f64>() / N_WORKERS as f64,
        );
    }

    stats
}

fn main() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut workers = spawn_workers(&mut rng);
    let mut investors = spawn_investors(&mut rng);

    // Alkutilastot
    println!("=== EIN SOF — 1000 agentin simulaatio ===\n");
    let dist = |lo: f64, hi: f64| workers.iter()
        .filter(|w| w.empathy >= lo && w.empathy < hi)
        .count();
    println!("Empatiajakauma:");
    println!("  rejected   (0.0–0.2): {:>4}", dist(0.0, 0.2));
    println!("  restricted (0.2–0.5): {:>4}", dist(0.2, 0.5));
    println!("  accepted   (0.5–0.8): {:>4}", dist(0.5, 0.8));
    println!("  node       (0.8–1.0): {:>4}\n", dist(0.8, 1.01));

    let mut total_profit = 0.0;
    let mut total_worker = 0.0;
    let mut total_investor = 0.0;
    let mut total_wellness = 0.0;

    for p in 0..N_PERIODS {
        let s = simulate_period(p, &mut workers, &mut investors, &mut rng);
        total_profit   += s.profit_total;
        total_worker   += s.worker_share_total;
        total_investor += s.investor_share_total;
        total_wellness += s.wellness_avg;
    }

    println!("\n=== YHTEENVETO ({} jaksoa) ===", N_PERIODS);
    println!("Kokonaisvoitto:        {:>12.2}", total_profit);
    println!("Työntekijöiden osuus:  {:>12.2}", total_worker);
    println!("Sijoittajien osuus:    {:>12.2}", total_investor);
    println!("Keskim. hyvinvointi:   {:>12.3}", total_wellness / N_PERIODS as f64);
    println!("Bannatut:              {:>12}",
             workers.iter().filter(|w| w.banned).count());

    let w_balances: f64 = workers.par_iter().map(|w| w.balance).sum();
    let i_balances: f64 = investors.par_iter().map(|i| i.balance).sum();
    println!("Työntekijöiden varat:  {:>12.2}", w_balances);
    println!("Sijoittajien varat:    {:>12.2}", i_balances);

    // Gini-kerroin (yksinkertaistettu)
    let mut balances: Vec<f64> = workers.iter().map(|w| w.balance).collect();
    balances.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = balances.len() as f64;
    let sum: f64 = balances.iter().sum();
    let gini = if sum > 0.0 {
        let mut cum = 0.0;
        let mut num = 0.0;
        for (i, b) in balances.iter().enumerate() {
            cum += b;
            num += (2.0 * (i as f64 + 1.0) - n - 1.0) * cum;
        }
        num / (n * sum)
    } else { 0.0 };
    println!("Gini (työntekijät):    {:>12.4}", gini);
}
