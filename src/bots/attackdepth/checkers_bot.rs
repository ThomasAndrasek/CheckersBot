use core::time;

use crate::checkers;

pub const PIECE_VALUE: f64 = 1.0;
pub const KING_VALUE: f64 = 3.0;

pub fn evaluate_board(board: &mut checkers::Board) -> f64 {
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

fn mini(board: &mut checkers::Board, depth: u32, mut alpha: f64, mut beta: f64) -> f64 {
    if depth == 0 {
        return evaluate_board(board);
    }

    let moves = board.search_white_all();

    if moves.len() == 0 {
        return std::f64::MAX;
    }

    let mut best_score = std::f64::MAX;

    for m in moves.iter() {
        board.make_move(m);

        let attack_extension = match m.captures.len() {
            0 => 0,
            _ => 1,
        };

        let score = max(board, depth - 1 + attack_extension, alpha, beta);

        if score < best_score {
            best_score = score;
        }

        board.undo_move(m);

        if score < alpha {
            break;
        }

        if score < beta {
            beta = score;
        }
    }

    best_score
}

fn max(board: &mut checkers::Board, depth: u32, mut alpha: f64, mut beta: f64) -> f64 {
    if depth == 0 {
        return evaluate_board(board);
    }

    let moves = board.search_black_all();

    if moves.len() == 0 {
        return std::f64::MIN;
    }

    let mut best_score = std::f64::MIN;

    for m in moves.iter() {
        board.make_move(m);

        let attack_extension = match m.captures.len() {
            0 => 0,
            _ => 1,
        };

        let score = mini(board, depth - 1 + attack_extension, alpha, beta);

        if score > best_score {
            best_score = score;
        }

        board.undo_move(m);

        if score > beta {
            break;
        }

        if score > alpha {
            alpha = score;
        }
    }

    best_score
}

fn find_best_move(board: &mut checkers::Board, color: u8, depth: u32) -> (checkers::Move, f64) {
    match color {
        checkers::BLACK => {
            let moves = board.search_black_all();

            let mut best_score = std::f64::MIN;
            let mut best_move_index = 0;

            let mut index = 0;
            for m in moves.iter() {
                board.make_move(m);

                let score = mini(board, depth - 1, std::f64::MIN, std::f64::MAX);

                if score > best_score {
                    best_score = score;
                    best_move_index = index;
                }

                board.undo_move(m);

                index += 1;
            }

            (moves[best_move_index].clone(), best_score)
        }
        checkers::WHITE => {
            let moves = board.search_white_all();

            let mut best_score = std::f64::MAX;
            let mut best_move_index = 0;

            let mut index = 0;
            for m in moves.iter() {
                board.make_move(m);

                let score = max(board, depth - 1, std::f64::MIN, std::f64::MAX);

                if score < best_score {
                    best_score = score;
                    best_move_index = index;
                }

                board.undo_move(m);

                index += 1;
            }

            (moves[best_move_index].clone(), best_score)
        }
        _ => { (checkers::Move::new(0), 0.0) }
    }
}

pub fn pick_move(board: &mut checkers::Board, color: u8, time_restraint_ms: i64) -> (checkers::Move, f64) {
    let moves = match color {
        checkers::BLACK => board.search_black_all(),
        checkers::WHITE => board.search_white_all(),
        _ => panic!("Invalid color"),
    }; 

    if moves.len() == 0 {
        return (checkers::Move::new(0), 0.0);
    }

    let mut move_depth = 2;

    let start_time = chrono::Utc::now();
    let mut now = chrono::Utc::now();

    let mut best_move = find_best_move(board, color, 1);
    while (now - start_time).num_milliseconds() < time_restraint_ms {
        best_move = find_best_move(board, color, move_depth);

        move_depth += 1;
        now = chrono::Utc::now();
    }

    best_move
}
