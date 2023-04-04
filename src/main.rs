#![allow(non_snake_case)]

mod chessboard;
mod white_utils;
mod black_utils;
use std::{vec, env, time::Instant};

use crate::chessboard::chessboard::ChessBoard;

fn main() {
    let cb = ChessBoard::new();
    cb.print_chessboard();

    // let white_move = cb.get_all_legal_white_moves();
    // println!("Move 1 #: {}", white_move.0.len());
    // for (i, mov) in white_move.1.iter().enumerate() {
    //     println!("Move 2 #: {}", mov.len());
    //     for m in mov {
    //         // m.print_chessboard();
    //     }


    //     return;
    // }

    // println!("SIZE: {}",std::mem::size_of::<ChessBoard>());

    let mut last_positions: Vec<ChessBoard> = vec![cb];
    env::set_var("RUST_BACKTRACE", "full");
    let (mut legal_moves, mut pseudo_legal_white_moves, mut pseudo_legal_black_moves): (Vec<ChessBoard>, Vec<Vec<ChessBoard>>, Vec<Vec<ChessBoard>>) = (vec![], vec![], vec![]);
    let mut pseudo_legal_opposite_moves;
    let mut param: Option<&Vec<ChessBoard>>;
    let now = Instant::now();

    
    for ply_num in 1..6 {
        let mut positions: Vec<ChessBoard> = vec![];
        for (i,pos) in last_positions.iter().enumerate() {
            if pos.white_to_move {

                if pseudo_legal_white_moves.is_empty(){
                    param = None;
                } else {
                    param = Some(&pseudo_legal_white_moves[i]);
                    // println!("{:?}", param);
                }

                (legal_moves, pseudo_legal_opposite_moves) = pos.get_all_legal_white_moves(param);
                positions.extend(legal_moves);
                pseudo_legal_black_moves.extend(pseudo_legal_opposite_moves);
                pseudo_legal_white_moves.clear();
                // println!("Move #{}: {} possible moves", ply_num, positions.len())
            } else {
                if pseudo_legal_black_moves.is_empty(){
                    param = None;
                } else {
                    param = Some(&pseudo_legal_black_moves[i]);
                }
                (legal_moves, pseudo_legal_opposite_moves) = pos.get_all_legal_black_moves(param);
                positions.extend(legal_moves);
                pseudo_legal_white_moves.extend(pseudo_legal_opposite_moves);
                pseudo_legal_black_moves.clear();
                // opposite_pseudo_legal_positions.extend(pseudo_legal_opposite_moves);
                // println!("Move #{}: {} possible moves", ply_num, positions.len())  
            }
        }
        
        // if ply_num == 3 {
        //     for pos in &positions {
        //         pos.print_chessboard();
        //     }
        // }

        println!("Possible positions after move #{}: {} positions", ply_num, positions.len());
        println!("pseudo_legal_white_moves len #{}: {}", ply_num, pseudo_legal_white_moves.len());
        println!("pseudo_legal_black_moves len #{}: {}", ply_num, pseudo_legal_black_moves.len());

        
        last_positions = positions;
        // positions.clear();
        
    }

    let elapsed = now.elapsed();
    println!("time for 5 ply {}", elapsed.as_secs());
    
    // let mut total_positions = 0;

    // positions = cb.get_all_legal_white_moves().0;
    // total_positions = positions.len();
    // println!("Possible positions after move #1: {} positions", total_positions);
    // total_positions = 0;

    // let mut next_positions = vec![];

    // // positions[16].print_chessboard();
    // // total_positions = positions[16].get_all_legal_black_moves().0.len();

    // for mut pos in positions {
    //     // println!("AAA ");
    //     next_positions.extend(pos.get_all_legal_black_moves().0);
    //     total_positions += next_positions.len();
    // }
    // println!("Possible positions after move #2: {} positions",  next_positions.len());

    // // let mut next_positions: Vec<ChessBoard> = vec![];
    // let mut positions: Vec<ChessBoard> = vec![];
    // // total_positions = 0;
    // for mut pos in next_positions {
    //     positions.extend(pos.get_all_legal_white_moves().0);
    //     // total_positions += positions.len();
    // }
    // println!("Possible positions after move #3: {} positions",  positions.len());

    // let mut next_positions: Vec<ChessBoard> = vec![];
    // // total_positions = 0;
    // for mut pos in positions {
    //     next_positions.extend(pos.get_all_legal_black_moves().0);
    //     // total_positions += positions.len()
    // }
    // println!("Possible positions after move #4: {} positions",  next_positions.len());

}
