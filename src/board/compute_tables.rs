use crate::Board;

use super::{pop, set, Side, NOT_AB_FILE, NOT_A_FILE, NOT_HG_FILE, NOT_H_FILE, BISHOP_MAGIC_NUMBERS, BISHOP_RELEVANT_BITS, ROOK_MAGIC_NUMBERS, ROOK_RELEVANT_BITS};

pub trait Computable {
    fn mask_pawn_attacks(&self, side: u32, square: u32) -> u64;
    fn mask_knight_attacks(&self, square: u32) -> u64;
    fn mask_king_attacks(&self, square: u32) -> u64;
    fn mask_bishop_attacks(&self, square: u32) -> u64;
    fn mask_rook_attacks(&self, square: u32) -> u64;
    fn compute_attack_tables(&mut self);
    fn compute_sliding_tables(&self);
    fn set_occupancy(&self, index: i32, bits_in_mask: i32, attack_mask: u64) -> u64;
    fn get_bishop_attacks(&self, square: usize, occupancy: u64) -> u64;
    fn get_rook_attacks(&self, square: usize, occupancy: u64) -> u64;
    fn get_queen_attacks(&self, square: usize, occupancy: u64) -> u64;
}

impl Computable for Board {
    fn mask_pawn_attacks(&self, side: u32, square: u32) -> u64 {
        let mut attacks = 0u64;
        let mut bitboard = 0u64;

        set(&mut bitboard, square);

        if side == 0 {
            if bitboard >> 7 & NOT_A_FILE != 0 {
                attacks |= bitboard >> 7;
            }
            if bitboard >> 9 & NOT_H_FILE != 0 {
                attacks |= bitboard >> 9;
            }
        } else {
            if bitboard << 7 & NOT_H_FILE != 0 {
                attacks |= bitboard << 7;
            }
            if bitboard << 9 & NOT_A_FILE != 0 {
                attacks |= bitboard << 9;
            }
        }

        return attacks;
    }

    fn mask_knight_attacks(&self, square: u32) -> u64 {
        let mut attacks = 0u64;
        let mut bitboard = 0u64;

        set(&mut bitboard, square);

        if bitboard >> 17 & NOT_H_FILE != 0 {
            attacks |= bitboard >> 17;
        }
        if bitboard >> 15 & NOT_A_FILE != 0 {
            attacks |= bitboard >> 15;
        }
        if bitboard >> 10 & NOT_HG_FILE != 0 {
            attacks |= bitboard >> 10;
        }
        if bitboard >> 6 & NOT_AB_FILE != 0 {
            attacks |= bitboard >> 6;
        }
        if bitboard << 17 & NOT_A_FILE != 0 {
            attacks |= bitboard << 17;
        }
        if bitboard << 15 & NOT_H_FILE != 0 {
            attacks |= bitboard << 15;
        }
        if bitboard << 10 & NOT_AB_FILE != 0 {
            attacks |= bitboard << 10;
        }
        if bitboard << 6 & NOT_HG_FILE != 0 {
            attacks |= bitboard << 6;
        }

        return attacks;
    }

    fn mask_king_attacks(&self, square: u32) -> u64 {
        let mut attacks = 0u64;
        let mut bitboard = 0u64;

        set(&mut bitboard, square);

        if bitboard >> 8 != 0 {
            attacks |= bitboard >> 8;
        }
        if bitboard >> 9 & NOT_H_FILE != 0 {
            attacks |= bitboard >> 9;
        }
        if bitboard >> 7 & NOT_A_FILE != 0 {
            attacks |= bitboard >> 7;
        }
        if bitboard >> 1 & NOT_H_FILE != 0 {
            attacks |= bitboard >> 1;
        }
        if bitboard << 8 != 0 {
            attacks |= bitboard << 8;
        }
        if bitboard << 9 & NOT_A_FILE != 0 {
            attacks |= bitboard << 9;
        }
        if bitboard << 7 & NOT_H_FILE != 0 {
            attacks |= bitboard << 7;
        }
        if bitboard << 1 & NOT_A_FILE != 0 {
            attacks |= bitboard << 1;
        }

        return attacks;
    }

    fn mask_bishop_attacks(&self, square: u32) -> u64 {
        todo!()
    }

    fn mask_rook_attacks(&self, square: u32) -> u64 {
        todo!()
    }

    fn compute_attack_tables(&mut self) {
        for square in 0..64 as u32 {
            self.pawn_attacks[Side::WHITE as usize][square as usize] =
                self.mask_pawn_attacks(Side::WHITE as u32, square);
            self.pawn_attacks[Side::BLACK as usize][square as usize] =
                self.mask_pawn_attacks(Side::BLACK as u32, square);

            self.knight_attacks[square as usize] = self.mask_bishop_attacks(square);
            self.king_attacks[square as usize] = self.mask_king_attacks(square);
        }
    }

    fn compute_sliding_tables(&self) {
        todo!()
    }

    fn set_occupancy(&self, index: i32, bits_in_mask: i32, attack_mask: u64) -> u64 {
        let mut occupancy = 0u64;
        let mut att_mask = attack_mask;

        for count in 0..bits_in_mask {
            let square = attack_mask.trailing_zeros();
            pop(&mut att_mask, square);

            if index & (1 << count) != 0 {
                occupancy |= 1 << square;
            }
        }

        return occupancy;
    }

    fn get_bishop_attacks(&self, square: usize, occupancy: u64) -> u64 {
        let mut occ = occupancy & self.bishop_masks[square];
        occ *= BISHOP_MAGIC_NUMBERS[square];
        occ >>= 64 - BISHOP_RELEVANT_BITS[square];

        return self.bishop_attacks[square][occ as usize];
    }

    fn get_rook_attacks(&self, square: usize, occupancy: u64) -> u64 {
        let mut occ = occupancy & self.rook_masks[square];
        occ *= ROOK_MAGIC_NUMBERS[square];
        occ >>= 64 - ROOK_RELEVANT_BITS[square];

        return self.rook_attacks[square][occ as usize];
    }

    fn get_queen_attacks(&self, square: usize, occupancy: u64) -> u64 {
        return self.get_bishop_attacks(square, occupancy) | self.get_rook_attacks(square, occupancy);
    }
}
