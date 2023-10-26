#![allow(non_snake_case)]

mod chessboard;
mod white_utils;
mod black_utils;
use std::{vec, env, time::Instant, collections::HashMap};

use crate::chessboard::chessboard::ChessBoard;

const DEPTH: u64 = 4;
fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    // env::set_var("RUST_MIN_STACK", "33554432");
    let mut cb = ChessBoard::new();
    

    // cb.white_queens = 0x1000000000000;
    // cb.black_pawns = 0xFE000000000000;
    // cb.white_to_move = false;

    // cb.black_rooks = 0x8001000000000000;
    // cb.black_pawns = 0xFE000000000000;
    // cb.white_queens = 0;

    // cb.white_queens = 0x8000000;

    // cb.white_pawns = 0x80007F00;
    // cb.white_to_move = false;
    // cb.black_pawns = 0xFE000100000000;
    // cb.white_to_move = true;
    // cb.white_rooks = 0x800001;
    // cb.white_to_move = false;
    // cb.black_pawns = 0xFE000001000000;
    // cb.white_to_move = true;

    cb.print_chessboard();
    // let x = cb.get_all_legal_white_moves(None).0;
    // for a in x {
    //     a.print_chessboard();
    //     // println!("AA");
    //     println!("{}", a.evaluate());
    // }
    let now = Instant::now();
    // cb.perft(3);
    let numOfPositions = cb.perft(4);
    // let search = cb.minimax(DEPTH);
    // for mov in search.iter() {
    //     mov.0.print_chessboard();
    //     println!("Eval: {}", mov.0.evaluate());
    //     println!("Minimax eval: {}", mov.1);
    // }

    println!("------------------------------------");
    // let search2 = search.iter().last().unwrap().0.minimax(3);

    // for mov in search2.iter().rev() {
    //     mov.0.print_chessboard();
    //     println!("Eval: {}", mov.0.evaluate());
    //     println!("Minimax eval: {}", mov.1);
    // }

    // for (k, v) in x.0 { 
    //     k.print_chessboard();
    //     println!("Positions: {}", v);
    // }
    // println!("Total positions: {}", x.1);
    // println!("{:#?} \n\n", cb.perft(5));
    println!("Number of positions at depth {}: {}", DEPTH, numOfPositions.1);
    println!("Elapsed time to calculate to depth {}: {}ms", DEPTH, now.elapsed().as_millis());
    // cb.perft(5);

    // let white_move = cb.get_all_legal_white_moves();
    // println!("Move 1 #: {}", white_move.0.len());
    // for (i, mov) in white_move.1.iter().enumerate() {
    //     println!("Move 2 #: {}", mov.len());
    //     for m in mov {
    //         // m.print_chessboard();
    //     }


    //     return;
    // }

    println!("std::mem::size_of::<ChessBoard>(): {}",std::mem::size_of::<ChessBoard>());

    let mut last_positions: Vec<ChessBoard> = vec![cb];
    
    let (mut legal_moves, mut pseudo_legal_white_moves, mut pseudo_legal_black_moves): (Vec<ChessBoard>, Vec<Vec<ChessBoard>>, Vec<Vec<ChessBoard>>) = (vec![], vec![], vec![]);
    // let mut pseudo_legal_opposite_moves;
    let mut param: Option<&Vec<ChessBoard>>;
    let now = Instant::now();

    let mut first_moves: HashMap<ChessBoard, u64> = HashMap::new();
    // for ply_num in 1..6 {
    //     println!("Ply #{}", ply_num);
    //     let mut positions: Vec<ChessBoard> = vec![];
    //     println!("Positions len {}", positions.len());
    //     for (i,pos) in last_positions.iter().enumerate() {
    //         if pos.white_to_move {

    //             if pseudo_legal_white_moves.is_empty(){
    //                 param = None;
    //             } else {
    //                 param = None;
    //                 // param = Some(&pseudo_legal_white_moves[i]);
    //                 // println!("{:?}", param);
    //             }

    //             (legal_moves, pseudo_legal_opposite_moves) = pos.get_all_legal_white_moves(param);
    //             positions.extend(legal_moves);
    //             pseudo_legal_black_moves.extend(pseudo_legal_opposite_moves);
    //             pseudo_legal_white_moves.clear();
    //             // println!("Move #{}: {} possible moves", ply_num, positions.len())
    //         } else {
    //             if pseudo_legal_black_moves.is_empty(){
    //                 param = None;
    //             } else {
    //                 param = None;
    //                 // param = Some(&pseudo_legal_black_moves[i]);
    //             }
    //             (legal_moves, pseudo_legal_opposite_moves) = pos.get_all_legal_black_moves(param);
    //             positions.extend(legal_moves);
    //             pseudo_legal_white_moves.extend(pseudo_legal_opposite_moves);
    //             pseudo_legal_black_moves.clear();
    //             // opposite_pseudo_legal_positions.extend(pseudo_legal_opposite_moves);
    //             // println!("Move #{}: {} possible moves", ply_num, positions.len())  
    //         }

    //     }

    //     // if ply_num == 1 {
    //     //     for pos in &positions {
    //     //         first_moves.insert(*pos, 0);
    //     //     }
    //     // } else {
    //     //     for  in  {
                
    //     //     }
    //     // }

    //     // let mut counter = 0;
    //     // if ply_num == 5 {
    //     //     for pos in &positions {
    //     //         if pos.black_queens & 0x800000000000000 == 0 || pos.black_bishops & 0x2000000000000000 == 0 || pos.black_bishops & 0x400000000000000 == 0
    //     //         || pos.black_knights & 0x200000000000000 == 0 || pos.black_knights & 0x4000000000000000 == 0   {
    //     //             pos.print_chessboard();
    //     //         }
    //     //     }
    //     //     // println!("NUMBER OF CHECKMATES: {}", counter);
    //     // }

    //     println!("Possible positions after move #{}: {} positions", ply_num, positions.len());
    //     println!("pseudo_legal_white_moves len #{}: {}", ply_num, pseudo_legal_white_moves.len());
    //     println!("pseudo_legal_black_moves len #{}: {}", ply_num, pseudo_legal_black_moves.len());

        
    //     last_positions = positions;
    //     // positions.clear();
        
    // }
    

}
