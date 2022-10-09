use crate::Board;

pub trait Initial {
    fn init(&self, fen : &String);
    fn init_tables(&self);
    fn fen_to_sq(&self, fen : &String);
    fn init_zobrist(&self);
}

impl Initial for Board {
    fn init(&self, fen : &String) {
        todo!()
    }

    fn init_tables(&self) {
        todo!()
    }

    fn fen_to_sq(&self, fen : &String) {
        todo!()
    }

    fn init_zobrist(&self) {
        todo!()
    }
}