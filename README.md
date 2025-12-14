# QARX-256 (Experimental ARX Block Cipher)

QARX-256 is an **experimental 256-bit block cipher** designed as a research project, not as a production-ready encryption scheme.

- Block size: 256 bits (4 × 64-bit words)
- Key size: 512 bits (8 × 64-bit words)
- Rounds: 24
- Design style: ARX (Add-Rotate-XOR) with XOR-only linear diffusion
- Key schedule: SHA3-512(key || round_index)

> ⚠️ **WARNING**  
> QARX-256 is **EXPERIMENTAL** and has **not** undergone public cryptanalysis.  
> It is **NOT** suitable for protecting real-world sensitive data.  
> Intended use: research, lab experiments, malware/ransomware simulation, and red-team tooling in controlled environments only.

## Project Layout

- `src/qarx256_core.rs` – core QARX-256 block cipher (encrypt/decrypt, key schedule)
- `src/qarx256_kdf.rs` – experimental SHA3-based KDF (placeholder, Argon2 hook for future)
- `src/qarx256_modes.rs` – adaptive / entropy-aware engine (lab / obfuscation feature)
- `src/main.rs` – simple demo / self-test

## Status

- [x] Core cipher implemented
- [x] Reference implementation in Rust
- [x] Basic roundtrip tests
- [ ] Differential / linear cryptanalysis for reduced rounds
- [ ] CTR / XTS modes for file encryption labs
- [ ] Proper Argon2-based KDF

## Disclaimer

This project is for **learning and research**.  
Use NIST-approved standards like AES-256 and standardized post-quantum algorithms for any real-world security.
