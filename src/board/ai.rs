

use std::hash;

use crate::Board;
use crate::board::*;

use super::validity::Valid;
use super::{compute_tables::Computable, Moves};
use super::flip;

pub trait AI {
    fn evaluate(&self) -> i32;
    fn search_position(&mut self, depth : i32) -> u64;
    fn negamax(&self, alpha : i32, beta : i32, depth : i32) -> i32;
    fn score_move(&mut self, score_move : u64) -> i32;
    fn quiescence(&self, alpha : i32, beta : i32) -> i32;
    fn sort_moves(&mut self, moves : Moves, best_move : u64) -> u8;
    fn enable_pv(&mut self, moves : Moves);
    fn reset_hashes(&mut self);
    fn read_hash_entry(&self, alpha : i32, beta : i32, depth : u32, best_move : &mut u64) -> i32;
    fn set_hash_entry(&mut self, value : i32, depth : u32, flags : u8, best_move : u64);
    fn is_repetition(&self) -> i32;
    fn set_file_rank_mask(&self, file : i32, rank : i32) -> u64;
    fn init_evaluation_masks(&mut self);
}



impl AI for Board {
    fn evaluate(&self) -> i32 {
        let mut mg_score = 0;

        let mut eg_score = 0;

        let mut phase : u32 = 24;

        let mut bitboard : u64;

        let mut piece : usize;

        let mut square : u32;

        for bb_piece in 0..12 {
            bitboard = self.bitboards[bb_piece];

            let mut temp_mg_score = mg_score;


            while bitboard != 0 {
                piece = bb_piece;

                let mut num_pawns : i32;

                square = bitboard.trailing_zeros();
                match piece {
                    0 => {
                        let num_pawns = (self.bitboards[0] & self.file_masks[square as usize]).count_ones();
    
                        if num_pawns > 1 {
                            mg_score += num_pawns as i32 * self.double_pawn_penalty;
                            eg_score += num_pawns as i32 * self.double_pawn_penalty;
                            
                        }

                        if self.bitboards[0] & self.isolated_pawn_masks[square as usize] == 0 {
                            mg_score += self.isolated_pawn_penalty;
                            eg_score += self.isolated_pawn_penalty;
                        }
                        else {
                            let val = (self.bitboards[0] & self.isolated_pawn_masks[square as usize]).count_ones() as i32 * (-self.isolated_pawn_penalty);
                            mg_score += val;
                            eg_score += (val as f32 * 1.1) as i32;
                        }

                        if self.white_passed_pawn_masks[square as usize] & self.bitboards[6] == 0 {
                            let sq = 7 - (square / 8);
                            let val = self.passed_pawn_bonus[sq as usize];
                            mg_score += val;
                            eg_score += (val as f32 * 1.2) as i32;
                        }
                    },
                    2 => {
                        let val = (self.get_bishop_attacks(square as i32, self.occupancies[Side::WHITE as usize])).count_ones();
                        mg_score += val as i32;
                        eg_score += (val as f32 * 1.1) as i32;
                    },
                    3 => {
                        if (self.bitboards[0] | self.bitboards[6]) & self.file_masks[square as usize] == 0 {
                            mg_score += self.open_file_score;
                            eg_score += self.open_file_score;
                        }
                        else if self.bitboards[0] & self.file_masks[square as usize] == 0{
                            mg_score += self.semi_open_file_score;
                            eg_score += self.semi_open_file_score;
                        }
                    },
                    4 => {
                        let val = self.get_queen_attacks(square as i32, self.occupancies[Side::BOTH as usize]).count_ones();
                        mg_score += val as i32;
                        eg_score += (val as f32 * 1.1) as i32;

                    },
                    5 => {
                        if (self.bitboards[0] | self.bitboards[6]) & self.file_masks[square as usize] == 0 {
                            mg_score += self.open_file_score;
                            eg_score += self.open_file_score;
                        }
                        else if self.bitboards[0] & self.file_masks[square as usize] == 0{
                            mg_score += self.semi_open_file_score;
                            eg_score += self.semi_open_file_score;
                        }
                        
                    },
                    6 => {
                        let num_pawns = (self.bitboards[6] & self.file_masks[square as usize]).count_ones();
    
                        if num_pawns > 1 {
                            mg_score += num_pawns as i32 * self.double_pawn_penalty;
                            eg_score += num_pawns as i32 * self.double_pawn_penalty;
                        }

                        if self.bitboards[6] & self.isolated_pawn_masks[square as usize] == 0 {
                            mg_score += self.isolated_pawn_penalty;
                            eg_score += self.isolated_pawn_penalty;
                        }
                        else {
                            let val = (self.bitboards[6] & self.isolated_pawn_masks[square as usize]).count_ones() as i32 * (-self.isolated_pawn_penalty);
                            mg_score += val;
                            eg_score += (val as f32 * 1.1) as i32;
                        }

                        if self.black_passed_pawn_masks[square as usize] & self.bitboards[0] == 0 {
                            let sq = 7 - (square / 8);
                            let val = self.passed_pawn_bonus[sq as usize];
                            mg_score += val;
                            eg_score += (val as f32 * 1.2) as i32;
                        }
                    },
                    8 => {
                        let val = (self.get_bishop_attacks(square as i32, self.occupancies[Side::WHITE as usize])).count_ones();
                        mg_score += val as i32;
                        eg_score += (val as f32 * 1.1) as i32;
                    },
                    9 => {
                        if (self.bitboards[0] | self.bitboards[6]) & self.file_masks[square as usize] == 0 {
                            mg_score += self.open_file_score;
                            eg_score += self.open_file_score;
                        }
                        else if self.bitboards[6] & self.file_masks[square as usize] == 0{
                            mg_score += self.semi_open_file_score;
                            eg_score += self.semi_open_file_score;
                        }
                    },
                    10 => {
                        let val = self.get_queen_attacks(square as i32, self.occupancies[Side::BOTH as usize]).count_ones();
                        mg_score += val as i32;
                        eg_score += (val as f32 * 1.1) as i32;
                    },
                    11 => {
                        if (self.bitboards[0] | self.bitboards[6]) & self.file_masks[square as usize] == 0 {
                            mg_score += self.open_file_score;
                            eg_score += self.open_file_score;
                        }
                        else if self.bitboards[6] & self.file_masks[square as usize] == 0{
                            mg_score += self.semi_open_file_score;
                            eg_score += self.semi_open_file_score;
                        }
                    },
                    _ => {

                    }
                    
                };
                mg_score += MG_VALUE[piece];
                eg_score += EG_VALUE[piece];

                if bb_piece < 6 {
                    mg_score += MG_PIECE_SCORES[piece][square as usize];
                    eg_score += EG_PIECE_SCORES[piece][square as usize];
                }
                else {
                    mg_score += MG_PIECE_SCORES[piece - 6][flip(square) as usize];
                    eg_score += EG_PIECE_SCORES[piece - 6][flip(square) as usize];
                }
                
                phase -= PHASE_INC[piece % 6];

                pop(&mut bitboard, square);
            }
        }
        mg_score += 35;

        let curr_phase = ((phase * 256 + (24 / 2)) / 24) as i32;

        let score = ((mg_score * (256 - curr_phase)) + (eg_score * curr_phase)) / 256;

        return if self.side == (Side::WHITE as i32) {score} else {-score};
    }

