// qarx256_core.rs - Clean, standard QARX-256 core cipher

use sha3::{Digest, Sha3_512};

pub const BLOCK_SIZE: usize = 32; // 256 bits
pub const KEY_SIZE: usize = 64;   // 512 bits
pub const ROUNDS: usize = 24;

// Fixed rotation constants
const ROT0: u32 = 13;
const ROT1: u32 = 37;
const ROT2: u32 = 51;
const ROT3: u32 = 7;

// Diffusion constants
const DIFFUSION_0: u64 = 0x123456789ABCDEF0;
const DIFFUSION_1: u64 = 0xFEDCBA9876543210;
const DIFFUSION_2: u64 = 0xA1B2C3D4E5F67890;
const DIFFUSION_3: u64 = 0x09876543210FEDC;

// Core cipher context
#[derive(Clone)]
pub struct Qarx256Ctx {
    pub rk: [[u64; 4]; ROUNDS],
    pub iv: Option<[u64; 4]>,
}

impl Default for Qarx256Ctx {
    fn default() -> Self {
        Qarx256Ctx {
            rk: [[0u64; 4]; ROUNDS],
            iv: None,
        }
    }
}

// Key setup using standard PRF
pub fn qarx256_key_setup(ctx: &mut Qarx256Ctx, key: &[u8; KEY_SIZE]) {
    let mut buf = [0u8; KEY_SIZE + 8];
    buf[..KEY_SIZE].copy_from_slice(key);

    for i in 0..ROUNDS {
        buf[KEY_SIZE..].copy_from_slice(&(i as u64).to_le_bytes());
        let digest = Sha3_512::digest(&buf);

        for j in 0..4 {
            let start = j * 8;
            ctx.rk[i][j] = u64::from_le_bytes(digest[start..start + 8].try_into().unwrap());
        }
    }
}

// Standard block encryption
pub fn qarx256_encrypt_block(ctx: &Qarx256Ctx, input: &[u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
    let mut state = [
        u64::from_le_bytes(input[0..8].try_into().unwrap()),
        u64::from_le_bytes(input[8..16].try_into().unwrap()),
        u64::from_le_bytes(input[16..24].try_into().unwrap()),
        u64::from_le_bytes(input[24..32].try_into().unwrap()),
    ];

    if let Some(iv) = ctx.iv {
        state[0] ^= iv[0];
        state[1] ^= iv[1];
        state[2] ^= iv[2];
        state[3] ^= iv[3];
    }

    for i in 0..ROUNDS {
        state = round_encrypt(state, ctx.rk[i]);
    }

    let mut output = [0u8; BLOCK_SIZE];
    for i in 0..4 {
        output[i*8..(i+1)*8].copy_from_slice(&state[i].to_le_bytes());
    }
    output
}

// Standard block decryption
pub fn qarx256_decrypt_block(ctx: &Qarx256Ctx, input: &[u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
    let mut state = [
        u64::from_le_bytes(input[0..8].try_into().unwrap()),
        u64::from_le_bytes(input[8..16].try_into().unwrap()),
        u64::from_le_bytes(input[16..24].try_into().unwrap()),
        u64::from_le_bytes(input[24..32].try_into().unwrap()),
    ];

    for i in (0..ROUNDS).rev() {
        state = round_decrypt(state, ctx.rk[i]);
    }

    if let Some(iv) = ctx.iv {
        state[0] ^= iv[0];
        state[1] ^= iv[1];
        state[2] ^= iv[2];
        state[3] ^= iv[3];
    }

    let mut output = [0u8; BLOCK_SIZE];
    for i in 0..4 {
        output[i*8..(i+1)*8].copy_from_slice(&state[i].to_le_bytes());
    }
    output
}

pub fn round_encrypt(mut x: [u64; 4], rk: [u64; 4]) -> [u64; 4] {
    let (mut x0, mut x1, mut x2, mut x3) = (x[0], x[1], x[2], x[3]);
    x0 = x0.wrapping_add(rk[0]);
    x1 = x1.wrapping_add(rk[1]);
    x2 = x2.wrapping_add(rk[2]);
    x3 = x3.wrapping_add(rk[3]);
    x0 = x0.wrapping_add(x1);
    x1 = x1.rotate_left(ROT0) ^ x0;
    x2 = x2.wrapping_add(x3);
    x3 = x3.rotate_left(ROT1) ^ x2;
    x0 = x0.wrapping_add(x3);
    x3 = x3.rotate_left(ROT2) ^ x0;
    x2 = x2.wrapping_add(x1);
    x1 = x1.rotate_left(ROT3) ^ x2;
    let t0 = x0 ^ x2;
    let t1 = x1 ^ x3;
    x0 ^= t1;
    x3 ^= t0;
    x1 ^= x0;
    x2 ^= x3;
    x[0] ^= DIFFUSION_0;
    x[1] ^= DIFFUSION_1;
    x[2] ^= DIFFUSION_2;
    x[3] ^= DIFFUSION_3;
    x
}

pub 22
(mut x: [u64; 4], rk: [u64; 4]) -> [u64; 4] {
    x[0] ^= DIFFUSION_0;
    x[1] ^= DIFFUSION_1;
    x[2] ^= DIFFUSION_2;
    x[3] ^= DIFFUSION_3;
    x[2] ^= x[3];
    x[1] ^= x[0];
    let t1 = x[1] ^ x[3];
    let t0 = x[0] ^ x[2];
    x[3] ^= t0;
    x[0] ^= t1;
    x[1] = (x[1] ^ x[2]).rotate_right(ROT3);
    x[2] = x[2].wrapping_sub(x[1]);
    x[3] = (x[3] ^ x[0]).rotate_right(ROT2);
    x[0] = x[0].wrapping_sub(x[3]);
    x[3] = (x[3] ^ x[2]).rotate_right(ROT1);
    x[2] = x[2].wrapping_sub(x[3]);
    x[1] = (x[1] ^ x[0]).rotate_right(ROT0);
    x[0] = x[0].wrapping_sub(x[1]);
    x[0] = x[0].wrapping_sub(rk[0]);
    x[1] = x[1].wrapping_sub(rk[1]);
    x[2] = x[2].wrapping_sub(rk[2]);
    x[3] = x[3].wrapping_sub(rk[3]);
    x
}
