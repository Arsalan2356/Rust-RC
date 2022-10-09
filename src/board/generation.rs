use crate::Board;

use super::Moves;

pub trait Generatable {
    fn generate_moves(&self, move_list : &mut Moves);
    fn add_move(&self, move_list : &mut Moves, store_move : i32);
    fn generate_captures(&self, move_list : &mut Moves);
}

impl Generatable for Board {
    fn generate_moves(&self, move_list : &mut Moves) {
        todo!()
    }

    fn add_move(&self, move_list : &mut Moves, store_move : i32) {
        todo!()
    }

    fn generate_captures(&self, move_list : &mut Moves) {
        todo!()
    }
}