> **⚠️ CRITICAL: Experimental Cipher - NOT FOR PRODUCTION**
>
> This cipher has **NOT been cryptographically audited**.
> Do NOT use this for:
> - Real data encryption
> - Passwords or sensitive data
> - Production systems
> - Any security-critical applications
>
> See [SECURITY.md](SECURITY.md) for details.
>

# QARX-256 (Experimental ARX Block Cipher)

## Name and Design Philosophy

**QARX = Quantum-Aware Add-Rotate-XOR**

- **Q – Quantum-Aware**  
  Design generic quantum attacks (especially Grover-style search) ko dhyaan me rakh kar kiya gaya hai, effective security bits ko high rakhne ke goal ke saath.

- **A – Add**  
  Modular addition operations ARX construction ka core part hain.

- **R – Rotate**  
  Bit rotations diffusion aur avalanche effect improve karne ke liye use kiye gaye hain.

- **X – XOR**  
  XOR operations non-linearity aur state mixing ke liye use hote hain.

- **256**  
  256-bit block size (4 × 64-bit words) ka large state brute force aur generic quantum search ko computationally mehenga banane ke research goal ko reflect karta hai.


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
