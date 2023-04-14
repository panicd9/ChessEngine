// Board representation

//      A    B    C    D    E    F    G    H
//    +----+----+----+----+----+----+----+----+
//  8 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 |  8th rank
//    +----+----+----+----+----+----+----+----+
//  7 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 |  7th rank
//    +----+----+----+----+----+----+----+----+
//  6 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 |  6th rank
//    +----+----+----+----+----+----+----+----+
//  5 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 |  5th rank
//    +----+----+----+----+----+----+----+----+
//  4 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 |  4th rank
//    +----+----+----+----+----+----+----+----+
//  3 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 |  3rd rank
//    +----+----+----+----+----+----+----+----+
//  2 |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 |  2nd rank
//    +----+----+----+----+----+----+----+----+
//  1 |  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  1st rank
//    +----+----+----+----+----+----+----+----+
//       A    B    C    D    E    F    G    H - file(s)

#![allow(non_camel_case_types, dead_code)]

pub mod chessboard {
    use std::collections::HashMap;
    use std::num;
    use std::result;
    use std::vec;

    use crate::white_utils::*;
    use crate::black_utils::*;

    pub(crate) trait OverflowingLeftShift {
        fn overflowing_loss_checked_shl(self, rhs: u32) -> (Self, bool) 
            where Self: std::marker::Sized;
    }

    impl OverflowingLeftShift for u64 {
        fn overflowing_loss_checked_shl(self, rhs: u32) -> (Self, bool) {
            if rhs <= self.leading_zeros() {
                return (self << rhs, false);
            } else {
                return (0, true);
            }
        }
    }
    enum EnumSquare {
        a1, b1, c1, d1, e1, f1, g1, h1, //  0 ..  7
        a2, b2, c2, d2, e2, f2, g2, h2, //  8 .. 15
        a3, b3, c3, d3, e3, f3, g3, h3, // 16 .. 23
        a4, b4, c4, d4, e4, f4, g4, h4, // 24 .. 31
        a5, b5, c5, d5, e5, f5, g5, h5, // 32 .. 39
        a6, b6, c6, d6, e6, f6, g6, h6, // 40 .. 47
        a7, b7, c7, d7, e7, f7, g7, h7, // 48 .. 55
        a8, b8, c8, d8, e8, f8, g8, h8  // 56 .. 63
    }
      
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct ChessBoard {

        pub white_to_move: bool,
        pub prev_pos_pawns: u64,

        pub white_moved_king: bool,
        pub white_moved_A_rook: bool,
        pub white_moved_H_rook: bool,
        pub white_pawns: u64,
        pub white_rooks: u64,
        pub white_knights: u64,
        pub white_bishops: u64,
        pub white_queens: u64,
        pub white_king: u64,
    
        pub black_moved_king: bool,
        pub black_moved_A_rook: bool,
        pub black_moved_H_rook: bool,
        pub black_pawns: u64,
        pub black_rooks: u64,
        pub black_knights: u64,
        pub black_bishops: u64,
        pub black_queens: u64,
        pub black_king: u64,
    }
    
    impl ChessBoard {
        pub fn new() -> Self {
            Self { 
                white_to_move: true,
                prev_pos_pawns: 0,

                white_moved_king: false,
                white_moved_A_rook: false,
                white_moved_H_rook: false,
                white_pawns: 0xFF00,
                white_rooks: 0x81,
                white_knights: 0x42,
                white_bishops: 0x24,
                white_queens: 0x8,
                white_king: 0x10,
            
                black_moved_king: false,
                black_moved_A_rook: false,
                black_moved_H_rook: false,
                black_pawns: 0xFF000000000000,
                black_rooks: 0x8100000000000000,
                black_knights: 0x4200000000000000,
                black_bishops: 0x2400000000000000,
                black_queens: 0x800000000000000,
                black_king: 0x1000000000000000,
            }
        }

        pub fn get_all_white_pieces (&self) -> u64 {
            self.white_pawns | self.white_rooks | self.white_knights | self.white_bishops | self.white_queens | self.white_king
        }

        pub fn get_all_black_pieces (&self) -> u64 {
            self.black_pawns | self.black_rooks | self.black_knights | self.black_bishops | self.black_queens | self.black_king
        }

        pub fn remove_white_piece(&mut self, square: u64) {
            // if self.get_all_white_pieces() & square == 0 { println!("GRESKA 1 !!!"); return; }; // TODO: remove

            if self.white_pawns & square > 0 { self.white_pawns -= square; return; }
            if self.white_rooks & square > 0 { self.white_rooks -= square; return; }
            if self.white_knights & square > 0 { self.white_knights -= square; return; }
            if self.white_bishops & square > 0 { self.white_bishops -= square; return; }
            if self.white_queens & square > 0 { self.white_queens -= square; return; }
            if self.white_king & square > 0 { self.white_king -= square; return; }
        }

        pub fn remove_black_piece(&mut self, square: u64) {
            // if self.get_all_black_pieces() & square == 0 { println!("GRESKA 2 !!!"); return; }; // TODO: remove

            if self.black_pawns & square > 0 { self.black_pawns -= square; return; }
            if self.black_rooks & square > 0 { self.black_rooks -= square; return; }
            if self.black_knights & square > 0 { self.black_knights -= square; return; }
            if self.black_bishops & square > 0 { self.black_bishops -= square; return; }
            if self.black_queens & square > 0 { self.black_queens -= square; return; }
            if self.black_king & square > 0 { self.black_king -= square; return; }
        }

        pub fn get_all_pieces(&self) -> u64 {
            self.get_all_white_pieces() | self.get_all_black_pieces()
        }

        pub fn print_chessboard(&self) {
            println!("---------------\n");
            let mut square;
            for i in 0..8 {
                for j in 56 - i * 8..64 - i * 8 {
                    square = 2_u64.pow(j);
                    if self.black_pawns & square > 0 { print!("\u{265F} "); continue; }
                    if self.black_rooks & square > 0 { print!("\u{265C} "); continue; }
                    if self.black_knights & square > 0 { print!("\u{265E} "); continue; }
                    if self.black_bishops & square > 0 { print!("\u{265D} "); continue; }
                    if self.black_queens & square > 0 { print!("\u{265B} "); continue; }
                    if self.black_king & square > 0 { print!("\u{265A} " ); continue; }

                    if self.white_pawns & square > 0 { print!("\u{2659} "); continue; }
                    if self.white_rooks & square > 0 { print!("\u{2656} "); continue; }
                    if self.white_knights & square > 0 { print!("\u{2658} "); continue; }
                    if self.white_bishops & square > 0 { print!("\u{2657} "); continue; }
                    if self.white_queens & square > 0 { print!("\u{2655} "); continue; }
                    if self.white_king & square > 0 { print!("\u{2654} "); continue; }
                    print!("  ");
                }
                print!("\n");
            }
            print!("\n");
        }

