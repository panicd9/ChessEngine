use crate::{chessboard::chessboard::{ChessBoard, Constants}, white_utils::get_all_attacked_squares_by_white};

pub fn black_bishop_move(bishop_square: u64, attacked_square: u64, curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) -> bool {

    if curr_chessboard.get_all_black_pieces() & attacked_square > 0 {
        return true;
    }

    let mut hit_enemy_piece: bool = false;
    let mut new_chessboard = curr_chessboard.clone();
    new_chessboard.white_to_move = !new_chessboard.white_to_move;
    new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
    

    new_chessboard.black_bishops -= bishop_square;
    new_chessboard.black_bishops += attacked_square;

    if curr_chessboard.get_all_white_pieces() & attacked_square > 0 {
        new_chessboard.remove_white_piece(attacked_square);
        hit_enemy_piece = true;
    }
    
    result.push(new_chessboard);
    return hit_enemy_piece;
}

pub fn black_queen_move(queen_square: u64, attacked_square: u64, curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) -> bool {
    
    if curr_chessboard.get_all_black_pieces() & attacked_square > 0 {
        return true;
    }

    let mut hit_enemy_piece: bool = false;
    let mut new_chessboard = curr_chessboard.clone();
    
    new_chessboard.white_to_move = !new_chessboard.white_to_move;
    new_chessboard.prev_pos_pawns = new_chessboard.black_pawns;
    

    new_chessboard.black_queens -= queen_square;
    new_chessboard.black_queens += attacked_square;

    if curr_chessboard.get_all_white_pieces() & attacked_square > 0 {
        new_chessboard.remove_white_piece(attacked_square);
        hit_enemy_piece = true;
    }
    
    result.push(new_chessboard);
    return hit_enemy_piece;
}

pub fn black_king_move(king_square: u64, attacked_square: u64, curr_chessboard: &ChessBoard, result: &mut Vec<ChessBoard>) {
    // if get_all_attacked_squares_by_white(curr_chessboard).0 & attacked_square > 0 {
    //     return;
    // }

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

pub fn get_all_attacked_squares_by_black(cb: &ChessBoard) -> (u64, Vec<ChessBoard>) {
    let mut attacked_squares: u64 = 0;

    let mut pseudo_legal_black_moves: Vec<ChessBoard> = vec![];


    pseudo_legal_black_moves.append(&mut cb.get_all_pseudo_legal_black_rook_moves());
    pseudo_legal_black_moves.append(&mut cb.get_all_pseudo_legal_black_knight_moves());
    pseudo_legal_black_moves.append(&mut cb.get_all_pseudo_legal_black_bishop_moves());
    pseudo_legal_black_moves.append(&mut cb.get_all_pseudo_legal_black_queen_moves());
    pseudo_legal_black_moves.append(&mut cb.get_all_pseudo_legal_black_king_moves());
    
    for pos in &pseudo_legal_black_moves {
        attacked_squares = attacked_squares | pos.get_all_black_pieces();
    }

    pseudo_legal_black_moves.append(&mut cb.get_all_pseudo_legal_black_pawn_moves());

    // add attacked squares by pawn

    let mut square: u64;
    // 64 - 8 = 56 // cant occupy 1st rank
    for i in 8..56 {
        square = 1 << i;
        // square =  2_u64.pow(i);
        // check if pawn occupies square
        if cb.black_pawns & square > 0 {

            // CHECK FOR DOWNRIGHT TAKE)
            let attacked_square = square >> 7;
            // cant be on A file after taking DOWNRIGHT
            if attacked_square & Constants::A_FILE == 0 {
                attacked_squares = attacked_squares | attacked_square;
            }

            // CHECK FOR DOWNLEFT TAKE
            let attacked_square = square >> 9;
            // cant be on H file after taking DOWNLEFT
            if attacked_square > 0 && attacked_square & Constants::H_FILE == 0 {
                //check if there is enemy piece and take
                attacked_squares = attacked_squares | attacked_square;
            }
        }
    }

    return (attacked_squares, pseudo_legal_black_moves);
}

pub fn are_black_short_castling_squares_under_attack(chessboard: &ChessBoard) -> bool {

    let (e8, f8, g8) = (0x1000000000000000, 0x2000000000000000, 0x4000000000000000);
    let (e8_attacked, f8_attacked, g8_attacked);

    let all_attacked_squares_by_white = get_all_attacked_squares_by_white(chessboard).0;

    e8_attacked = all_attacked_squares_by_white & e8 > 0;

    f8_attacked = all_attacked_squares_by_white & f8 > 0;

    g8_attacked = all_attacked_squares_by_white & g8 > 0;


    if e8_attacked || f8_attacked || g8_attacked {
        return true;
    } else {
        return false;
    }
}

pub fn are_black_long_castling_squares_under_attack(chessboard: &ChessBoard) -> bool {
    let (c8, d8, e8) = (0x400000000000000, 0x800000000000000, 0x1000000000000000);
    let (c8_attacked, d8_attacked, e8_attacked);
    
    let all_attacked_squares_by_white = get_all_attacked_squares_by_white(chessboard).0;

    e8_attacked = all_attacked_squares_by_white & e8 > 0;

    d8_attacked = all_attacked_squares_by_white & d8 > 0;

    c8_attacked = all_attacked_squares_by_white & c8 > 0;


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