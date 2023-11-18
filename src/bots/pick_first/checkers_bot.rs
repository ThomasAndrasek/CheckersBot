use crate::checkers;

pub const PIECE_VALUE: f64 = 1.0;
pub const KING_VALUE: f64 = 3.0;

pub fn evaluate_board(board: &checkers::Board) -> f64 {
    let mut score = 0.0;

    for piece in board.board.iter() {
        match *piece {
            checkers::BLACK => score += PIECE_VALUE,
            checkers::BLACK_KING => score += KING_VALUE,
            checkers::WHITE => score -= PIECE_VALUE,
            checkers::WHITE_KING => score -= KING_VALUE,
            _ => (),
        }
    }

    score
}

pub fn pick_move(board: &mut checkers::Board, color: u8, time_restraint_ms: u64) -> (checkers::Move, f64) {
    let moves = match color {
        checkers::BLACK => board.search_black_all(),
        checkers::WHITE => board.search_white_all(),
        _ => panic!("Invalid color"),
    };

    (moves[0].clone(), evaluate_board(board))
}