        pub fn is_white_king_checked(&self) -> (bool, Vec<ChessBoard>) {
            let mut next_pseudo_legal_black_moves = self.get_all_pseudo_legal_black_moves();
            let starting_len = next_pseudo_legal_black_moves.len();

            next_pseudo_legal_black_moves.retain(|&e| e.white_king > 0);
            let curr_len = next_pseudo_legal_black_moves.len();

            let is_king_checked = starting_len != curr_len;

            return (is_king_checked, next_pseudo_legal_black_moves);
        }

        pub fn is_black_king_checked(&self) -> (bool, Vec<ChessBoard>) {
            let mut next_pseudo_legal_white_moves = self.get_all_pseudo_legal_white_moves();
            let starting_len = next_pseudo_legal_white_moves.len();

            next_pseudo_legal_white_moves.retain(|&e| e.black_king > 0);
            let curr_len = next_pseudo_legal_white_moves.len();

            let is_king_checked = starting_len != curr_len;

            return (is_king_checked, next_pseudo_legal_white_moves);
        }
        
        pub fn get_all_pseudo_legal_white_pawn_moves(&self) -> Vec<ChessBoard> {
            let mut result: Vec<ChessBoard> = vec![];
            let white_pawns = self.white_pawns;
            let mut square: u64;
            // 64 - 8 = 56 // cant occupy 1st rank
            for i in 8..56 {
                square = 1 << i;
                // square =  2_u64.pow(i);
                // check if pawn occupies square
                if white_pawns & square > 0 {

                    // CHECK FOR UPRIGHT TAKE ( attacked_square = square << 9 )
                    // let attacked_square = square * 2_u64.pow(9);
                    let (attacked_square, overflow) = square.overflowing_loss_checked_shl(9);
                    // cant be on A file after taking upright
                    if !overflow && attacked_square & Constants::A_FILE == 0 {
                        //check if there is enemy piece and take
                        check_white_pawn_take(square, attacked_square, self, &mut result);
                    }

                    // CHECK FOR UPLEFT TAKE ( attacked_square = square << 7 )
                    let attacked_square = square << 7;
                    // cant be on H file after taking upleft
                    if attacked_square & Constants::H_FILE == 0 {
                        //check if there is enemy piece and take
                        check_white_pawn_take(square, attacked_square, self, &mut result);
                    }

                    // CHECK FOR 1 SQUARE FORWARD
                    // let forward_square_1 = square * 2_u64.pow(8);
                    let forward_square_1 = square << 8;

                    if (forward_square_1 & self.get_all_pieces()) == 0 {
                        white_pawn_forward(square, forward_square_1, self, &mut result);
                    }

                    // CHECK FOR 2 SQUARES FORWARD
                    let forward_square_2 = square << 16;

                    if square & Constants::SECOND_RANK > 0 && forward_square_1 & self.get_all_pieces() == 0 && forward_square_2 & self.get_all_pieces() == 0 {
                        white_pawn_forward(square, forward_square_2, self, &mut result)
                    }

                    // CHECK FOR EN PASSANT
                    if square & Constants::FIFTH_RANK > 0 {
                        // CHECK LEFT EN PASSANT
                        if (square >> 1) & self.black_pawns > 0 && self.prev_pos_pawns & ( square >> 1 << 16 ) > 0 {
                            white_en_passant_move(square, square >> 1, self, &mut result)
                        }


                        // CHECK RIGHT EN PASSANT
                        if (square << 1) & self.black_pawns > 0 && self.prev_pos_pawns & ( square << 1 << 16 ) > 0 {
                            white_en_passant_move(square, square << 1, self, &mut result)
                        }
                    }
                }
            };

            return result;
        }

        pub fn get_all_pseudo_legal_black_pawn_moves(&self) -> Vec<ChessBoard> {
            let mut result: Vec<ChessBoard> = vec![];
            let black_pawns = self.black_pawns;
            let mut square: u64;
            // 64 - 8 = 56 // cant occupy 1st rank
            for i in 0..56 {
                square =  1 << i;
                // check if pawn occupies square
                if black_pawns & square > 0 {

                    // CHECK FOR DOWNRIGHT TAKE
                    let attacked_square = square >> 7;
                    // cant be on A file after taking DOWNRIGHT
                    if attacked_square & Constants::A_FILE == 0 {
                        //check if there is enemy piece
                        if attacked_square & self.get_all_white_pieces() > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_pawns -= square;
                            new_chessboard.black_pawns += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        }
                    }

                    // CHECK FOR DOWNLEFT TAKE
                    let attacked_square = square >> 9;
                    // cant be on H file after taking DOWNLEFT
                    if attacked_square > 0 && attacked_square & Constants::H_FILE == 0 {
                        //check if there is enemy piece
                        if attacked_square & self.get_all_white_pieces() > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_pawns -= square;
                            new_chessboard.black_pawns += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        }
                    }

                    // CHECK FOR 1 SQUARE FORWARD
                    let forward_square_1 = square >> 8;

                    if (forward_square_1 & self.get_all_pieces()) == 0 {
                        // PUSH PAWN IF NOT ON EIGHT RANK
                        if (forward_square_1 & Constants::FIRST_RANK) == 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_pawns -= square;
                            new_chessboard.black_pawns += forward_square_1;
                            result.push(new_chessboard);
                        }
                        // TODO: PROMOTE TO QUEEN 
                        else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_pawns -= square;
                            new_chessboard.black_queens += forward_square_1;
                            result.push(new_chessboard);
                        }



                    }

                    // CHECK FOR 2 SQUARES FORWARD
                    let forward_square_2 = square >> 16;

