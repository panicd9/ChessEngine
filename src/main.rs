#![allow(non_snake_case)]

mod chessboard;
mod white_utils;
use chessboard::chessboard::ChessBoard;


fn main() {
    let cb = ChessBoard::new();
    cb.print_chessboard();

    let white_move = cb.get_all_legal_white_moves();
    println!("Move 1 #: {}", white_move.0.len());
    for (i, mov) in white_move.1.iter().enumerate() {
        println!("Move 2 #: {}", mov.len());
        for m in mov {
            // m.print_chessboard();
        }


        return;
    }


}
