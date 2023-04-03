use crate::chessboard::chessboard::ChessBoard;

pub fn black_bishop_move(bishop_square: u64, attacked_square: u64, curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {
    let mut new_chessboard = curr_chessboard.clone();
    new_chessboard.white_to_move = !new_chessboard.white_to_move;
    new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
    

    new_chessboard.black_bishops -= bishop_square;
    new_chessboard.black_bishops += attacked_square;

    if curr_chessboard.get_all_white_pieces() & attacked_square > 0 {
        new_chessboard.remove_white_piece(attacked_square);
    }
    
    result.push(new_chessboard);
}

pub fn black_queen_move(queen_square: u64, attacked_square: u64, curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {
    let mut new_chessboard = curr_chessboard.clone();
    new_chessboard.white_to_move = !new_chessboard.white_to_move;
    new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
    

    new_chessboard.black_queens -= queen_square;
    new_chessboard.black_queens += attacked_square;

    if curr_chessboard.get_all_white_pieces() & attacked_square > 0 {
        new_chessboard.remove_white_piece(attacked_square);
    }
    
    result.push(new_chessboard);
}

pub fn black_king_move(king_square: u64, attacked_square: u64, curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {
    let mut new_chessboard = curr_chessboard.clone();
    new_chessboard.white_to_move = !new_chessboard.white_to_move;
    new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
    

    new_chessboard.black_king -= king_square;
    new_chessboard.black_king += attacked_square;
    new_chessboard.black_moved_king = true;

    if curr_chessboard.get_all_white_pieces() & attacked_square > 0 {
        new_chessboard.remove_white_piece(attacked_square);
    }
    
    result.push(new_chessboard);
}

pub fn are_black_short_castling_squares_under_attack(chessboard: &mut ChessBoard) -> bool {
    let king_position = chessboard.black_king;
    let (e8, f8, g8) = (0x1000000000000000, 0x2000000000000000, 0x4000000000000000);
    let (e8_attacked, f8_attacked, g8_attacked);

    chessboard.black_king = e8;
    e8_attacked = chessboard.is_black_king_checked().0;
    chessboard.black_king = f8;
    f8_attacked = chessboard.is_black_king_checked().0;
    chessboard.black_king = g8;
    g8_attacked = chessboard.is_black_king_checked().0;

    chessboard.black_king = king_position;

    if e8_attacked || f8_attacked || g8_attacked {
        return true;
    } else {
        return false;
    }
}

pub fn are_black_long_castling_squares_under_attack(chessboard: &mut ChessBoard) -> bool {
    let king_position = chessboard.black_king;
    let (c8, d8, e8) = (0x400000000000000, 0x800000000000000, 0x1000000000000000);
    let (c8_attacked, d8_attacked, e8_attacked);
    
    chessboard.black_king = e8;
    e8_attacked = chessboard.is_black_king_checked().0;
    chessboard.black_king = d8;
    d8_attacked = chessboard.is_black_king_checked().0;
    chessboard.black_king = c8;
    c8_attacked = chessboard.is_black_king_checked().0;

    chessboard.black_king = king_position;

    if e8_attacked || d8_attacked || c8_attacked {
        return true;
    } else {
        return false;
    }
}

pub fn black_short_castle(curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {

    let (f8, g8, h8) = (0x2000000000000000, 0x4000000000000000, 0x8000000000000000);

    let mut new_chessboard = curr_chessboard.clone();
    new_chessboard.white_to_move = !new_chessboard.white_to_move;
    new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
    
    new_chessboard.black_king = g8;
    new_chessboard.black_moved_king = true;
    new_chessboard.black_moved_H_rook = true;
    new_chessboard.black_rooks -= h8;
    new_chessboard.black_rooks += f8;
    
    result.push(new_chessboard);
}

pub fn black_long_castle(curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {

    let (a8, c8, d8) = (0x1000000000000, 0x4000000000000, 0x8000000000000);

    let mut new_chessboard = curr_chessboard.clone();
    new_chessboard.white_to_move = !new_chessboard.white_to_move;
    new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
    

    new_chessboard.black_king = c8;
    new_chessboard.black_moved_king = true;
    new_chessboard.black_moved_A_rook = true;
    new_chessboard.black_rooks -= a8;
    new_chessboard.black_rooks += d8;
    
    result.push(new_chessboard);
}

pub fn black_en_passant_move(pawn_square: u64, white_pawn_square: u64, curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {
    let mut new_chessboard = curr_chessboard.clone();
    new_chessboard.white_to_move = !new_chessboard.white_to_move;
    new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
    
    new_chessboard.black_pawns -= pawn_square;
    new_chessboard.black_pawns += white_pawn_square >> 8 ;

    new_chessboard.white_pawns -= white_pawn_square;
    
    result.push(new_chessboard);
}