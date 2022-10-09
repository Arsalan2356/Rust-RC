use crate::{Board, board::{compute_tables::Computable, ai::AI}};




pub trait Initial {
    fn init(&self, fen : &String);
    fn init_tables(&mut self);
    fn fen_to_sq(&self, fen : &String) -> u32;
    fn init_zobrist(&mut self);
}

impl Initial for Board {
    fn init(&self, fen : &String) {
        todo!()
    }

    fn init_tables(&mut self) {
        println!("Computing attack tables for non-sliding pieces");
        self.compute_attack_tables();
        print!("Done");
        println!("Computing attack tables for sliding pieces");
        self.compute_sliding_tables();
        println!("Done");
        println!("Generating Zobrist Keys");
        self.init_zobrist();
        println!("Done");
        println!("Generating Evaluation Masks");
        self.init_evaluation_masks();
        println!("Done");
    }

    fn fen_to_sq(&self, fen : &String) -> u32 {
        let rank = 8 - (fen.chars().nth(1).unwrap() as u32 - '0' as u32);
        let file = fen.chars().nth(0).unwrap() as u32 - 'a' as u32;

        return rank * 8 + file;

    }

    fn init_zobrist(&mut self) {
        for piece in 0..=11 {
            for square in 0..64 {
                self.zobrist_keys[piece][square] = rand::random();
                if piece == 11 {
                    self.en_passant_zobrist[square] = rand::random();
                }
            }
        }

        for i in 0..16 {
            self.castle_zobrist[i] = rand::random();
        }

        self.zobrist_side_key = rand::random();
    }
}