                    if square & Constants::SEVENTH_RANK > 0 && forward_square_1 & self.get_all_pieces() == 0 && forward_square_2 & self.get_all_pieces() == 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                        
                        new_chessboard.black_pawns -= square;
                        new_chessboard.black_pawns += forward_square_2;
                        result.push(new_chessboard);
                    }

                    // CHECK FOR EN PASSANT
                    if square & Constants::FOURTH_RANK > 0 {
                        // CHECK LEFT EN PASSANT
                        if (square >> 1) & self.white_pawns > 0 && self.prev_pos_pawns & ( square >> 1 >> 16 ) > 0 {
                            black_en_passant_move(square, square >> 1, self, &mut result)
                        }


                        // CHECK RIGHT EN PASSANT
                        if (square << 1) & self.white_pawns > 0 && self.prev_pos_pawns & ( square << 1 >> 16 ) > 0 {
                            black_en_passant_move(square, square << 1, self, &mut result)
                        }
                    }
                }
            };

            return result;
        }
    
        pub fn get_all_pseudo_legal_white_rook_moves(&self) -> Vec<ChessBoard> {
            
            let mut result: Vec<ChessBoard> = vec![];
            let white_rooks = self.white_rooks;
            let mut square: u64;
            for i in 0..64 {
                square =  1 << i;
                // check if rook occupies square
                if white_rooks & square > 0 {

                    // CHECK UP
                    let (mut attacked_square, mut overflow) = square.overflowing_loss_checked_shl(8);
                    while !overflow && (self.get_all_pieces() & attacked_square) == 0 {
                        
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                    
                        new_chessboard.white_rooks -= square;
                        new_chessboard.white_rooks += attacked_square;
                        result.push(new_chessboard);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(8);
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_black_pieces() & attacked_square > 0 && !overflow{
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                    
                        new_chessboard.white_rooks -= square;
                        new_chessboard.white_rooks += attacked_square;
                        new_chessboard.remove_black_piece(attacked_square);
                        result.push(new_chessboard);
                    }

                    // CHECK DOWN
                    attacked_square = square >> 8;
                    while attacked_square > 0 && (self.get_all_pieces() & attacked_square) == 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                    
                        new_chessboard.white_rooks -= square;
                        new_chessboard.white_rooks += attacked_square;
                        result.push(new_chessboard);
                        attacked_square = attacked_square >> 8;
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_black_pieces() & attacked_square > 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                    
                        new_chessboard.white_rooks -= square;
                        new_chessboard.white_rooks += attacked_square;
                        new_chessboard.remove_black_piece(attacked_square);
                        result.push(new_chessboard);
                    }

                    // CHECK LEFT
                    attacked_square = square >> 1;
                    while (attacked_square & Constants::H_FILE) == 0 && (self.get_all_pieces() & attacked_square) == 0 && attacked_square > 0 {
                        
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                    
                        new_chessboard.white_rooks -= square;
                        new_chessboard.white_rooks += attacked_square;
                        result.push(new_chessboard);
                        attacked_square = attacked_square >> 1;
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_black_pieces() & attacked_square > 0 && ( attacked_square > 0 && (attacked_square & Constants::H_FILE) == 0 ){
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                    
                        new_chessboard.white_rooks -= square;
                        new_chessboard.white_rooks += attacked_square;
                        new_chessboard.remove_black_piece(attacked_square);
                        result.push(new_chessboard);
                    }

                    // CHECK RIGHT
                    (attacked_square, overflow) = square.overflowing_loss_checked_shl(1);
                    while (attacked_square & Constants::A_FILE) == 0 && (self.get_all_pieces() & attacked_square) == 0 && !overflow {
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                    
                        new_chessboard.white_rooks -= square;
                        new_chessboard.white_rooks += attacked_square;
                        result.push(new_chessboard);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(1);
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_black_pieces() & attacked_square > 0 && ( (attacked_square & Constants::A_FILE) == 0 && !overflow ){
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                    
                        new_chessboard.white_rooks -= square;
                        new_chessboard.white_rooks += attacked_square;
                        new_chessboard.remove_black_piece(attacked_square);
                        result.push(new_chessboard);
                    }
                }
            }
            return result;
        }
    
        pub fn get_all_pseudo_legal_black_rook_moves(&self) -> Vec<ChessBoard> {
            let mut result: Vec<ChessBoard> = vec![];
            let black_rooks = self.black_rooks;
            let mut square: u64;
            for i in 0..64 {
                square =  1 << i;
                // check if rook occupies square
                if black_rooks & square > 0 {

                    // CHECK UP
                    let (mut attacked_square, mut overflow) = square.overflowing_loss_checked_shl(8);
                    while !overflow && (self.get_all_pieces() & attacked_square) == 0{
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                    
                        new_chessboard.black_rooks -= square;
                        new_chessboard.black_rooks += attacked_square;
                        result.push(new_chessboard);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(8);
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_white_pieces() & attacked_square > 0 && !overflow{
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                    
                        new_chessboard.black_rooks -= square;
                        new_chessboard.black_rooks += attacked_square;
                        new_chessboard.remove_white_piece(attacked_square);
                        result.push(new_chessboard);
                    }

                    // CHECK DOWN
                    attacked_square = square >> 8;
                    while attacked_square > 0 && (self.get_all_pieces() & attacked_square) == 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                    
                        new_chessboard.black_rooks -= square;
                        new_chessboard.black_rooks += attacked_square;
                        result.push(new_chessboard);
                        attacked_square = attacked_square >> 8;
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_white_pieces() & attacked_square > 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                    
                        new_chessboard.black_rooks -= square;
                        new_chessboard.black_rooks += attacked_square;
                        new_chessboard.remove_white_piece(attacked_square);
                        result.push(new_chessboard);
                    }

                    // CHECK LEFT
                    attacked_square = square >> 1;
                    while (attacked_square & Constants::H_FILE) == 0 && (self.get_all_pieces() & attacked_square) == 0 && attacked_square > 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                    
                        new_chessboard.black_rooks -= square;
                        new_chessboard.black_rooks += attacked_square;
                        result.push(new_chessboard);
                        attacked_square = attacked_square >> 1;
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_white_pieces() & attacked_square > 0 && ( (attacked_square & Constants::H_FILE) == 0 && attacked_square > 0 ) {
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                    
                        new_chessboard.black_rooks -= square;
                        new_chessboard.black_rooks += attacked_square;
                        new_chessboard.remove_white_piece(attacked_square);
                        result.push(new_chessboard);
                    }

                    // CHECK RIGHT
                    (attacked_square, overflow) = square.overflowing_loss_checked_shl(1);
                    while (attacked_square & Constants::A_FILE) == 0 && (self.get_all_pieces() & attacked_square) == 0 && !overflow {
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                    
                        new_chessboard.black_rooks -= square;
                        new_chessboard.black_rooks += attacked_square;
                        result.push(new_chessboard);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(1);
                        
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_white_pieces() & attacked_square > 0 && ( (attacked_square & Constants::A_FILE) == 0 && !overflow ){
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_to_move = !new_chessboard.white_to_move;
                        new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                    
                        new_chessboard.black_rooks -= square;
                        new_chessboard.black_rooks += attacked_square;
                        new_chessboard.remove_white_piece(attacked_square);
                        result.push(new_chessboard);
                    }
                }
            }
            return result;
        }
    
        pub fn get_all_pseudo_legal_white_knight_moves(&self) -> Vec<ChessBoard> {
            let mut result: Vec<ChessBoard> = vec![];
            let white_knights = self.white_knights;
            let mut square: u64;
            for i in 0..64 {
                square =  1 << i;
                // check if knight occupies square
                if white_knights & square > 0 {
                    let (mut attacked_square, mut overflow ) = square.overflowing_loss_checked_shl(17);

                    // UP-RIGHT
                    if !overflow && (attacked_square & Constants::A_FILE) == 0 && (attacked_square & self.get_all_white_pieces()) == 0 {
                        if (self.get_all_black_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // UP-LEFT
                    (attacked_square, overflow)  = square.overflowing_loss_checked_shl(15);

                    if !overflow && (attacked_square & Constants::H_FILE) == 0 && (attacked_square & self.get_all_white_pieces()) == 0 {
                        
                        if (self.get_all_black_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // DOWN-RIGHT
                    attacked_square  = square >> 15;
                    if (attacked_square > 0) && (attacked_square & Constants::A_FILE) == 0 && (attacked_square & self.get_all_white_pieces()) == 0 {
                        if (self.get_all_black_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // DOWN-LEFT
                    attacked_square  = square >> 17;
                    if (attacked_square > 0) && (attacked_square & Constants::H_FILE) == 0 && (attacked_square & self.get_all_white_pieces()) == 0 {
                        if (self.get_all_black_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // RIGHT-UP --^
                    (attacked_square, overflow)  = square.overflowing_loss_checked_shl(10);
                    if !overflow && (attacked_square & Constants::A_FILE) == 0 && (attacked_square & Constants::B_FILE) == 0 && (attacked_square & self.get_all_white_pieces()) == 0 {
                        if (self.get_all_black_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // RIGHT-DOWN
                    attacked_square  = square >> 6;
                    if attacked_square > 0 && (attacked_square & Constants::A_FILE) == 0 && (attacked_square & Constants::B_FILE) == 0 && (attacked_square & self.get_all_white_pieces()) == 0 {
                        if (self.get_all_black_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // LEFT-UP ^--
                    (attacked_square, overflow)  = square.overflowing_loss_checked_shl(6);
                    if !overflow && (attacked_square & Constants::G_FILE) == 0 && (attacked_square & Constants::H_FILE) == 0 && (attacked_square & self.get_all_white_pieces()) == 0  {
                        if (self.get_all_black_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // LEFT-DOWN
                    attacked_square  = square >> 10;
                    if attacked_square > 0 && (attacked_square & Constants::G_FILE) == 0 && (attacked_square & Constants::H_FILE) == 0 && (attacked_square & self.get_all_white_pieces()) == 0  {
                        if (self.get_all_black_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
                            
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                }
            }
            return result;
        }
    
        pub fn get_all_pseudo_legal_black_knight_moves(&self) -> Vec<ChessBoard> {
            let mut result: Vec<ChessBoard> = vec![];
            let black_knights = self.black_knights;
            let mut square: u64;
            for i in 0..64 {
                square =  1 << i;
                // check if knight occupies square
                if black_knights & square > 0 {
                    let (mut attacked_square, mut overflow ) = square.overflowing_loss_checked_shl(17);

                    // UP-RIGHT
                    if !overflow && (attacked_square & Constants::A_FILE) == 0 && (attacked_square & self.get_all_black_pieces()) == 0 {
                        if (self.get_all_white_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // UP-LEFT
                    (attacked_square, overflow)  = square.overflowing_loss_checked_shl(15);

                    if !overflow && (attacked_square & Constants::H_FILE) == 0 && (attacked_square & self.get_all_black_pieces()) == 0 {
                        
                        if (self.get_all_white_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // DOWN-RIGHT
                    attacked_square  = square >> 15;
                    if (attacked_square > 0) && (attacked_square & Constants::A_FILE) == 0 && (attacked_square & self.get_all_black_pieces()) == 0 {
                        if (self.get_all_white_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // DOWN-LEFT
                    attacked_square  = square >> 17;
                    if (attacked_square > 0) && (attacked_square & Constants::H_FILE) == 0 && (attacked_square & self.get_all_black_pieces()) == 0 {
                        if (self.get_all_white_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // RIGHT-UP --^
                    (attacked_square, overflow)  = square.overflowing_loss_checked_shl(10);
                    if !overflow && (attacked_square & Constants::A_FILE) == 0 && (attacked_square & Constants::B_FILE) == 0 && (attacked_square & self.get_all_black_pieces()) == 0 {
                        if (self.get_all_white_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // RIGHT-DOWN
                    attacked_square  = square >> 6;
                    if attacked_square > 0 && (attacked_square & Constants::A_FILE) == 0 && (attacked_square & Constants::B_FILE) == 0 && (attacked_square & self.get_all_black_pieces()) == 0 {
                        if (self.get_all_white_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // LEFT-UP ^--
                    (attacked_square, overflow)  = square.overflowing_loss_checked_shl(6);
                    if !overflow && (attacked_square & Constants::G_FILE) == 0 && (attacked_square & Constants::H_FILE) == 0 && (attacked_square & self.get_all_black_pieces()) == 0  {
                        if (self.get_all_white_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // LEFT-DOWN
                    attacked_square  = square >> 10;
                    if attacked_square > 0 && (attacked_square & Constants::G_FILE) == 0 && (attacked_square & Constants::H_FILE) == 0 && (attacked_square & self.get_all_black_pieces()) == 0  {
                        if (self.get_all_white_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_to_move = !new_chessboard.white_to_move;
                            new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
                            
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                }
            }
            return result;
        }

        pub fn get_all_pseudo_legal_white_bishop_moves(&self) -> Vec<ChessBoard> {
            let mut result: Vec<ChessBoard> = vec![];
            let white_bishops = self.white_bishops;
            let mut square: u64;
            let mut hit_piece: bool;
            for i in 0..64 {
                square =  1 << i;

                if white_bishops & square > 0 {
                    // CHECK UPRIGHT
                    hit_piece = false;
                    let (mut attacked_square, mut overflow) = square.overflowing_loss_checked_shl(9);
                    while !overflow && !hit_piece && attacked_square & Constants::A_FILE == 0  {
                        hit_piece = white_bishop_move(square, attacked_square, self, &mut result);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(9);
                    }
                    // CHECK UPLEFT
                    hit_piece = false;
                    let (mut attacked_square, mut overflow) = square.overflowing_loss_checked_shl(7);
                    while !overflow && !hit_piece && attacked_square & Constants::H_FILE == 0 {
                        hit_piece = white_bishop_move(square, attacked_square, self, &mut result);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(7);
                    }
                    // CHECK DOWNRIGHT
                    hit_piece = false;
                    let mut attacked_square = square >> 7;
                    while (attacked_square > 0) && !hit_piece && attacked_square & Constants::A_FILE == 0 {
                        hit_piece = white_bishop_move(square, attacked_square, self, &mut result);
                        attacked_square = attacked_square >> 7;
                    }

                    // CHECK DOWNLEFT
                    hit_piece = false;
                    let mut attacked_square = square >> 9 ;
                    while (attacked_square > 0) && !hit_piece && attacked_square & Constants::H_FILE == 0 {
                        hit_piece = white_bishop_move(square, attacked_square, self, &mut result);
                        attacked_square = attacked_square >> 9;
                    }
                }
            }
            return result;
        }
        
        pub fn get_all_pseudo_legal_black_bishop_moves(&self) -> Vec<ChessBoard> {
            let mut result: Vec<ChessBoard> = vec![];
            let black_bishops = self.black_bishops;
            let mut square: u64;
            let mut hit_piece: bool;
            for i in 0..64 {
                square =  1 << i;

                if black_bishops & square > 0 {
                    // CHECK UPRIGHT
                    hit_piece = false;
                    let (mut attacked_square, mut overflow) = square.overflowing_loss_checked_shl(9);
                    while !overflow && !hit_piece && attacked_square & Constants::A_FILE == 0  {
                        hit_piece = black_bishop_move(square, attacked_square, self, &mut result);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(9);
                    }
                    // CHECK UPLEFT
                    hit_piece = false;
                    let (mut attacked_square, mut overflow) = square.overflowing_loss_checked_shl(7);
                    while !overflow && !hit_piece && attacked_square & Constants::H_FILE == 0 {
                        hit_piece = black_bishop_move(square, attacked_square, self, &mut result);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(7);
                    }
                    // CHECK DOWNRIGHT
                    hit_piece = false;
                    let mut attacked_square = square >> 7;
                    while (attacked_square > 0) && !hit_piece && attacked_square & Constants::A_FILE == 0 {
                        hit_piece = black_bishop_move(square, attacked_square, self, &mut result);
                        attacked_square = attacked_square >> 7;
                    }

                    // CHECK DOWNLEFT
                    hit_piece = false;
                    let mut attacked_square = square >> 9 ;
                    while (attacked_square > 0) && !hit_piece && attacked_square & Constants::H_FILE == 0 {
                        hit_piece = black_bishop_move(square, attacked_square, self, &mut result);
                        attacked_square = attacked_square >> 9;
                    }
                }
            }
            return result;

        }
        
        pub fn get_all_pseudo_legal_white_queen_moves(&self) -> Vec<ChessBoard> {
            let mut result: Vec<ChessBoard> = vec![];
            let white_queens = self.white_queens;
            let mut square: u64;
            let mut hit_piece: bool;
            for i in 0..64 {
                square =  1 << i;
                // check if queen occupies square
                if white_queens & square > 0 {

                    // CHECK UP
                    hit_piece = false;
                    let (mut attacked_square, mut overflow) = square.overflowing_loss_checked_shl(8);
                    while !overflow && !hit_piece {
                        
                        hit_piece = white_queen_move(square, attacked_square, self, &mut result);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(8);
                    }

                    // CHECK DOWN
                    hit_piece = false;
                    attacked_square = square >> 8;
                    while attacked_square > 0 && !hit_piece {
                        hit_piece = white_queen_move(square, attacked_square, self, &mut result);
                        attacked_square = attacked_square >> 8;
                    }

                    // CHECK LEFT
                    hit_piece = false;
                    attacked_square = square >> 1;
                    while (attacked_square & Constants::H_FILE) == 0 && !hit_piece && attacked_square > 0 {
                        hit_piece = white_queen_move(square, attacked_square, self, &mut result);
                        attacked_square = attacked_square >> 1;
                    }


                    // CHECK RIGHT
                    hit_piece = false;
                    (attacked_square, overflow) = square.overflowing_loss_checked_shl(1);
                    while (attacked_square & Constants::A_FILE) == 0 && !hit_piece && !overflow {
                        hit_piece = white_queen_move(square, attacked_square, self, &mut result);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(1);
                    }

                    // CHECK UPRIGHT
                    hit_piece = false;
                    let (mut attacked_square, mut overflow) = square.overflowing_loss_checked_shl(9);
                    while !overflow && !hit_piece && attacked_square & Constants::A_FILE == 0  {
                        hit_piece = white_queen_move(square, attacked_square, self, &mut result);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(9);
                    }
                    // CHECK UPLEFT
                    hit_piece = false;
                    let (mut attacked_square, mut overflow) = square.overflowing_loss_checked_shl(7);
                    while !overflow && !hit_piece && attacked_square & Constants::H_FILE == 0 {
                        hit_piece = white_queen_move(square, attacked_square, self, &mut result);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(7);
                    }
                    // CHECK DOWNRIGHT
                    hit_piece = false;
                    let mut attacked_square = square >> 7;
                    while (attacked_square > 0) && !hit_piece && attacked_square & Constants::A_FILE == 0 {
                        hit_piece = white_queen_move(square, attacked_square, self, &mut result);
                        attacked_square = attacked_square >> 7;
                    }

                    // CHECK DOWNLEFT
                    hit_piece = false;
                    let mut attacked_square = square >> 9 ;
                    while (attacked_square > 0) && !hit_piece && attacked_square & Constants::H_FILE == 0 {
                        hit_piece = white_queen_move(square, attacked_square, self, &mut result);
                        attacked_square = attacked_square >> 9;
                    }


                }
            }
            return result;

        }

        pub fn get_all_pseudo_legal_black_queen_moves(&self) -> Vec<ChessBoard> {
            let mut result: Vec<ChessBoard> = vec![];
            let black_queens = self.black_queens;
            let mut square: u64;
            let mut hit_piece;
            for i in 0..64 {
                square =  1 << i;
                // check if queen occupies square
                if black_queens & square > 0 {

                    // CHECK UP
                    hit_piece = false;
                    let (mut attacked_square, mut overflow) = square.overflowing_loss_checked_shl(8);
                    while !overflow && !hit_piece {
                        
                        hit_piece = black_queen_move(square, attacked_square, self, &mut result);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(8);
                    }

                    // CHECK DOWN
                    hit_piece = false;
                    attacked_square = square >> 8;
                    while attacked_square > 0 && !hit_piece {
                        hit_piece = black_queen_move(square, attacked_square, self, &mut result);
                        attacked_square = attacked_square >> 8;
                    }

                    // CHECK LEFT
                    hit_piece = false;
                    attacked_square = square >> 1;
                    while (attacked_square & Constants::H_FILE) == 0 && !hit_piece && attacked_square > 0 {
                        hit_piece = black_queen_move(square, attacked_square, self, &mut result);
                        attacked_square = attacked_square >> 1;
                    }


                    // CHECK RIGHT
                    hit_piece = false;
                    (attacked_square, overflow) = square.overflowing_loss_checked_shl(1);
                    while (attacked_square & Constants::A_FILE) == 0 && !hit_piece && !overflow {
                        hit_piece = black_queen_move(square, attacked_square, self, &mut result);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(1);
                    }

                    // CHECK UPRIGHT
                    hit_piece = false;
                    let (mut attacked_square, mut overflow) = square.overflowing_loss_checked_shl(9);
                    while !overflow && !hit_piece && attacked_square & Constants::A_FILE == 0  {
                        hit_piece = black_queen_move(square, attacked_square, self, &mut result);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(9);
                    }
                    // CHECK UPLEFT
                    hit_piece = false;
                    let (mut attacked_square, mut overflow) = square.overflowing_loss_checked_shl(7);
                    while !overflow && !hit_piece && attacked_square & Constants::H_FILE == 0 {
                        hit_piece = black_queen_move(square, attacked_square, self, &mut result);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(7);
                    }
                    // CHECK DOWNRIGHT
                    hit_piece = false;
                    let mut attacked_square = square >> 7;
                    while (attacked_square > 0) && !hit_piece && attacked_square & Constants::A_FILE == 0 {
                        hit_piece = black_queen_move(square, attacked_square, self, &mut result);
                        attacked_square = attacked_square >> 7;
                    }

                    // CHECK DOWNLEFT
                    hit_piece = false;
                    let mut attacked_square = square >> 9 ;
                    while (attacked_square > 0) && !hit_piece && attacked_square & Constants::H_FILE == 0 {
                        hit_piece = black_queen_move(square, attacked_square, self, &mut result);
                        attacked_square = attacked_square >> 9;
                    }


                }
            }
            return result;
        }
        
        pub fn get_all_pseudo_legal_white_king_moves(&self) -> Vec<ChessBoard> {
            let mut result: Vec<ChessBoard> = vec![];
            let white_king = self.white_king;

            // CHECK UP
            let (mut attacked_square, mut overflow) = white_king.overflowing_loss_checked_shl(8);
            if !overflow && (self.get_all_white_pieces() & attacked_square) == 0 {            
                white_king_move(white_king, attacked_square, self, &mut result);
            }

            // CHECK DOWN
            attacked_square = white_king >> 8;
            if attacked_square > 0 && (self.get_all_white_pieces() & attacked_square) == 0 {
                white_king_move(white_king, attacked_square, self, &mut result);
            }

            // CHECK LEFT
            attacked_square = white_king >> 1;
            if (attacked_square & Constants::H_FILE) == 0 && (self.get_all_white_pieces() & attacked_square) == 0 && attacked_square > 0 {
                white_king_move(white_king, attacked_square, self, &mut result);
            }

            // CHECK RIGHT
            (attacked_square, overflow) = white_king.overflowing_loss_checked_shl(1);
            if (attacked_square & Constants::A_FILE) == 0 && (self.get_all_white_pieces() & attacked_square) == 0 && !overflow {
                white_king_move(white_king, attacked_square, self, &mut result);
            }

            // CHECK UPRIGHT
            (attacked_square, overflow) = white_king.overflowing_loss_checked_shl(9);
            if !overflow && (self.get_all_white_pieces() & attacked_square) == 0 && attacked_square & Constants::A_FILE == 0  {
                white_king_move(white_king, attacked_square, self, &mut result);
            }
            // CHECK UPLEFT
            (attacked_square, overflow) = white_king.overflowing_loss_checked_shl(7);
            if !overflow && (self.get_all_white_pieces() & attacked_square) == 0 && attacked_square & Constants::H_FILE == 0 {
                white_king_move(white_king, attacked_square, self, &mut result);
            }

            // CHECK DOWNRIGHT
            attacked_square = white_king >> 7;
            if (attacked_square > 0) && (self.get_all_white_pieces() & attacked_square) == 0 && attacked_square & Constants::A_FILE == 0 {
                white_king_move(white_king, attacked_square, self, &mut result);
            }

            // CHECK DOWNLEFT
            attacked_square = white_king >> 9 ;
            if (attacked_square > 0) && (self.get_all_white_pieces() & attacked_square) == 0 && attacked_square & Constants::H_FILE == 0 {
                white_king_move(white_king, attacked_square, self, &mut result);
            }

            // CHECK CASTLE
            if !self.white_moved_king {
                // CHECK IF THERE ARE PIECES ON CASTLING SQUARES AND IF ROOK IS MOVED
                if(!self.white_moved_A_rook) && (white_king >> 1 & self.get_all_pieces() == 0) && (white_king >> 2 & self.get_all_pieces() == 0) {
                    if !are_white_short_castling_squares_under_attack(self){
                        white_short_castle(self, &mut result);
                    }
                }

                // CHECK IF THERE ARE PIECES ON CASTLING SQUARES AND IF ROOK IS MOVED
                if (!self.white_moved_H_rook) && (white_king << 1 & self.get_all_pieces() == 0) && (white_king << 2 & self.get_all_pieces() == 0) {
                    if !are_white_long_castling_squares_under_attack(self){
                        white_long_castle(self, &mut result);
                    }
                }
            }

            return result;
        }
        
        pub fn get_all_pseudo_legal_black_king_moves(&self) -> Vec<ChessBoard> {
            let mut result: Vec<ChessBoard> = vec![];
            let black_king = self.black_king;

            // CHECK UP
            let (mut attacked_square, mut overflow) = black_king.overflowing_loss_checked_shl(8);
            if !overflow && (self.get_all_black_pieces() & attacked_square) == 0 {            
                black_king_move(black_king, attacked_square, self, &mut result);
            }

            // CHECK DOWN
            attacked_square = black_king >> 8;
            if attacked_square > 0 && (self.get_all_black_pieces() & attacked_square) == 0 {
                black_king_move(black_king, attacked_square, self, &mut result);
            }

            // CHECK LEFT
            attacked_square = black_king >> 1;
            if (attacked_square & Constants::H_FILE) == 0 && (self.get_all_black_pieces() & attacked_square) == 0 && attacked_square > 0 {
                black_king_move(black_king, attacked_square, self, &mut result);
            }

            // CHECK RIGHT
            (attacked_square, overflow) = black_king.overflowing_loss_checked_shl(1);
            if (attacked_square & Constants::A_FILE) == 0 && (self.get_all_black_pieces() & attacked_square) == 0 && !overflow {
                black_king_move(black_king, attacked_square, self, &mut result);
            }

            // CHECK UPRIGHT
            (attacked_square, overflow) = black_king.overflowing_loss_checked_shl(9);
            if !overflow && (self.get_all_black_pieces() & attacked_square) == 0 && attacked_square & Constants::A_FILE == 0  {
                black_king_move(black_king, attacked_square, self, &mut result);
            }
            // CHECK UPLEFT
            (attacked_square, overflow) = black_king.overflowing_loss_checked_shl(7);
            if !overflow && (self.get_all_black_pieces() & attacked_square) == 0 && attacked_square & Constants::H_FILE == 0 {
                black_king_move(black_king, attacked_square, self, &mut result);
            }

            // CHECK DOWNRIGHT
            attacked_square = black_king >> 7;
            if (attacked_square > 0) && (self.get_all_black_pieces() & attacked_square) == 0 && attacked_square & Constants::A_FILE == 0 {
                black_king_move(black_king, attacked_square, self, &mut result);
            }

            // CHECK DOWNLEFT
            attacked_square = black_king >> 9 ;
            if (attacked_square > 0) && (self.get_all_black_pieces() & attacked_square) == 0 && attacked_square & Constants::H_FILE == 0 {
                black_king_move(black_king, attacked_square, self, &mut result);
            }

            // CHECK CASTLE
            if !self.black_moved_king {
                // CHECK IF THERE ARE PIECES ON CASTLING SQUARES AND IF ROOK IS MOVED
                if(!self.black_moved_A_rook) && (black_king >> 1 & self.get_all_pieces() == 0) && (black_king >> 2 & self.get_all_pieces() == 0) {
                    if !are_black_short_castling_squares_under_attack(self){
                        black_short_castle(self, &mut result);
                    }
                }

                // CHECK IF THERE ARE PIECES ON CASTLING SQUARES AND IF ROOK IS MOVED
                if (!self.black_moved_H_rook) && (black_king << 1 & self.get_all_pieces() == 0) && (black_king << 2 & self.get_all_pieces() == 0) {
                    if !are_black_long_castling_squares_under_attack(self){
                        black_long_castle(self, &mut result);
                    }
                }
            }

            return result;
        }
        
        pub fn get_all_pseudo_legal_white_moves(&self) -> Vec<ChessBoard> {
            let mut pseudo_legal_moves: Vec<ChessBoard> = vec![];

            pseudo_legal_moves.append(&mut self.get_all_pseudo_legal_white_pawn_moves());
            pseudo_legal_moves.append(&mut self.get_all_pseudo_legal_white_rook_moves());
            pseudo_legal_moves.append(&mut self.get_all_pseudo_legal_white_knight_moves());
            pseudo_legal_moves.append(&mut self.get_all_pseudo_legal_white_bishop_moves());
            pseudo_legal_moves.append(&mut self.get_all_pseudo_legal_white_queen_moves());
            pseudo_legal_moves.append(&mut self.get_all_pseudo_legal_white_king_moves());

            return pseudo_legal_moves;
        }

        pub fn get_all_pseudo_legal_black_moves(&self) -> Vec<ChessBoard> {
            let mut pseudo_legal_moves: Vec<ChessBoard> = vec![];
            // TODO: add other pieces
            pseudo_legal_moves.append(&mut self.get_all_pseudo_legal_black_pawn_moves());
            pseudo_legal_moves.append(&mut self.get_all_pseudo_legal_black_rook_moves());
            pseudo_legal_moves.append(&mut self.get_all_pseudo_legal_black_knight_moves());
            pseudo_legal_moves.append(&mut self.get_all_pseudo_legal_black_bishop_moves());
            pseudo_legal_moves.append(&mut self.get_all_pseudo_legal_black_queen_moves());
            pseudo_legal_moves.append(&mut self.get_all_pseudo_legal_black_king_moves());

            return pseudo_legal_moves;
        }

        pub fn get_all_legal_white_moves(&self, pseudo_legal_white_moves: Option<&Vec<ChessBoard>>) -> (Vec<ChessBoard>, Vec<Vec<ChessBoard>>) {
            
            let mut result: Vec<ChessBoard> = vec![];
            let _pseudo_legal_white_moves: &Vec<ChessBoard>;
            let temp ;
            // let now = Instant::now();
            if pseudo_legal_white_moves.is_some() {
                // let now = Instant::now();
                _pseudo_legal_white_moves = pseudo_legal_white_moves.unwrap();
                // println!("{}", now.elapsed().as_micros());
            } else {
                // println!("A");
                temp = self.get_all_pseudo_legal_white_moves();
                _pseudo_legal_white_moves = &temp;
            }
            
            // let elapsed = now.elapsed();
            // println!("{}", elapsed.as_micros());

            let mut is_checked_after_white_move: bool;
            let mut pseudo_legal_black_moves;
            let mut pseudo_legal_black_moves_for_each_white_move = vec![];

            for mov in _pseudo_legal_white_moves {
                // let now = Instant::now();
                (is_checked_after_white_move, pseudo_legal_black_moves) = mov.is_white_king_checked();
                // let elapsed = now.elapsed();
                // println!("is king checked? {}", elapsed.as_micros());

                if !is_checked_after_white_move {
                    result.push(*mov);
                    pseudo_legal_black_moves_for_each_white_move.push(pseudo_legal_black_moves)
                } 
            }

            return (result, pseudo_legal_black_moves_for_each_white_move);
        }

        pub fn get_all_legal_black_moves(&self, pseudo_legal_black_moves: Option<&Vec<ChessBoard>>) -> (Vec<ChessBoard>, Vec<Vec<ChessBoard>>) {

            let mut result: Vec<ChessBoard> = vec![];
            // let now = Instant::now();

            let _pseudo_legal_black_moves: &Vec<ChessBoard>;
            let temp ;

            if pseudo_legal_black_moves.is_some() {
                _pseudo_legal_black_moves = pseudo_legal_black_moves.unwrap();
            } else {
                // println!("B");
                temp = self.get_all_pseudo_legal_black_moves();
                _pseudo_legal_black_moves = &temp;
            }
            // let elapsed = now.elapsed();
            // println!("{}", elapsed.as_micros());

            let mut is_checked_after_black_move: bool;
            let mut pseudo_legal_white_moves;
            let mut pseudo_legal_white_moves_for_each_black_move = vec![];

            for mov in _pseudo_legal_black_moves {
                (is_checked_after_black_move, pseudo_legal_white_moves) = mov.is_black_king_checked();

                if !is_checked_after_black_move {
                    result.push(*mov);
                    pseudo_legal_white_moves_for_each_black_move.push(pseudo_legal_white_moves)
                }
            }

            return (result, pseudo_legal_white_moves_for_each_black_move);

        }
        
        pub fn legal_moves(&self, pseudo_legal: Option<&Vec<ChessBoard>>) -> (Vec<ChessBoard>, Vec<Vec<ChessBoard>>){
            let param;
            if pseudo_legal.is_some() {
                param = pseudo_legal;
            } else {
                param = None
            }

            if self.white_to_move {
                return self.get_all_legal_white_moves(param);
            } else {
                return self.get_all_legal_black_moves(param); 
            }
        }

        pub fn whiteDoubledPawns(&self) -> u32 {
            let pawns = self.white_pawns;
            let mask = pawns << 8;
            return (pawns & mask).count_ones();
        }

        pub fn blackDoubledPawns(&self) -> u32 {
            let pawns = self.black_pawns;
            let mask = pawns >> 8;
            return (pawns & mask).count_ones();
        }

        pub fn whiteBlockedPawns(&self) -> u32 {
            let pawns = self.white_pawns;
            let mask = pawns << 8;
            let all_pieces = self.get_all_pieces();

            return (all_pieces & mask).count_ones();
        }

        pub fn blackBlockedPawns(&self) -> u32 {
            let pawns = self.black_pawns;
            let mask = pawns >> 8;
            let all_pieces = self.get_all_pieces();

            return (all_pieces & mask).count_ones();
        }

        // pub fn perft(&self, depth: u64) {
        //     if depth == 0 {
        //         return;
        //     }

        //     if self.white_to_move {
        //         for pos in self.get_all_legal_white_moves(None).0{
        //             pos.perft(depth - 1);
        //         }
        //     } else {
        //         for pos in self.get_all_legal_black_moves(None).0{
        //             pos.perft(depth - 1);
        //         }
        //     }
        // }

        pub fn _perft(&self, depth: u64,  currDepth: u64, numberOfPositions: u64, pseudo_legal: Option<&Vec<ChessBoard>>) -> u64{
            let mut _numberOfPositions= numberOfPositions;
            let param;
            if pseudo_legal.is_some() {
                param = pseudo_legal;
            } else {
                param = None;
            }
            // let mut currDepth = currDepth;
            if depth == currDepth {
                _numberOfPositions += 1;
                
            } else if depth < currDepth {
                return 0;
            } else {
                if self.white_to_move {
                    let (positions, next) = self.get_all_legal_white_moves(param);
                    for (i, pos) in positions.iter().enumerate() {
                        _numberOfPositions = pos._perft(depth, currDepth + 1, _numberOfPositions, Some(&next[i]));
                    }
                } else {
                    let (positions, next) = self.get_all_legal_black_moves(param);
                    for (i, pos) in positions.iter().enumerate() {
                        _numberOfPositions = pos._perft(depth, currDepth + 1, _numberOfPositions, Some(&next[i]));
                    }
                }
            }

            // currDepth -= 1;
            // println!("{}", _numberOfPositions);
            return _numberOfPositions;

        }

        pub fn perft(&self, depth: u64) -> (HashMap<ChessBoard, u64>, u64) {
            let mut result = HashMap::new();
            let mut total = 0;
            let (positions, next) ;
            if self.white_to_move {
                (positions, next) = self.get_all_legal_white_moves(None);
            } else {
                (positions, next) = self.get_all_legal_black_moves(None);
            }

            for (i, pos) in positions.iter().enumerate() {
                let num_of_positions = pos._perft(depth - 1, 0, 0, Some(&next[i]));
                total += num_of_positions;
                result.insert(*pos, num_of_positions);
            }

            return (result, total);
        }

        pub fn _minimax(&self, depth: u64, pseudo_legal: Option<&Vec<ChessBoard>>) -> f32 {
            if depth == 0 {
                return self.evaluate();
            }

            let param;
            if pseudo_legal.is_some() {
                param = pseudo_legal;
            } else {
                param = None;
            }

            if self.white_to_move {
                let mut value = f32::NEG_INFINITY;
                let (moves, next_pseudo_legal) = self.get_all_legal_white_moves(param);
                for (i, mov) in moves.iter().enumerate() {
                    value = f32::max(value, mov._minimax(depth - 1, Some(&next_pseudo_legal[i])));
                }
                return value;
            } else {
                let mut value = f32::INFINITY;
                let (moves, next_pseudo_legal) = self.get_all_legal_black_moves(param);
        
                for (i, mov) in moves.iter().enumerate() {
                    value = f32::min(value, mov._minimax(depth - 1, Some(&next_pseudo_legal[i])));
                }
                return value;
            }

        }


        pub fn minimax(&self, depth: u64) -> Vec<(ChessBoard, f32)> {
            let mut res: Vec<(ChessBoard, f32)> = vec![];

            let (moves, next_pseudo_legal) = self.legal_moves(None);
            for (i, mov) in moves.iter().enumerate() {
                res.push((*mov, self._minimax(depth - 1, Some(&next_pseudo_legal[i]))))
            }
            

            res.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            return res;
        }

        pub fn evaluate(&self) -> f32 {
            let (wK, bK) = (self.white_king.count_ones() as i32, self.black_king.count_ones() as i32);
            let (wQ, bQ) = (self.white_queens.count_ones() as i32, self.black_queens.count_ones() as i32);
            let (wR, bR) = (self.white_rooks.count_ones() as i32, self.black_rooks.count_ones() as i32);
            let (wB, bB) = (self.white_bishops.count_ones() as i32, self.black_bishops.count_ones() as i32);
            let (wN, bN) = (self.white_knights.count_ones() as i32, self.black_knights.count_ones() as i32);
            let (wP, bP) = (self.white_pawns.count_ones() as i32, self.black_pawns.count_ones() as i32);

            // Doubled pawns
            let (wDoubled, bDoubled) = (self.whiteDoubledPawns() as i32, self.blackDoubledPawns() as i32); 
            // Blocked pawns
            let (wBlocked, bBlocked) = (self.whiteBlockedPawns() as i32, self.blackBlockedPawns() as i32);

            // TODO: add mobility
            let evaluation :f32 = (200 * (wK - bK)
                                + 9 * (wQ - bQ)
                                + 5 * (wR - bR)
                                + 3 * (wB - bB + wN - bN)
                                + 1 * (wP - bP)) as f32
                                - 0.5 * (wDoubled - bDoubled + wBlocked - bBlocked) as f32;

            return evaluation;
        }

    }

    
    
    pub struct Constants;
    impl Constants {
        pub const A_FILE: u64 = 0x0101010101010101;
        pub const B_FILE: u64 = 0x0202020202020202;
        pub const G_FILE: u64 = 0x04040404040404040;
        pub const H_FILE: u64 = 0x8080808080808080;
        pub const FIRST_RANK: u64 = 0x00000000000000FF;
        pub const FOURTH_RANK: u64 = 0xFF000000;
        pub const FIFTH_RANK: u64 = 0xFF00000000;
        pub const EIGHT_RANK: u64 = 0xFF00000000000000;
        pub const A1_H8_DIAGONAL: u64 = 0x8040201008040201;
        pub const H1_A8_ANTIDIAGONAL: u64 = 0x0102040810204080;
        pub const LIGHT_SQUARES: u64 = 0x55AA55AA55AA55AA;
        pub const DARK_SQUARES: u64 = 0xAA55AA55AA55AA55;
        pub const SECOND_RANK: u64 = 0xFF00;
        pub const SEVENTH_RANK: u64 = 0xFF000000000000;
    }

}