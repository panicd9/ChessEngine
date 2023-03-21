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
    use std::vec;

    use crate::white_utils::*;

    trait OverflowingLeftShift {
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
      
    #[derive(Clone, Copy)]
    pub struct ChessBoard {

        pub white_to_move: bool,
        pub white_can_castle: bool,
        pub white_pawns: u64,
        pub white_rooks: u64,
        pub white_knights: u64,
        pub white_bishops: u64,
        pub white_queens: u64,
        pub white_king: u64,
    
        pub black_can_castle: bool,
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
                white_can_castle: true,
                white_pawns: 0xFF00,
                white_rooks: 0x81,
                white_knights: 0x42,
                white_bishops: 0x24,
                white_queens: 0x8,
                white_king: 0x10,
            
                black_can_castle: true,
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
            if self.get_all_white_pieces() & square == 0 { println!("GRESKA 1 !!!"); return; }; // TODO: remove

            if self.white_pawns & square > 0 { self.white_pawns -= square; return; }
            if self.white_rooks & square > 0 { self.white_rooks -= square; return; }
            if self.white_knights & square > 0 { self.white_knights -= square; return; }
            if self.white_bishops & square > 0 { self.white_bishops -= square; return; }
            if self.white_queens & square > 0 { self.white_queens -= square; return; }
        }

        pub fn remove_black_piece(&mut self, square: u64) {
            if self.get_all_black_pieces() & square == 0 { println!("GRESKA 2 !!!"); return; }; // TODO: remove

            if self.black_pawns & square > 0 { self.black_pawns -= square; return; }
            if self.black_rooks & square > 0 { self.black_rooks -= square; return; }
            if self.black_knights & square > 0 { self.black_knights -= square; return; }
            if self.black_bishops & square > 0 { self.black_bishops -= square; return; }
            if self.black_queens & square > 0 { self.black_queens -= square; return; }
        }

        pub fn get_all_pieces(&self) -> u64 {
            self.get_all_white_pieces() | self.get_all_black_pieces()
        }

        pub fn print_chessboard(&self) {
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
            let mut next_legal_positions = self.get_all_pseudo_legal_black_moves();
            let starting_len = next_legal_positions.len();

            next_legal_positions.retain(|&e| e.white_king > 0);
            let curr_len = next_legal_positions.len();

            let is_king_checked = starting_len != curr_len;

            return (is_king_checked, next_legal_positions);
        }

        pub fn is_black_king_checked(&self) -> (bool, Vec<ChessBoard>) {
            let mut next_legal_positions = self.get_all_pseudo_legal_white_moves();
            let starting_len = next_legal_positions.len();

            next_legal_positions.retain(|&e| e.black_king > 0);
            let curr_len = next_legal_positions.len();

            let is_king_checked = starting_len != curr_len;

            return (is_king_checked, next_legal_positions);
        }
        
        pub fn get_all_pseudo_legal_white_pawn_moves(&self) -> Vec<ChessBoard> {
            let mut result: Vec<ChessBoard> = vec![];
            let white_pawns = self.white_pawns;
            let mut square: u64;
            // 64 - 8 = 56 // cant occupy 1st rank
            for i in 8..64 {
                square =  2_u64.pow(i);
                // check if pawn occupies square
                if white_pawns & square > 0 {

                    // CHECK FOR UPRIGHT TAKE ( attacked_square = square << 9 )
                    let attacked_square = square * 2_u64.pow(9);
                    // cant be on A file after taking upright
                    if attacked_square & Constants::A_FILE == 0 {
                        //check if there is enemy piece
                        check_white_pawn_take(square, attacked_square, self, &mut result);
                        // if attacked_square & self.get_all_black_pieces() > 0 {
                        //     let mut new_chessboard = self.clone();
                        //     new_chessboard.white_pawns -= square;
                        //     new_chessboard.white_pawns += attacked_square;
                        //     new_chessboard.remove_black_piece(attacked_square);
                        //     result.push(new_chessboard);
                        // }
                    }

                    // CHECK FOR UPLEFT TAKE ( attacked_square = square << 7 )
                    let attacked_square = square * 2_u64.pow(7);
                    // cant be on H file after taking upleft
                    if attacked_square & Constants::H_FILE == 0 {
                        //check if there is enemy piece
                        if attacked_square & self.get_all_black_pieces() > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_pawns -= square;
                            new_chessboard.white_pawns += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        }
                    }

                    // CHECK FOR 1 SQUARE FORWARD
                    let forward_square_1 = square * 2_u64.pow(8);

                    if (forward_square_1 & self.get_all_pieces()) == 0 {
                        // PUSH PAWN IF NOT ON EIGHT RANK
                        if (forward_square_1 & Constants::EIGHT_RANK) == 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_pawns -= square;
                            new_chessboard.white_pawns += forward_square_1;
                            result.push(new_chessboard);
                        }
                        // TODO: PROMOTE TO QUEEN 
                        else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_pawns -= square;
                            new_chessboard.white_queens += forward_square_1;
                            result.push(new_chessboard);
                        }



                    }

                    // CHECK FOR 2 SQUARES FORWARD
                    let forward_square_2 = square * 2_u64.pow(16);

                    if square & Constants::SECOND_RANK > 0 && forward_square_1 & self.get_all_pieces() == 0 && forward_square_2 & self.get_all_pieces() == 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_pawns -= square;
                        new_chessboard.white_pawns += forward_square_2;
                        result.push(new_chessboard);
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
            for i in 8..64 {
                square =  2_u64.pow(i);
                // check if pawn occupies square
                if black_pawns & square > 0 {

                    // CHECK FOR DOWNRIGHT TAKE
                    let attacked_square = square >> 7;
                    // cant be on A file after taking downright
                    if attacked_square & Constants::A_FILE == 0 {
                        //check if there is enemy piece
                        if attacked_square & self.get_all_white_pieces() > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.black_pawns -= square;
                            new_chessboard.black_pawns += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        }
                    }

                    // CHECK FOR DOWNLEFT TAKE
                    let attacked_square = square >> 9;
                    // cant be on H file after taking upleft
                    if attacked_square & Constants::H_FILE == 0 {
                        //check if there is enemy piece
                        if attacked_square & self.get_all_white_pieces() > 0 {
                            let mut new_chessboard = self.clone();
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
                            new_chessboard.black_pawns -= square;
                            new_chessboard.black_pawns += forward_square_1;
                            result.push(new_chessboard);
                        }
                        // TODO: PROMOTE TO QUEEN 
                        else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.black_pawns -= square;
                            new_chessboard.black_queens += forward_square_1;
                            result.push(new_chessboard);
                        }



                    }

                    // CHECK FOR 2 SQUARES FORWARD
                    let forward_square_2 = square >> 16;

                    if square & Constants::SEVENTH_RANK > 0 && forward_square_1 & self.get_all_pieces() == 0 && forward_square_2 & self.get_all_pieces() == 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.black_pawns -= square;
                        new_chessboard.black_pawns += forward_square_2;
                        result.push(new_chessboard);
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
                square =  2_u64.pow(i);
                // check if rook occupies square
                if white_rooks & square > 0 {

                    // CHECK UP
                    let (mut attacked_square, mut overflow) = square.overflowing_loss_checked_shl(8);
                    while !overflow && (self.get_all_pieces() & attacked_square) == 0 {
                        
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_rooks -= square;
                        new_chessboard.white_rooks += attacked_square;
                        result.push(new_chessboard);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(8);
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_black_pieces() & attacked_square > 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_rooks -= square;
                        new_chessboard.white_rooks += attacked_square;
                        new_chessboard.remove_black_piece(attacked_square);
                        result.push(new_chessboard);
                    }

                    // CHECK DOWN
                    attacked_square = square >> 8;
                    while attacked_square > 0 && (self.get_all_pieces() & attacked_square) == 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_rooks -= square;
                        new_chessboard.white_rooks += attacked_square;
                        result.push(new_chessboard);
                        attacked_square = attacked_square >> 8;
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_black_pieces() & attacked_square > 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_rooks -= square;
                        new_chessboard.white_rooks += attacked_square;
                        new_chessboard.remove_black_piece(attacked_square);
                        result.push(new_chessboard);
                    }

                    // CHECK LEFT
                    attacked_square = square >> 1;
                    while (attacked_square & Constants::H_FILE) == 0 && (self.get_all_pieces() & attacked_square) == 0 && attacked_square > 0 {
                        
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_rooks -= square;
                        new_chessboard.white_rooks += attacked_square;
                        result.push(new_chessboard);
                        attacked_square = attacked_square >> 1;
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_black_pieces() & attacked_square > 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_rooks -= square;
                        new_chessboard.white_rooks += attacked_square;
                        new_chessboard.remove_black_piece(attacked_square);
                        result.push(new_chessboard);
                    }

                    // CHECK RIGHT
                    (attacked_square, overflow) = square.overflowing_loss_checked_shl(1);
                    while (attacked_square & Constants::A_FILE) == 0 && (self.get_all_pieces() & attacked_square) == 0 && !overflow {
                        let mut new_chessboard = self.clone();
                        new_chessboard.white_rooks -= square;
                        new_chessboard.white_rooks += attacked_square;
                        result.push(new_chessboard);
                        (attacked_square, overflow) = square.overflowing_loss_checked_shl(1);
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_black_pieces() & attacked_square > 0 {
                        let mut new_chessboard = self.clone();
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
                square =  2_u64.pow(i);
                // check if rook occupies square
                if black_rooks & square > 0 {

                    // CHECK UP
                    let (mut attacked_square, mut overflow) = square.overflowing_loss_checked_shl(8);
                    while !overflow && (self.get_all_pieces() & attacked_square) == 0 && attacked_square != 2_u64.pow(64){
                        let mut new_chessboard = self.clone();
                        new_chessboard.black_rooks -= square;
                        new_chessboard.black_rooks += attacked_square;
                        result.push(new_chessboard);
                        (attacked_square, overflow) = attacked_square.overflowing_loss_checked_shl(8);
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_white_pieces() & attacked_square > 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.black_rooks -= square;
                        new_chessboard.black_rooks += attacked_square;
                        new_chessboard.remove_black_piece(attacked_square);
                        result.push(new_chessboard);
                    }

                    // CHECK DOWN
                    attacked_square = square >> 8;
                    while attacked_square > 0 && (self.get_all_pieces() & attacked_square) == 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.black_rooks -= square;
                        new_chessboard.black_rooks += attacked_square;
                        result.push(new_chessboard);
                        attacked_square = attacked_square >> 8;
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_white_pieces() & attacked_square > 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.black_rooks -= square;
                        new_chessboard.black_rooks += attacked_square;
                        new_chessboard.remove_white_piece(attacked_square);
                        result.push(new_chessboard);
                    }

                    // CHECK LEFT
                    attacked_square = square >> 1;
                    while (attacked_square & Constants::H_FILE) == 0 && (self.get_all_pieces() & attacked_square) == 0 && attacked_square > 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.black_rooks -= square;
                        new_chessboard.black_rooks += attacked_square;
                        result.push(new_chessboard);
                        attacked_square = attacked_square >> 1;
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_white_pieces() & attacked_square > 0 {
                        let mut new_chessboard = self.clone();
                        new_chessboard.black_rooks -= square;
                        new_chessboard.black_rooks += attacked_square;
                        new_chessboard.remove_white_piece(attacked_square);
                        result.push(new_chessboard);
                    }

                    // CHECK RIGHT
                    (attacked_square, overflow) = square.overflowing_loss_checked_shl(1);
                    while (attacked_square & Constants::A_FILE) == 0 && (self.get_all_pieces() & attacked_square) == 0 && !overflow {
                        let mut new_chessboard = self.clone();
                        new_chessboard.black_rooks -= square;
                        new_chessboard.black_rooks += attacked_square;
                        result.push(new_chessboard);
                        (attacked_square, overflow) = square.overflowing_loss_checked_shl(1);
                        
                    }
                    //check if while exited because of enemy piece
                    if self.get_all_white_pieces() & attacked_square > 0 {
                        let mut new_chessboard = self.clone();
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
                square =  2_u64.pow(i);
                // check if knight occupies square
                if white_knights & square > 0 {
                    let (mut attacked_square, mut overflow ) = square.overflowing_loss_checked_shl(17);

                    // UP-RIGHT
                    if !overflow && (attacked_square & Constants::A_FILE) == 0 && (attacked_square & self.get_all_white_pieces()) == 0 {
                        if (self.get_all_black_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
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
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            
                            let mut new_chessboard = self.clone();
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
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // DOWN-LEFT
                    attacked_square  = square >> 17;
                    if (attacked_square > 0) && (attacked_square & Constants::H_FILE) == 0 && self.get_all_white_pieces() == 0 {
                        if (self.get_all_black_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
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
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
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
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
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
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
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
                            new_chessboard.white_knights -= square;
                            new_chessboard.white_knights += attacked_square;
                            new_chessboard.remove_black_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
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
                square =  2_u64.pow(i);
                // check if knight occupies square
                if black_knights & square > 0 {
                    let (mut attacked_square, mut overflow ) = square.overflowing_loss_checked_shl(17);

                    // UP-RIGHT
                    if !overflow && (attacked_square & Constants::A_FILE) == 0 && (attacked_square & self.get_all_black_pieces()) == 0 {
                        if (self.get_all_white_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                    // UP-LEFT
                    (attacked_square, overflow)  = square.overflowing_loss_checked_shl(15);

                    if !overflow && (attacked_square & Constants::H_FILE) == 0 && (attacked_square & self.get_all_white_pieces()) == 0 {
                        
                        if (self.get_all_white_pieces() & attacked_square) > 0 {
                            let mut new_chessboard = self.clone();
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
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
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
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
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
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
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
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
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
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
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
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
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            new_chessboard.remove_white_piece(attacked_square);
                            result.push(new_chessboard);
                        } else {
                            let mut new_chessboard = self.clone();
                            new_chessboard.black_knights -= square;
                            new_chessboard.black_knights += attacked_square;
                            result.push(new_chessboard);
                        }
                    }

                }
            }
            return result;
        }

        // pub fn get_all_pseudo_legal_black_bishop_moves(&self) -> Vec<ChessBoard> {}
        
        pub fn get_all_pseudo_legal_white_moves(&self) -> Vec<ChessBoard> {
            let mut psedo_legal_moves: Vec<ChessBoard> = vec![];
            // TODO: add other pieces
            psedo_legal_moves.append(&mut self.get_all_pseudo_legal_white_pawn_moves());
            psedo_legal_moves.append(&mut self.get_all_pseudo_legal_white_rook_moves());
            psedo_legal_moves.append(&mut self.get_all_pseudo_legal_white_knight_moves());
            
            return psedo_legal_moves;
        }

        pub fn get_all_pseudo_legal_black_moves(&self) -> Vec<ChessBoard> {
            let mut pseudo_legal_moves: Vec<ChessBoard> = vec![];
            // TODO: add other pieces
            pseudo_legal_moves.append(&mut self.get_all_pseudo_legal_black_pawn_moves());
            pseudo_legal_moves.append(&mut self.get_all_pseudo_legal_black_rook_moves());
            pseudo_legal_moves.append(&mut self.get_all_pseudo_legal_black_knight_moves());
            return pseudo_legal_moves;
        }

        pub fn get_all_legal_white_moves(&self) -> (Vec<ChessBoard>, Vec<Vec<ChessBoard>>) {
            
            let mut result: Vec<ChessBoard> = vec![];
            let psedo_legal_moves: Vec<ChessBoard> = self.get_all_pseudo_legal_white_moves();

            let mut is_checked_after_white_move: bool;
            let mut black_moves;
            let mut black_moves_for_each_white_move: Vec<Vec<ChessBoard>> = vec![];

            for mov in psedo_legal_moves {
                (is_checked_after_white_move, black_moves) = mov.is_white_king_checked();

                if !is_checked_after_white_move {
                    result.push(mov);
                    black_moves_for_each_white_move.push(black_moves)
                } 
            }

            return (result, black_moves_for_each_white_move);
        }

        pub fn get_all_legal_black_moves(&self) -> Vec<ChessBoard> {
            // TODO: add other pieces
            let mut result: Vec<ChessBoard> = vec![];
            result.append(&mut self.get_all_pseudo_legal_black_pawn_moves());
            result.append(&mut self.get_all_pseudo_legal_black_rook_moves());
            result.append(&mut self.get_all_pseudo_legal_black_knight_moves());
            
            return result;
        }

    }

    
    
    struct Constants;
    impl Constants {
        pub const A_FILE: u64 = 0x0101010101010101;
        pub const B_FILE: u64 = 0x0202020202020202;
        pub const G_FILE: u64 = 0x04040404040404040;
        pub const H_FILE: u64 = 0x8080808080808080;
        pub const FIRST_RANK: u64 = 0x00000000000000FF;
        pub const EIGHT_RANK: u64 = 0xFF00000000000000;
        pub const A1_H8_DIAGONAL: u64 = 0x8040201008040201;
        pub const H1_A8_ANTIDIAGONAL: u64 = 0x0102040810204080;
        pub const LIGHT_SQUARES: u64 = 0x55AA55AA55AA55AA;
        pub const DARK_SQUARES: u64 = 0xAA55AA55AA55AA55;
        pub const SECOND_RANK: u64 = 0xFF00;
        pub const SEVENTH_RANK: u64 = 0xFF000000000000;
    }
}