    fn search_position(&mut self, depth : i32) -> u64 {
        if self.is_checkmate {
            return 0;
        }

        self.killer_moves = [[0; 2]; MAX_PLY as usize];
        self.history_moves = [[0; 12]; MAX_PLY as usize];
        self.pv_table = [[0; MAX_PLY as usize]; MAX_PLY as usize];
        self.pv_length = [0; MAX_PLY as usize];

        self.nodes = 0;
        self.follow_pv = 0;
        self.score_pv = 0;

        let mut alpha = -50000;
        let mut beta = 50000;

        let mut score = 0;

        for current_depth in 1..depth {
            self.follow_pv = 1;
            score = self.negamax(alpha, beta, current_depth);

            if score <= alpha || score >= beta {
                alpha = -50000;
                beta = 50000;
            }
            else {
                alpha = score - 50;
                beta = score + 50;
            }

        }

        let move_x = self.pv_table[0][0];
        let mut pgn : String = "".to_string();

        self.to_pgn(&mut pgn, self.diff_calc(move_x));

        if score > -49000 && score < -48000 {
            println!("Best Move : {}", pgn);
            println!("Eval : Mate in {}", (score + 49000) / 2 - 1);
            println!("Nodes : {}", self.nodes);
            println!("Depth : {}", depth);
        }
        else if score > 48000 && score < 49000 {
            println!("Best Move : {}", pgn);
            println!("Eval : Mate in {}", (49000 - score) / 2 + 1);
            println!("Nodes : {}", self.nodes);
            println!("Depth : {}", depth);
        }
        else {
            println!("Best Move : {}", pgn);
            println!("Eval : {}", score);
            println!("Nodes : {}", self.nodes);
            println!("Depth : {}", depth);
        }

        return self.pv_table[0][0];
    }

    fn negamax(&self, alpha : i32, beta : i32, depth : i32) -> i32 {
        todo!()
    }

    fn score_move(&mut self, score_move : u64) -> i32 {
        if self.score_pv == 1 {
            if self.pv_table[0][self.ply as usize] == score_move {
                self.score_pv = 0;
                return 900;
            }
        }

        if get_move_capture(score_move) != 0 {
            let target_piece = get_move_capture_piece(score_move);
            return MVV_LVA[get_move_piece(score_move) as usize][target_piece as usize];
        }
        else {
            if self.killer_moves[0][self.ply as usize] == score_move {
                return 90;
            }
            else if self.killer_moves[1][self.ply as usize] == score_move {
                return 80;
            }
            else {
                return self.history_moves[get_move_piece(score_move) as usize][get_move_target(score_move) as usize];
            }
        }
    }

    fn quiescence(&self, alpha : i32, beta : i32) -> i32 {
        todo!()
    }

