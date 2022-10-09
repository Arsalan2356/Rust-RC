use crate::Board;

use super::Moves;

pub trait Valid {
    fn populate_move(&self, populated_move : &mut u64) -> bool;
    fn is_square_attacked(&self, square : i32, side : i32) -> i32;
    fn make_move(&self, made_move : u64, move_flag : i32) -> i32;
    fn update_log(&self, valid_move : u64);
    fn update_game_state(&self);
    fn diff_calc(&self, diff_move : u64) -> i32;
    fn check_validity(&self, valid_move : u64) -> i32;
    fn legalize_moves(&self, player_moves : Moves, pseudo_legal_moves : Moves);
    fn to_pgn(&self, pgn : &mut String, diff : i32);
}

impl Valid for Board {
    fn populate_move(&self, populated_move : &mut u64) -> bool {
        todo!()
    }

    fn is_square_attacked(&self, square : i32, side : i32) -> i32 {
        todo!()
    }

    fn make_move(&self, made_move : u64, move_flag : i32) -> i32 {
        todo!()
    }

    fn update_log(&self, valid_move : u64) {
        todo!()
    }

    fn update_game_state(&self) {
        todo!()
    }

    fn diff_calc(&self, diff_move : u64) -> i32 {
        todo!()
    }

    fn check_validity(&self, valid_move : u64) -> i32 {
        todo!()
    }

    fn legalize_moves(&self, player_moves : Moves, pseudo_legal_moves : Moves) {
        todo!()
    }

    fn to_pgn(&self, pgn : &mut String, diff : i32) {
        todo!()
    }
}