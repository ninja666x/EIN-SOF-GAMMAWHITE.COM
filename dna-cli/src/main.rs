//! EIN SOF DNA-CLI — Commit, don't reveal.
//! Copyright © 2026 Tuomas Johannes Karjalainen — 8:30 AM EET, Turku

use aes_gcm::{aead::{Aead, KeyInit, OsRng}, Aes256Gcm, Nonce};
use clap::{Parser, Subcommand};
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dna-cli", version, about = "EIN SOF DNA commitment tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Laske SHA-256-sitoumus raakadatasta
    Commit {
        /// Syötetiedosto (23andMe .txt tai .vcf)
        #[arg(short, long)]
        input: PathBuf,
        /// Tulostiedosto (commitment hex)
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Salaa raakadata AES-256-GCM:llä
    Encrypt {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        /// Avain tiedostossa (32 tavua hex)
        #[arg(short, long)]
        key: PathBuf,
    },
    /// Vertaa sitoumusta raakadataan
    Verify {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        commitment: PathBuf,
    },
}

fn commit(input: &PathBuf, output: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let raw = fs::read(input)?;
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);

    let mut hasher = Sha256::new();
    hasher.update(&salt);
    hasher.update(&raw);
    let hash = hasher.finalize();

    // Tuloste: salt (hex) + hash (hex)
    let mut out = String::new();
    out.push_str(&format!("salt={}\n", hex::encode(salt)));
    out.push_str(&format!("commitment={}\n", hex::encode(hash)));
    fs::write(output, out)?;

    println!("✅ Sitoumus laskettu: {}", hex::encode(hash));
    println!("⚠️  Säilytä salt turvallisesti — ilman sitä et voi todentaa.");
    Ok(())
}

fn encrypt(input: &PathBuf, output: &PathBuf, key_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let raw = fs::read(input)?;
    let key_hex = fs::read_to_string(key_path)?.trim().to_string();
    let key_bytes = hex::decode(&key_hex)?;
    if key_bytes.len() != 32 {
        return Err("Avaimen on oltava 32 tavua (64 hex-merkkiä)".into());
    }
    let cipher = Aes256Gcm::new_from_slice(&key_bytes).map_err(|_| "Virheellinen avain")?;

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, raw.as_ref())
        .map_err(|_| "Salaus epäonnistui")?;

    let mut out = Vec::new();
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    fs::write(output, &out)?;

    println!("✅ Salattu: {} tavua", out.len());
    println!("⚠️  Avain: pidä offline-tilassa, älä koskaan lohkoketjuun.");
    Ok(())
}

fn verify(input: &PathBuf, commitment_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let raw = fs::read(input)?;
    let content = fs::read_to_string(commitment_path)?;
    let mut salt_hex = String::new();
    let mut expected_hex = String::new();
    for line in content.lines() {
        if let Some(v) = line.strip_prefix("salt=") { salt_hex = v.to_string(); }
        if let Some(v) = line.strip_prefix("commitment=") { expected_hex = v.to_string(); }
    }
    let salt = hex::decode(&salt_hex)?;
    let expected = hex::decode(&expected_hex)?;

    let mut hasher = Sha256::new();
    hasher.update(&salt);
    hasher.update(&raw);
    let actual = hasher.finalize();

    if actual.as_slice() == expected.as_slice() {
        println!("✅ Sitoumus täsmää. Data on alkuperäistä.");
    } else {
        println!("❌ Sitoumus ei täsmää. Data on muuttunut.");
        std::process::exit(1);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Commit { input, output } => commit(input, output),
        Commands::Encrypt { input, output, key } => encrypt(input, output, key),
        Commands::Verify { input, commitment } => verify(input, commitment),
    }
}
