use crate::chessboard::chessboard::ChessBoard;


pub fn check_white_pawn_take(pawn_square: u64, attacked_square: u64, curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {
    if attacked_square & curr_chessboard.get_all_black_pieces() > 0 {
        let mut new_chessboard = curr_chessboard.clone();
        new_chessboard.white_pawns -= pawn_square;
        new_chessboard.white_pawns += attacked_square;
        new_chessboard.remove_black_piece(attacked_square);
        result.push(new_chessboard);
    }
}