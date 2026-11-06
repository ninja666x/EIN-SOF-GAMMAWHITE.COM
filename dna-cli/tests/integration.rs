// EIN SOF DNA-CLI — integraatiotestit
// Copyright © 2026 Tuomas Johannes Karjalainen

use std::fs;
use std::process::Command;

fn cli_path() -> &'static str {
    env!("CARGO_BIN_EXE_dna-cli")
}

#[test]
fn commit_creates_output_with_salt_and_hash() {
    let tmp = tempdir();
    let input = tmp.join("raw.txt");
    let output = tmp.join("commitment.txt");
    fs::write(&input, b"ATCGATCGATCG...fake-dna...").unwrap();

    let status = Command::new(cli_path())
        .args(["commit", "-i", input.to_str().unwrap(),
               "-o", output.to_str().unwrap()])
        .status().unwrap();
    assert!(status.success());

    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("salt="));
    assert!(content.contains("commitment="));

    let commitment_line = content.lines()
        .find(|l| l.starts_with("commitment=")).unwrap();
    let hash_hex = commitment_line.trim_start_matches("commitment=");
    assert_eq!(hash_hex.len(), 64, "SHA-256 = 64 hex-merkkiä");
}

#[test]
fn verify_passes_for_original_data() {
    let tmp = tempdir();
    let input = tmp.join("raw.txt");
    let output = tmp.join("c.txt");
    fs::write(&input, b"original-dna").unwrap();

    Command::new(cli_path())
        .args(["commit", "-i", input.to_str().unwrap(),
               "-o", output.to_str().unwrap()])
        .status().unwrap();

    let status = Command::new(cli_path())
        .args(["verify", "-i", input.to_str().unwrap(),
               "-c", output.to_str().unwrap()])
        .status().unwrap();
    assert!(status.success());
}

#[test]
fn verify_fails_for_tampered_data() {
    let tmp = tempdir();
    let input = tmp.join("raw.txt");
    let output = tmp.join("c.txt");
    fs::write(&input, b"original-dna").unwrap();

    Command::new(cli_path())
        .args(["commit", "-i", input.to_str().unwrap(),
               "-o", output.to_str().unwrap()])
        .status().unwrap();

    // Muuta dataa
    fs::write(&input, b"tampered-dna").unwrap();

    let status = Command::new(cli_path())
        .args(["verify", "-i", input.to_str().unwrap(),
               "-c", output.to_str().unwrap()])
        .status().unwrap();
    assert!(!status.success(), "verify:n pitäisi epäonnistua");
}

#[test]
fn encrypt_roundtrip() {
    use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce};
    use rand::RngCore;

    let tmp = tempdir();
    let input = tmp.join("plain.txt");
    let enc = tmp.join("cipher.enc");
    let key_path = tmp.join("key.hex");

    let data = b"salainen-genomi-...";
    fs::write(&input, data).unwrap();

    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    let key_hex: String = key.iter().map(|b| format!("{:02x}", b)).collect();
    fs::write(&key_path, key_hex.as_bytes()).unwrap();

    let status = Command::new(cli_path())
        .args(["encrypt", "-i", input.to_str().unwrap(),
               "-o", enc.to_str().unwrap(),
               "-k", key_path.to_str().unwrap()])
        .status().unwrap();
    assert!(status.success());

    // Pura
    let cipher_bytes = fs::read(&enc).unwrap();
    let (nonce_bytes, ct) = cipher_bytes.split_at(12);
    let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
    let plaintext = cipher.decrypt(Nonce::from_slice(nonce_bytes), ct).unwrap();
    assert_eq!(&plaintext[..], data);
}

// --- apu ---
fn tempdir() -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!(
        "dna-cli-test-{}-{}",
        std::process::id(),
        rand::random::<u32>()
    ));
    fs::create_dir_all(&p).unwrap();
    p
}
