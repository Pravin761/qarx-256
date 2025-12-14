$ cargo build --release
./target/release/qarx-256

   Compiling qarx-256 v0.1.0 (/home/kali/qarx-256)
error[E0425]: cannot find function `round_encrypt` in this scope
   --> src/qarx256_modes.rs:75:21
    |
 75 |             state = round_encrypt(state, self.ctx.rk[i % ROUNDS]);
    |                     ^^^^^^^^^^^^^ not found in this scope
    |
note: function `crate::qarx256_core::round_encrypt` exists but is inaccessible
   --> src/qarx256_core.rs:106:1
    |
106 | fn round_encrypt(mut x: [u64; 4], rk: [u64; 4]) -> [u64; 4] {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

error[E0659]: `KEY_SIZE` is ambiguous
  --> src/main.rs:27:22
   |
27 |     let key = [42u8; KEY_SIZE];
   |                      ^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of multiple glob imports of a name in the same module
note: `KEY_SIZE` could refer to the constant imported here
  --> src/main.rs:7:5
   |
 7 | use qarx256_core::*;
   |     ^^^^^^^^^^^^^^^
   = help: consider adding an explicit import of `KEY_SIZE` to disambiguate
note: `KEY_SIZE` could also refer to the constant imported here
  --> src/main.rs:8:5
   |
 8 | use qarx256_kdf::*;
   |     ^^^^^^^^^^^^^^
   = help: consider adding an explicit import of `KEY_SIZE` to disambiguate

warning: unused import: `crate::BLOCK_SIZE`
 --> src/qarx256_kdf.rs:4:5
  |
4 | use crate::BLOCK_SIZE;
  |     ^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

error[E0599]: no method named `clone` found for struct `qarx256_core::Qarx256Ctx` in the current scope
  --> src/main.rs:52:51
   |
52 |     let adaptive = Qarx256AdaptiveEngine::new(ctx.clone());
   |                                                   ^^^^^ method not found in `qarx256_core::Qarx256Ctx`
   |
  ::: src/qarx256_core.rs:22:1
   |
22 | pub struct Qarx256Ctx {
   | --------------------- method `clone` not found for this struct
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `clone`, perhaps you need to implement it:
           candidate #1: `Clone`

warning: value assigned to `x1` is never read
   --> src/qarx256_core.rs:124:5
    |
124 |     x1 ^= x0;
    |     ^^^^^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `x2` is never read
   --> src/qarx256_core.rs:125:5
    |
125 |     x2 ^= x3;
    |     ^^^^^^^^
    |
    = help: maybe it is overwritten before being read?

Some errors have detailed explanations: E0425, E0599, E0659.
For more information about an error, try `rustc --explain E0425`.
warning: `qarx-256` (bin "qarx-256") generated 3 warnings
error: could not compile `qarx-256` (bin "qarx-256") due to 3 previous errors; 3 warnings emitted
QARX-256 Experimental Cipher
Block size: 256 bits (4 x 64-bit words)
Key size: 512 bits (8 x 64-bit words)
Rounds: 24

Design: ARX (Add-Rotate-XOR) with XOR-only linear diffusion
Key schedule: SHA3-512(key || round_index)

[!] WARNING: EXPERIMENTAL - Not suitable for production data
[+] Intended for: research, labs, malware simulation, red-team exercises
                                                                                                                              
┌──(kali㉿kali)-[~/qarx-256]
└─$ 
