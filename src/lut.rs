use crate::prelude::*;

pub struct LookUpTable {
    pub flips: [[[u8; 8]; 256]; 256],
    pub legal: [[[u8; 8]; 256]; 256],
}

impl LookUpTable {
    pub fn build() -> Self {
        let mut flips = [[[0; 8]; 256]; 256];
        let mut legal = [[[0; 8]; 256]; 256];

        for me in 0..256 {
            for op in 0..256 {
                if me & op != 0 {
                    continue;
                }

                for pos in 0..8 {
                    let f = Self::compute_flips(me as u8, op as u8, pos);
                    flips[me][op][pos] = f;
                    legal[me][op][pos] = if f != 0 { 1 } else { 0 };
                }
            }
        }

        Self { flips, legal }
    }

    fn compute_flips(me: u8, op: u8, pos: usize) -> u8 {
        let mut flips = 0u8;

        let mut mask = 0u8;
        let mut i = pos as i32 - 1;

        while i >= 0 {
            let bit = 1 << i;

            if me & bit != 0 {
                flips |= mask;
                break;
            }

            if op & bit == 0 {
                break;
            }

            mask |= bit;
            i -= 1;
        }

        let mut mask = 0u8;
        let mut i = pos as i32 + 1;

        while i < 8 {
            let bit = 1 << i;

            if me & bit != 0 {
                flips |= mask;
                break;
            }

            if op & bit == 0 {
                break;
            }

            mask |= bit;
            i += 1;
        }

        flips
    }
}
