// QARX-256 - Experimental ARX Block Cipher
// WARNING: This is EXPERIMENTAL software. Not for production use.
// Reference implementation for research and lab purposes only.

mod qarx256_core;
mod qarx256_kdf;
mod qarx256_modes;

use qarx256_core::{qarx256_key_setup, qarx256_encrypt_block, qarx256_decrypt_block, Qarx256Ctx, BLOCK_SIZE, KEY_SIZE};

fn main() {
    println!("QARX-256 Experimental Cipher");
    println!("Block size: 256 bits (4 x 64-bit words)");
    println!("Key size: 512 bits (8 x 64-bit words)");
    println!("Rounds: 24");
    println!("\nDesign: ARX (Add-Rotate-XOR) with XOR-only linear diffusion");
    println!("Key schedule: SHA3-512(key || round_index)");
    println!("\n[!] WARNING: EXPERIMENTAL - Not suitable for production data");
    println!("[+] Intended for: research, labs, malware simulation, red-team exercises");

    // Simple roundtrip test
    println!("\nRunning basic roundtrip test...");
    let mut ctx = Qarx256Ctx::default();
    let key = [0u8; KEY_SIZE];
    qarx256_key_setup(&mut ctx, &key);

    let plaintext = b"Hello, QARX-256! This is a test message for encryption.";
    let mut padded_plaintext = [0u8; BLOCK_SIZE];
    padded_plaintext[..plaintext.len()].copy_from_slice(plaintext);

    let ciphertext = qarx256_encrypt_block(&ctx, &padded_plaintext);
    let decrypted = qarx256_decrypt_block(&ctx, &ciphertext);

    if padded_plaintext == decrypted {
        println!("✓ Roundtrip test PASSED: Encryption and decryption work correctly.");
    } else {
        println!("✗ Roundtrip test FAILED: Decrypted text does not match original.");
    }

    println!("Original:  {:?}", &padded_plaintext[..plaintext.len()]);
    println!("Decrypted: {:?}", &decrypted[..plaintext.len()]);
}
