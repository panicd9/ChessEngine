use crate::chessboard::chessboard::ChessBoard;
use crate::chessboard::chessboard::Constants;


pub fn check_white_pawn_take(pawn_square: u64, attacked_square: u64, curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {
    if attacked_square & curr_chessboard.get_all_black_pieces() > 0 {

        if (attacked_square & Constants::EIGHT_RANK) == 0 {
            let mut new_chessboard = curr_chessboard.clone();
            new_chessboard.white_to_move = !new_chessboard.white_to_move;
            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;

            new_chessboard.white_pawns -= pawn_square;
            new_chessboard.white_pawns += attacked_square;
            new_chessboard.remove_black_piece(attacked_square);
            result.push(new_chessboard);
        }
        // TODO: PROMOTE TO QUEEN 
        else {
            let mut new_chessboard = curr_chessboard.clone();
            new_chessboard.white_to_move = !new_chessboard.white_to_move;
            new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;

            new_chessboard.white_pawns -= pawn_square;
            new_chessboard.white_queens += attacked_square;
            result.push(new_chessboard);
        }
    }
}

pub fn white_pawn_forward(pawn_square: u64, forward_square: u64, curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {
    // PUSH PAWN IF NOT ON EIGHT RANK
    if (forward_square & Constants::EIGHT_RANK) == 0 {
        let mut new_chessboard = curr_chessboard.clone();
        new_chessboard.white_to_move = !new_chessboard.white_to_move;
        new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
        
        new_chessboard.white_pawns -= pawn_square;
        new_chessboard.white_pawns += forward_square;
        result.push(new_chessboard);
    }
    // TODO: PROMOTE TO QUEEN 
    else {
        let mut new_chessboard = curr_chessboard.clone();
        new_chessboard.white_to_move = !new_chessboard.white_to_move;
        new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
        
        new_chessboard.white_pawns -= pawn_square;
        new_chessboard.white_queens += forward_square;
        result.push(new_chessboard);
    }
}

pub fn white_bishop_move(bishop_square: u64, attacked_square: u64, curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {
    let mut new_chessboard = curr_chessboard.clone();
    new_chessboard.white_to_move = !new_chessboard.white_to_move;
    new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
    
    new_chessboard.white_bishops -= bishop_square;
    new_chessboard.white_bishops += attacked_square;

    if curr_chessboard.get_all_black_pieces() & attacked_square > 0 {
        new_chessboard.remove_black_piece(attacked_square);
    }
    
    result.push(new_chessboard);
}

pub fn white_queen_move(queen_square: u64, attacked_square: u64, curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {
    let mut new_chessboard = curr_chessboard.clone();
    new_chessboard.white_to_move = !new_chessboard.white_to_move;
    new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
    
    new_chessboard.white_queens -= queen_square;
    new_chessboard.white_queens += attacked_square;

    if curr_chessboard.get_all_black_pieces() & attacked_square > 0 {
        new_chessboard.remove_black_piece(attacked_square);
    }
    
    result.push(new_chessboard);
}

pub fn white_king_move(king_square: u64, attacked_square: u64, curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {
    let mut new_chessboard = curr_chessboard.clone();
    new_chessboard.white_to_move = !new_chessboard.white_to_move;
    new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
    
    new_chessboard.white_king -= king_square;
    new_chessboard.white_king += attacked_square;
    new_chessboard.white_moved_king = true;

    if curr_chessboard.get_all_black_pieces() & attacked_square > 0 {
        new_chessboard.remove_black_piece(attacked_square);
    }
    
    result.push(new_chessboard);
}

pub fn are_white_short_castling_squares_under_attack(chessboard: &mut ChessBoard) -> bool {
    let king_position = chessboard.white_king;
    let (e1, f1, g1) = (16, 32, 64);
    let (e1_attacked, f1_attacked, g1_attacked);

    chessboard.white_king = e1;
    e1_attacked = chessboard.is_white_king_checked().0;
    chessboard.white_king = f1;
    f1_attacked = chessboard.is_white_king_checked().0;
    chessboard.white_king = g1;
    g1_attacked = chessboard.is_white_king_checked().0;

    chessboard.white_king = king_position;

    if e1_attacked || f1_attacked || g1_attacked {
        return true;
    } else {
        return false;
    }
}

pub fn are_white_long_castling_squares_under_attack(chessboard: &mut ChessBoard) -> bool {
    let king_position = chessboard.white_king;
    let (c1, d1, e1) = (4, 8, 16);
    let (c1_attacked, d1_attacked, e1_attacked);

    chessboard.white_king = e1;
    e1_attacked = chessboard.is_white_king_checked().0;
    chessboard.white_king = d1;
    d1_attacked = chessboard.is_white_king_checked().0;
    chessboard.white_king = c1;
    c1_attacked = chessboard.is_white_king_checked().0;

    chessboard.white_king = king_position;

    if e1_attacked || d1_attacked || c1_attacked {
        return true;
    } else {
        return false;
    }
}

pub fn white_short_castle(curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {

    let (f1, g1, h1) = (32, 64, 128);

    let mut new_chessboard = curr_chessboard.clone();
    new_chessboard.white_to_move = !new_chessboard.white_to_move;
    new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
    
    new_chessboard.white_king = g1;
    new_chessboard.white_moved_king = true;
    new_chessboard.white_moved_H_rook = true;
    new_chessboard.white_rooks -= h1;
    new_chessboard.white_rooks += f1;
    
    result.push(new_chessboard);
}

pub fn white_long_castle(curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {

    let (a1, c1, d1) = (1, 4, 8);

    let mut new_chessboard = curr_chessboard.clone();
    new_chessboard.white_to_move = !new_chessboard.white_to_move;
    new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
    
    new_chessboard.white_king = c1;
    new_chessboard.white_moved_king = true;
    new_chessboard.white_moved_A_rook = true;
    new_chessboard.white_rooks -= a1;
    new_chessboard.white_rooks += d1;
    
    result.push(new_chessboard);
}

pub fn white_en_passant_move(pawn_square: u64, black_pawn_square: u64, curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {
    let mut new_chessboard = curr_chessboard.clone();
    new_chessboard.white_to_move = !new_chessboard.white_to_move;
    new_chessboard.prev_pos_pawns = new_chessboard.white_pawns;
    
    new_chessboard.white_pawns -= pawn_square;
    new_chessboard.white_pawns += black_pawn_square << 8 ;

    new_chessboard.black_pawns -= black_pawn_square;
    
    result.push(new_chessboard);
}