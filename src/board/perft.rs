use crate::Board;

pub trait Perft {
    fn perft(&self, depth : i32);
    fn perft_depth(&self, depth : i32);
    fn perft_divide(&self, depth : i32);
    fn square_to_fen(&self, square : i32);

}

impl Perft for Board {
    fn perft(&self, depth : i32) {
        todo!()
    }

    fn perft_depth(&self, depth : i32) {
        todo!()
    }

    fn perft_divide(&self, depth : i32) {
        todo!()
    }

    fn square_to_fen(&self, square : i32) {
        todo!()
    }
}