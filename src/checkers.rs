use std::collections::HashSet;

pub const WHITE: u8 = 0b0001;
pub const BLACK: u8 = 0b0010;
pub const KING: u8 = 0b0100;
pub const WHITE_KING: u8 = WHITE | KING;
pub const BLACK_KING: u8 = BLACK | KING;
pub const EMPTY: u8 = 0;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Board {
    pub board: Vec<u8>,
    black_pieces_bitboard: u128,
    white_pieces_bitboard: u128,
    black_kings_bitboard: u128,
    white_kings_bitboard: u128,
    turn: u8,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Move {
    pub jumps: Vec<u8>,
    pub captures: Vec<u8>,
    pub captures_pieces: Vec<u8>,
    pub is_king: bool,
    pub color: u8,
}

impl Move {
    pub fn new(color: u8) -> Move {
        Move {
            jumps: Vec::new(),
            captures: Vec::new(),
            captures_pieces: Vec::new(),
            is_king: false,
            color,
        }
    }

    pub fn print(&self) {
        let mut output = String::new();
        for i in 0..self.jumps.len() {
            output.push_str(&format!("{} ", self.jumps[i]));
        }
        println!("{}", output);
    }
}

impl Board {
    pub fn new() -> Board {
        let mut board = Vec::new();
        
        for i in 0..64 {
            let mut square = EMPTY;
            if i < 8 {
                if i % 2 == 1 {
                    square = WHITE;
                }
            } else if i < 16 {
                if i % 2 == 0 {
                    square = WHITE;
                }
            } else if i < 24 {
                if i % 2 == 1 {
                    square = WHITE;
                }
            } else if i < 40 {
                square = EMPTY;
            } else if i < 48 {
                if i % 2 == 0 {
                    square = BLACK;
                }
            } else if i < 56 {
                if i % 2 == 1 {
                    square = BLACK;
                }
            } else if i < 64 {
                if i % 2 == 0 {
                    square = BLACK;
                }
            } else {
                square = EMPTY;
            }
            board.push(square);
        }

        let black_pieces_bitboard: u128 = 0b000000000000000000000000000000000000000000000010101010010101010110101010;
        let white_pieces_bitboard: u128 = 0b010101011010101001010101000000000000000000000000000000000000000000000000;
        let black_kings_bitboard: u128 = 0;
        let white_kings_bitboard: u128 = 0;

        Board {
            board,
            black_pieces_bitboard,
            white_pieces_bitboard,
            black_kings_bitboard,
            white_kings_bitboard,
            turn: WHITE,
        }
    }

    pub fn print(&self) {
            // WHITE should print 'w' and 'W' for kings
            // BLACK should print 'b' and 'B' for kings
            // EMPTY should print ' '
            //                     -----------------------------
            // it should look like | b |   | b |   | b |   | b |
            //                     -----------------------------
            //                     |   | b |   | b |   | b |   |
            //                     -----------------------------
            //                     | b |   | b |   | b |   | b |
            //                     -----------------------------
            //                     |   |   |   |   |   |   |   |
            //                     -----------------------------
            //                     |   |   |   |   |   |   |   |
            //                     -----------------------------
            //                     |   | w |   | w |   | w |   |
            //                     -----------------------------
            //                     | w |   | w |   | w |   | w |
            //                     -----------------------------
            //                     |   | w |   | w |   | w |   |
            //                     -----------------------------
    
        println!("---------------------------------");

        for i in 0..64 {
            let mut square = " ";
            if self.board[i] & WHITE != 0 {
                if self.board[i] & KING != 0 {
                    square = "W";
                } else {
                    square = "w";
                }
            } else if self.board[i] & BLACK != 0 {
                if self.board[i] & KING != 0 {
                    square = "B";
                } else {
                    square = "b";
                }
            }
            print!("| {} ", square);
            if i % 8 == 7 {
                println!("|");
            }

            if i % 8 == 7 {
                let temp = i - 7;
                for j in 0..8 {
                    if temp + j >= 10 {
                        print!("|-{}", temp + j);
                    } else {
                        print!("|-{}-", temp + j);
                    }
                }
                println!("|");
            }
        }
    }

    pub fn search_attack_black(&mut self, from: u8, current_move: Move, moves: &mut HashSet<Move>) {
        if from > 15 {         
            if from % 8 > 1 && self.board[from as usize - 9] & WHITE != 0 && self.board[from as usize - 18] == EMPTY {
                let mut new_move = current_move.clone();
                let to = from - 18;
                new_move.jumps.push(to);
                new_move.captures.push(from - 9);
                let captured = self.board[from as usize - 9];
                new_move.captures_pieces.push(captured);
                self.board[from as usize - 9] = EMPTY;
                self.board[from as usize - 18] = self.board[from as usize];

                let mut kinged = false;
                if to < 8 {
                    if self.board[from as usize - 18] & KING == 0 {
                        self.board[from as usize - 18] |= KING;
                        new_move.is_king = true;
                        kinged = true;
                    }
                }

                self.board[from as usize] = EMPTY;

                moves.insert(new_move.clone());
                self.search_attack_black(to, new_move, moves);

                self.board[from as usize] = self.board[from as usize - 18];

                if kinged { 
                    self.board[from as usize] &= !KING;
                }

                self.board[from as usize - 18] = EMPTY;
                self.board[from as usize - 9] = captured;
            }
            if from % 8 < 6 && self.board[from as usize - 7] & WHITE != 0 && self.board[from as usize - 14] == EMPTY {
                let mut new_move = current_move.clone();
                let to = from - 14;
                new_move.jumps.push(to);
                new_move.captures.push(from - 7);
                let captured = self.board[from as usize - 7];
                new_move.captures_pieces.push(captured);
                self.board[from as usize - 7] = EMPTY;
                self.board[from as usize - 14] = self.board[from as usize];

                let mut kinged = false;
                if to < 8 {
                    if self.board[from as usize - 14] & KING == 0 {
                        self.board[from as usize - 14] |= KING;
                        new_move.is_king = true;
                        kinged = true;
                    }
                }

                self.board[from as usize] = EMPTY;

                moves.insert(new_move.clone());
                self.search_attack_black(to, new_move, moves);

                self.board[from as usize] = self.board[from as usize - 14];

                if kinged {
                    self.board[from as usize] &= !KING;
                }

                self.board[from as usize - 14] = EMPTY;
                self.board[from as usize - 7] = captured;
            } 
        }

        if self.board[from as usize] & KING != 0 && from < 48 {
            if from % 8 > 1 && self.board[from as usize + 7] & WHITE != 0 && self.board[from as usize + 14] == EMPTY {
                let mut new_move = current_move.clone();
                let to = from + 14;
                new_move.jumps.push(to);
                new_move.captures.push(from + 7);
                let captured = self.board[from as usize + 7];
                new_move.captures_pieces.push(captured);
                self.board[from as usize + 7] = EMPTY;
                self.board[from as usize + 14] = self.board[from as usize];

                self.board[from as usize] = EMPTY;

                moves.insert(new_move.clone());
                self.search_attack_black(to, new_move, moves);

                self.board[from as usize] = self.board[from as usize + 14];

                self.board[from as usize + 14] = EMPTY;
                self.board[from as usize + 7] = captured;
            }
            if from % 8 < 6 && self.board[from as usize + 9] & WHITE != 0 && self.board[from as usize + 18] == EMPTY {
                let mut new_move = current_move.clone();
                let to = from + 18;
                new_move.jumps.push(to);
                new_move.captures.push(from + 9);
                let captured = self.board[from as usize + 9];
                new_move.captures_pieces.push(captured);
                self.board[from as usize + 9] = EMPTY;
                self.board[from as usize + 18] = self.board[from as usize];

                self.board[from as usize] = EMPTY;

                moves.insert(new_move.clone());
                self.search_attack_black(to, new_move, moves);

                self.board[from as usize] = self.board[from as usize + 18];

                self.board[from as usize + 18] = EMPTY;
                self.board[from as usize + 9] = captured;
            }
        }
    }

    pub fn search_attack_white(&mut self, from: u8, current_move: Move, moves: &mut HashSet<Move>) {
        if from < 48 {
            if from % 8 < 6 && self.board[from as usize + 9] & BLACK != 0 && self.board[from as usize + 18] == EMPTY {
                let mut new_move = current_move.clone();
                let to = from + 18;
                new_move.jumps.push(to);
                new_move.captures.push(from + 9);
                let captured = self.board[from as usize + 9];
                new_move.captures_pieces.push(captured);
                self.board[from as usize + 9] = EMPTY;
                self.board[from as usize + 18] = self.board[from as usize];

                let mut kinged = false;
                if to > 55 {
                    if self.board[from as usize + 18] & KING == 0 {
                        self.board[from as usize + 18] |= KING;
                        new_move.is_king = true;
                        kinged = true;
                    }
                }

                self.board[from as usize] = EMPTY;

                moves.insert(new_move.clone());
                self.search_attack_white(to, new_move, moves);

                self.board[from as usize] = self.board[from as usize + 18];

                if kinged {
                    self.board[from as usize] &= !KING;
                }

                self.board[from as usize + 18] = EMPTY;
                self.board[from as usize + 9] = captured;

            }
            if from % 8 > 1 && self.board[from as usize + 7] & BLACK != 0 && self.board[from as usize + 14] == EMPTY {
                let mut new_move = current_move.clone();
                let to = from + 14;
                new_move.jumps.push(to);
                new_move.captures.push(from + 7);
                let captured = self.board[from as usize + 7];
                new_move.captures_pieces.push(captured);
                self.board[from as usize + 7] = EMPTY;
                self.board[from as usize + 14] = self.board[from as usize];

                let mut kinged = false;
                if to > 55 {
                    if self.board[from as usize + 14] & KING == 0 {
                        self.board[from as usize + 14] |= KING;
                        new_move.is_king = true;
                        kinged = true;
                    }
                }

                self.board[from as usize] = EMPTY;

                moves.insert(new_move.clone());
                self.search_attack_white(to, new_move, moves);

                self.board[from as usize] = self.board[from as usize + 14];

                if kinged {
                    self.board[from as usize] &= !KING;
                }

                self.board[from as usize + 14] = EMPTY;
                self.board[from as usize + 7] = captured;
            }
        }

        if self.board[from as usize] & KING != 0 && from > 15 {
            if from % 8 < 6 && self.board[from as usize - 7] & BLACK != 0 && self.board[from as usize - 14] == EMPTY {
                let mut new_move = current_move.clone();
                let to = from - 14;
                new_move.jumps.push(to);
                new_move.captures.push(from - 7);
                let captured = self.board[from as usize - 7];
                new_move.captures_pieces.push(captured);
                self.board[from as usize - 7] = EMPTY;
                self.board[from as usize - 14] = self.board[from as usize];

                self.board[from as usize] = EMPTY;

                moves.insert(new_move.clone());
                self.search_attack_white(to, new_move, moves);

                self.board[from as usize] = self.board[from as usize - 14];

                if to > 55 {
                    self.board[from as usize] &= !KING;
                }

                self.board[from as usize - 14] = EMPTY;
                self.board[from as usize - 7] = captured;
            }
            if from % 8 > 1 && self.board[from as usize - 9] & BLACK != 0 && self.board[from as usize - 18] == EMPTY {
                let mut new_move = current_move.clone();
                let to = from - 18;
                new_move.jumps.push(to);
                new_move.captures.push(from - 9);
                let captured = self.board[from as usize - 9];
                new_move.captures_pieces.push(captured);
                self.board[from as usize - 9] = EMPTY;
                self.board[from as usize - 18] = self.board[from as usize];

                self.board[from as usize] = EMPTY;

                moves.insert(new_move.clone());
                self.search_attack_white(to, new_move, moves);

                self.board[from as usize] = self.board[from as usize - 18];

                if to > 55 {
                    self.board[from as usize] &= !KING;
                }

                self.board[from as usize - 18] = EMPTY;
                self.board[from as usize - 9] = captured;
            }
        }
    }

    pub fn search_black(&mut self, from: u8) -> Vec<Move> {
        let mut moves = HashSet::new();
        let mut current_move = Move::new(BLACK);
        current_move.jumps.push(from);

        if from > 7 && from % 8 > 0 {
            if self.board[from as usize - 9] == EMPTY {
                let mut new_move = current_move.clone();
                new_move.jumps.push(from - 9);
                if from < 16 && self.board[from as usize] & KING == 0 {
                    new_move.is_king = true;
                }
                moves.insert(new_move);
            }
            else if self.board[from as usize - 9] & WHITE != 0 && from % 8 > 1 {
                self.search_attack_black(from, current_move.clone(), &mut moves);
            }
        }

        if from > 7 && from % 8 < 7 {
            if self.board[from as usize - 7] == EMPTY {
                let mut new_move = current_move.clone();
                new_move.jumps.push(from - 7);
                if from < 16 && self.board[from as usize] & KING == 0 {
                    new_move.is_king = true;
                }
                moves.insert(new_move);
            }
            else if self.board[from as usize - 7] & WHITE != 0 && from % 8 < 6 {
                self.search_attack_black(from, current_move.clone(), &mut moves);
            }
        }

        if self.board[from as usize] & KING != 0 {
            if from < 56 && from % 8 > 0 {
                if self.board[from as usize + 7] == EMPTY {
                    let mut new_move = current_move.clone();
                    new_move.jumps.push(from + 7);
                    moves.insert(new_move);
                }
                else if self.board[from as usize + 7] & WHITE != 0 && from % 8 > 1 {
                    self.search_attack_black(from, current_move.clone(), &mut moves);
                }
            }

            if from < 56 && from % 8 < 7 {
                if self.board[from as usize + 9] == EMPTY {
                    let mut new_move = current_move.clone();
                    new_move.jumps.push(from + 9);
                    moves.insert(new_move);
                }
                else if self.board[from as usize + 9] & WHITE != 0 && from % 8 < 6 {
                    self.search_attack_black(from, current_move.clone(), &mut moves);
                }
            }
        }

        let mut result = Vec::new();

        let mut moves_with_attacks = Vec::new();
        for m in moves.iter() {
            if m.captures.len() > 0 {
                moves_with_attacks.push(m.clone());
            }
        }

        if moves_with_attacks.len() > 0 {
            for m in moves_with_attacks {
                result.push(m);
            }

            let mut max_captures = 0;
            for m in &result {
                if m.captures.len() > max_captures {
                    max_captures = m.captures.len();
                }
            }

            let mut i = 0;
            while i < result.len() {
                if result[i].captures.len() < max_captures {
                    result.remove(i);
                }
                else {
                    i += 1;
                }
            }
        }
        else {
            for m in moves {
                result.push(m);
            }
        }

        result
    }

    pub fn search_white(&mut self, from: u8) -> Vec<Move> {
        let mut moves = HashSet::new();
        let mut current_move = Move::new(WHITE);
        current_move.jumps.push(from);

        if from < 56 && from % 8 > 0 {
            if self.board[from as usize + 7] == EMPTY {
                let mut new_move = current_move.clone();
                new_move.jumps.push(from + 7);
                if from > 47 && self.board[from as usize] & KING == 0 {
                    new_move.is_king = true;
                }
                moves.insert(new_move);
            }
            else if self.board[from as usize + 7] & BLACK != 0 && from % 8 > 1 {
                self.search_attack_white(from, current_move.clone(), &mut moves);
            }
        }

        if from < 56 && from % 8 < 7 {
            if self.board[from as usize + 9] == EMPTY {
                let mut new_move = current_move.clone();
                new_move.jumps.push(from + 9);
                if from > 47 && self.board[from as usize] & KING == 0 {
                    new_move.is_king = true;
                }
                moves.insert(new_move);
            }
            else if self.board[from as usize + 9] & BLACK != 0 && from % 8 < 6 {
                self.search_attack_white(from, current_move.clone(), &mut moves);
            }
        }

        if self.board[from as usize] & KING != 0 {
            if from > 7 && from % 8 > 0 {
                if self.board[from as usize - 9] == EMPTY {
                    let mut new_move = current_move.clone();
                    new_move.jumps.push(from - 9);
                    moves.insert(new_move);
                }
                else if self.board[from as usize - 9] & BLACK != 0 && from % 8 > 1 {
                    self.search_attack_white(from, current_move.clone(), &mut moves);
                }
            }

            if from > 7 && from % 8 < 7 {
                if self.board[from as usize - 7] == EMPTY {
                    let mut new_move = current_move.clone();
                    new_move.jumps.push(from - 7);
                    moves.insert(new_move);
                }
                else if self.board[from as usize - 7] & BLACK != 0 && from % 8 < 6 {
                    self.search_attack_white(from, current_move.clone(), &mut moves);
                }
            }
        }

        let mut result = Vec::new();

        let mut moves_with_attacks = Vec::new();
        for m in moves.iter() {
            if m.captures.len() > 0 {
                moves_with_attacks.push(m.clone());
            }
        }

        if moves_with_attacks.len() > 0 {
            for m in moves_with_attacks {
                result.push(m);
            }

            let mut max_captures = 0;
            for m in &result {
                if m.captures.len() > max_captures {
                    max_captures = m.captures.len();
                }
            }

            let mut i = 0;
            while i < result.len() {
                if result[i].captures.len() < max_captures {
                    result.remove(i);
                }
                else {
                    i += 1;
                }
            }
        }
        else {
            for m in moves {
                result.push(m);
            }
        }

        result
    }

    pub fn search_black_all(&mut self) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 0..64 {
            if self.board[i] & BLACK != 0 {
                let mut m = self.search_black(i as u8);
                moves.append(&mut m);
            }
        }

        let mut result = Vec::new();
        // if there are moves with captures, only keep those
        let mut captures = false;
        for m in &moves {
            if m.captures.len() > 0 {
                captures = true;
                break;
            }
        }

        if captures {
            for m in moves {
                if m.captures.len() > 0 {
                    result.push(m);
                }
            }
        }
        else {
            for m in moves {
                result.push(m);
            }
        } 

        result
    }

    pub fn search_white_all(&mut self) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 0..64 {
            if self.board[i] & WHITE != 0 {
                let mut m = self.search_white(i as u8);
                moves.append(&mut m);
            }
        }

        let mut result = Vec::new();
        // if there are moves with captures, only keep those
        let mut captures = false;
        for m in &moves {
            if m.captures.len() > 0 {
                captures = true;
                break;
            }
        }

        if captures {
            for m in moves {
                if m.captures.len() > 0 {
                    result.push(m);
                }
            }
        }
        else {
            for m in moves {
                result.push(m);
            }
        } 

        result
    }

    pub fn make_move(&mut self, move_to_make: &Move) {
        let mut from = move_to_make.jumps[0];
        for i in 1..move_to_make.jumps.len() {
            let to = move_to_make.jumps[i];
            self.board[to as usize] = self.board[from as usize];
            self.board[from as usize] = EMPTY;
            from = to;
        }

        for i in 0..move_to_make.captures.len() {
            self.board[move_to_make.captures[i] as usize] = EMPTY;
        }

        if move_to_make.is_king {
            self.board[from as usize] = self.board[from as usize] | KING;
        }
    }

    pub fn undo_move(&mut self, move_to_undo: &Move) {
        let mut from = move_to_undo.jumps[move_to_undo.jumps.len() - 1];
        for i in (1..move_to_undo.jumps.len()).rev() {
            let to = move_to_undo.jumps[i - 1];
            self.board[to as usize] = self.board[from as usize];
            self.board[from as usize] = EMPTY;
            from = to;
        }

        for i in 0..move_to_undo.captures.len() {
            self.board[move_to_undo.captures[i] as usize] = move_to_undo.captures_pieces[i];
        }

        if move_to_undo.is_king {
            self.board[from as usize] = self.board[from as usize] & !KING;
        }
    }
}
