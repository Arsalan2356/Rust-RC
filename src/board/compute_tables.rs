use crate::Board;


pub trait Computable {
    fn mask_pawn_attacks(&self, side : i32, square : i32) -> u64;
    fn mask_knight_attacks(&self, square : i32) -> u64;
    fn mask_king_attacks(&self, square : i32) -> u64;
    fn mask_bishop_attacks(&self, square : i32) -> u64;
    fn mask_rook_attacks(&self, square : i32) -> u64;
    fn compute_attack_tables(&self);
    fn compute_sliding_tables(&self);
    fn set_occupancy(&self, index : i32, bits_in_mask : i32, attack_mask : u64) -> u64;
    fn get_bishop_attacks(&self, square : i32, occupancy : u64) -> u64;
    fn get_rook_attacks(&self, square : i32, occupancy : u64) -> u64;
    fn get_queen_attacks(&self, square : i32, occupancy : u64) -> u64;
}

impl Computable for Board {
    fn mask_pawn_attacks(&self, side : i32, square : i32) -> u64 {
        todo!()
    }

    fn mask_knight_attacks(&self, square : i32) -> u64 {
        todo!()
    }

    fn mask_king_attacks(&self, square : i32) -> u64 {
        todo!()
    }

    fn mask_bishop_attacks(&self, square : i32) -> u64 {
        todo!()
    }

    fn mask_rook_attacks(&self, square : i32) -> u64 {
        todo!()
    }

    fn compute_attack_tables(&self) {
        todo!()
    }

    fn compute_sliding_tables(&self) {
        todo!()
    }

    fn set_occupancy(&self, index : i32, bits_in_mask : i32, attack_mask : u64) -> u64 {
        todo!()
    }

    fn get_bishop_attacks(&self, square : i32, occupancy : u64) -> u64 {
        todo!()
    }

    fn get_rook_attacks(&self, square : i32, occupancy : u64) -> u64 {
        todo!()
    }

    fn get_queen_attacks(&self, square : i32, occupancy : u64) -> u64 {
        todo!()
    }
}