    fn sort_moves(&mut self, mut moves : Moves, best_move : u64) -> u8 {
        let mut move_scores: Vec<i32> = Vec::with_capacity(moves.count as usize);

        for count in 0..moves.count as usize {
            if best_move == moves.moves[count] {
                move_scores[count] = 1200;
            }
            else {
                move_scores[count] = self.score_move(moves.moves[count]);
            }
        }

        let mut i: i32 = 1;
        while i < moves.count {
            let mut score = move_scores[i as usize];
            let mut curr_move : u64 = moves.moves[i as usize];
            let mut j = i - 1;
            while j >= 0 && move_scores[j as usize] < score {
                move_scores[(j + 1) as usize] = move_scores[j as usize];
                moves.moves[(j + 1) as usize] = moves.moves[j as usize];
                j -= 1;
            }
            move_scores[(j + 1) as usize] = score;
            moves.moves[(j + 1) as usize] = curr_move;
            i += 1;
        }
        return 1;
    }

    fn enable_pv(&mut self, moves : Moves) {
        self.follow_pv = 0;
        for count in 0..moves.count {
            if self.pv_table[0][self.ply as usize] == moves.moves[count as usize] {
                self.score_pv = 1;
                self.follow_pv = 1;
                break;
            }
        }
    }

    fn reset_hashes(&mut self) {
        for i in 0..HASH_SIZE {
            self.transposition_table[i as usize] = Hash::default();
        }
    }

    fn read_hash_entry(&self, alpha : i32, beta : i32, depth : u32, best_move : &mut u64) -> i32 {
        let hash_entry = &self.transposition_table[(self.curr_zobrist_hash % HASH_SIZE as u64) as usize];
        
        if hash_entry.key == self.curr_zobrist_hash {
            if hash_entry.depth >= depth {
                let mut val = hash_entry.value;
                if val < -48000 {
                    val += self.ply as i32;
                }
                else if val > 48000 {
                    val -= self.ply as i32;
                }
                
                if hash_entry.flags == HASH_FLAG_EXACT {
                    return val;
                }
                else if hash_entry.flags == HASH_FLAG_ALPHA && val <= alpha {
                    return alpha;
                }
                else if hash_entry.flags == HASH_FLAG_BETA && val >= beta {
                    return beta;
                }
            }

            *best_move = hash_entry.best_move;
        }

        return 100000;
    }

    fn set_hash_entry(&mut self, value : i32, depth : u32, flags : u8, best_move : u64) {
        let hash_entry = &mut self.transposition_table[(self.curr_zobrist_hash % HASH_SIZE as u64) as usize];
        let mut val = value;


        if val < -48000 {
            val -= self.ply as i32;
        }
        else if val > 48000 {
            val += self.ply as i32;
        }

        hash_entry.key = self.curr_zobrist_hash;
        hash_entry.depth = depth;
        hash_entry.flags = flags;
        hash_entry.value = val;
        hash_entry.best_move = best_move;
    }

    fn is_repetition(&self) -> i32 {
        for i in 0..(self.repetition_index as usize) {
            if self.repetition_table[i] == self.curr_zobrist_hash {
                return 1;
            }
        }
        return 0;
    }

    fn set_file_rank_mask(&self, file : i32, rank : i32) -> u64 {
        let mut mask: u64 = 0;
        for r in 0..8 {
            for f in 0..8 {
                let sq = r * 8 + f;
                if f == file {
                    mask |= 1 << sq;
                }
                else if r == rank {
                    mask |= 1 << sq;
                }
            }
        }

        return mask;
    }

    fn init_evaluation_masks(&mut self) {
        for r in 0..8 as i32 {
            for f in 0..8 as i32 {
                let sq = (r * 8 + f) as usize;
                self.file_masks[sq] |= self.set_file_rank_mask(f, -1);
                self.rank_masks[sq] |= self.set_file_rank_mask(-1, r);

                self.isolated_pawn_masks[sq] |= self.set_file_rank_mask(f + 1, -1);
                self.isolated_pawn_masks[sq] |= self.set_file_rank_mask(f - 1, -1);

                self.white_passed_pawn_masks[sq] |= self.set_file_rank_mask(f - 1, -1);
                self.white_passed_pawn_masks[sq] |= self.set_file_rank_mask(f, -1);
                self.white_passed_pawn_masks[sq] |= self.set_file_rank_mask(f + 1, -1);
            
                for i in 0..(8 - r) {
                    self.white_passed_pawn_masks[sq] &= !(self.rank_masks[((7 - i) * 8 + f) as usize]);
                }

                self.black_passed_pawn_masks[sq] |= self.set_file_rank_mask(f - 1, -1);
                self.black_passed_pawn_masks[sq] |= self.set_file_rank_mask(f, -1);
                self.black_passed_pawn_masks[sq] |= self.set_file_rank_mask(f + 1, -1);
            
                for j in 0..(r + 1) {
                    self.black_passed_pawn_masks[sq] &= !(self.rank_masks[(j * 8 + f) as usize]);
                }
            
            }
        }
    }
}