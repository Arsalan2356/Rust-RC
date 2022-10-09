use crate::Board;

pub trait Magic {
    fn bishop_attacks(&self, square : i32, block : u64) -> u64;
    fn rook_attacks(&self, square : i32, block : u64) -> u64;
}

impl Magic for Board {
    fn bishop_attacks(&self, square : i32, block : u64) -> u64 {
        todo!()
    }

    fn rook_attacks(&self, square : i32, block : u64) -> u64 {
        todo!()
    }
}