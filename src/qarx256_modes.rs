// qarx256_modes.rs - Advanced modes module for research/lab use

use crate::*;

// Engine for adaptive features (research/lab only)
pub struct Qarx256AdaptiveEngine {
    ctx: Qarx256Ctx,
    entropy_threshold: f64,
}

impl Default for Qarx256AdaptiveEngine {
    fn default() -> Self {
        Qarx256AdaptiveEngine {
            ctx: Qarx256Ctx::default(),
            entropy_threshold: 0.7,
        }
    }
}

impl Qarx256AdaptiveEngine {
    pub fn new(ctx: Qarx256Ctx) -> Self {
        Qarx256AdaptiveEngine {
            ctx,
            entropy_threshold: 0.7,
        }
    }

    pub fn set_entropy_threshold(&mut self, threshold: f64) {
        self.entropy_threshold = threshold;
    }

    pub fn encrypt_with_mode(&self, input: &[u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
        let entropy = self.calculate_entropy(input);
        
        if entropy > self.entropy_threshold {
            self.encrypt_with_extra_rounds(input)
        } else {
            qarx256_encrypt_block(&self.ctx, input)
        }
    }

    fn calculate_entropy(&self, input: &[u8; BLOCK_SIZE]) -> f64 {
        let mut counts = [0u32; 256];
        for &byte in input {
            counts[byte as usize] += 1;
        }
        
        let mut entropy = 0.0;
        for &count in &counts {
            if count > 0 {
                let p = count as f64 / BLOCK_SIZE as f64;
                entropy -= p * p.log2();
            }
        }
        
        entropy / 8.0
    }

    fn encrypt_with_extra_rounds(&self, input: &[u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
        let mut state = [
            u64::from_le_bytes(input[0..8].try_into().unwrap()),
            u64::from_le_bytes(input[8..16].try_into().unwrap()),
            u64::from_le_bytes(input[16..24].try_into().unwrap()),
            u64::from_le_bytes(input[24..32].try_into().unwrap()),
        ];

        if let Some(iv) = self.ctx.iv {
            state[0] ^= iv[0];
            state[1] ^= iv[1];
            state[2] ^= iv[2];
            state[3] ^= iv[3];
        }

        for i in 0..ROUNDS * 2 {
            state = round_encrypt(state, self.ctx.rk[i % ROUNDS]);
        }

        let mut output = [0u8; BLOCK_SIZE];
        for i in 0..4 {
            output[i*8..(i+1)*8].copy_from_slice(&state[i].to_le_bytes());
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_engine_creation() {
        let ctx = Qarx256Ctx::default();
        let engine = Qarx256AdaptiveEngine::new(ctx);
        assert_eq!(engine.entropy_threshold, 0.7);
    }
}
