use crate::black_utils::get_all_attacked_squares_by_black;
// use crate::chessboard;
use crate::chessboard::chessboard::ChessBoard;
use crate::chessboard::chessboard::Constants;
use crate::chessboard::chessboard::OverflowingLeftShift;

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

pub fn get_all_attacked_squares_by_white(cb: &ChessBoard) -> (u64, Vec<ChessBoard>) {
    let mut attacked_squares: u64 = 0;

    let mut pseudo_legal_white_moves: Vec<ChessBoard> = vec![];


    pseudo_legal_white_moves.append(&mut cb.get_all_pseudo_legal_white_rook_moves());
    pseudo_legal_white_moves.append(&mut cb.get_all_pseudo_legal_white_knight_moves());
    pseudo_legal_white_moves.append(&mut cb.get_all_pseudo_legal_white_bishop_moves());
    pseudo_legal_white_moves.append(&mut cb.get_all_pseudo_legal_white_queen_moves());
    pseudo_legal_white_moves.append(&mut cb.get_all_pseudo_legal_white_king_moves());
    
    for pos in &pseudo_legal_white_moves {
        attacked_squares = attacked_squares | pos.get_all_white_pieces();
    }

    pseudo_legal_white_moves.append(&mut cb.get_all_pseudo_legal_white_pawn_moves());

    // add attacked squares by pawn

    let mut square: u64;
    // 64 - 8 = 56 // cant occupy 1st rank
    for i in 8..56 {
        square = 1 << i;
        // square =  2_u64.pow(i);
        // check if pawn occupies square
        if cb.white_pawns & square > 0 {

            // CHECK FOR UPRIGHT TAKE ( attacked_square = square << 9 )
            // let attacked_square = square * 2_u64.pow(9);
            let (attacked_square, overflow) = square.overflowing_loss_checked_shl(9);
            // cant be on A file after taking upright
            if !overflow && attacked_square & Constants::A_FILE == 0 {
                attacked_squares = attacked_squares | attacked_square;
            }

            // CHECK FOR UPLEFT TAKE ( attacked_square = square << 7 )
            let attacked_square = square << 7;
            // cant be on H file after taking upleft
            if attacked_square & Constants::H_FILE == 0 {
                attacked_squares = attacked_squares | attacked_square;
            }
        }
    }

    return (attacked_squares, pseudo_legal_white_moves);
}

pub fn are_white_short_castling_squares_under_attack(chessboard: &ChessBoard) -> bool {

    let (e1, f1, g1) = (16, 32, 64);
    let (e1_attacked, f1_attacked, g1_attacked);


    e1_attacked = get_all_attacked_squares_by_black(chessboard).0 & e1 > 0;

    f1_attacked = get_all_attacked_squares_by_black(chessboard).0 & f1 > 0;

    g1_attacked = get_all_attacked_squares_by_black(chessboard).0 & g1 > 0;


    

    if e1_attacked || f1_attacked || g1_attacked {
        return true;
    } else {
        return false;
    }


    

}

pub fn are_white_long_castling_squares_under_attack(chessboard: &ChessBoard) -> bool {
    // let king_position = chessboard.white_king;
    let (c1, d1, e1) = (4, 8, 16);
    let (c1_attacked, d1_attacked, e1_attacked);

    
    e1_attacked = get_all_attacked_squares_by_black(chessboard).0 & e1 > 0;

    d1_attacked = get_all_attacked_squares_by_black(chessboard).0 & d1 > 0;

    c1_attacked = get_all_attacked_squares_by_black(chessboard).0 & c1 > 0;



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