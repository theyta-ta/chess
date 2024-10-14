use crate::board::Board;
use crate::piece::{self, Colour};
use std::fmt;
use std::io::{self, Write};

pub struct Game {
    board: Board,
    curr_player: Colour,
    mate: bool,
    curr_move: u32,
    castles: [bool; 4],
}

type Pair = (usize, usize);

impl std::default::Default for Game {
    fn default() -> Self {
        Self {
            board: Board::default(),
            curr_player: Colour::default(),
            mate: false,
            curr_move: 1,
            castles: [true; 4],
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}

impl Game {
    fn in_bounds(pos: Pair) -> bool {
        pos.0 < 8 && pos.1 < 8
    }

    // panics if either pos1 or pos2 are not in bounds
    fn diff_colour(&self, pos1: Pair, pos2: Pair) -> bool {
        self.board[pos1].map(|p| p.colour()) != self.board[pos2].map(|p| p.colour())
    }

    // all of these check_/* snip */_moves assume the move is valid according to the piece there.
    // i.e. there is no checking that from_pos -> to_pos is a valid straght move in
    // check_straight_move
    fn check_straight_move(&self, from_pos: Pair, to_pos: Pair) -> bool {
        if !Self::in_bounds(from_pos) || !Self::in_bounds(to_pos) {
            return false;
        }
        // it is fine that it may check the "wrong way round", as either way it checks the path is
        // clear
        let ret = if from_pos.0 == to_pos.0 {
            let min = from_pos.1.min(to_pos.1);
            let max = from_pos.1.max(to_pos.1);

            self.board[to_pos.0][(min + 1)..max]
                .iter()
                .any(|p| p.is_some())
        } else {
            let min = from_pos.0.min(to_pos.0);
            let max = from_pos.0.max(to_pos.0);

            ((min + 1)..max)
                .map(|i| self.board[i][to_pos.1])
                .any(|p| p.is_some())
        };
        if !ret {
            false
        } else {
            self.diff_colour(from_pos, to_pos)
        }
    }

    fn check_diag_move(&self, from_pos: Pair, to_pos: Pair) -> bool {
        if !Self::in_bounds(from_pos) || !Self::in_bounds(to_pos) {
            return false;
        }
        let (min, max) = if from_pos.0 < to_pos.0 {
            (from_pos, to_pos)
        } else {
            (to_pos, from_pos)
        };
        if (from_pos.0 < to_pos.0) == (from_pos.1 < to_pos.1) {
            for i in 1..8 {
                if min.0 + i == max.0 {
                    break;
                } else if self.board[min.0 + i][min.1 + i].is_some() {
                    return false;
                }
            }
        } else {
            for i in 1..8 {
                if min.0 + i == max.0 {
                    break;
                } else if self.board[min.0 + i][min.1 - i].is_some() {
                    return false;
                }
            }
        }

        self.diff_colour(from_pos, to_pos)
    }

    fn check_knight_move(&self, from_pos: Pair, to_pos: Pair) -> bool {
        self.diff_colour(from_pos, to_pos)
    }

    fn square_attacked(&self, square: Pair, colour: Colour) -> bool {
        self.board
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, p)| ((i, j), p)))
            .any(|(pair, piece)| {
                piece
                    .map(|p| p.colour() != colour && self.check_move_non_castle(pair, square, p))
                    .unwrap_or(false)
            })
    }

    fn handle_castles(&mut self, from_pos: Pair, to_pos: Pair) -> bool {
        if !self.check_castles(from_pos, to_pos) {
            return false;
        }
        true
    }

    // TODO: separate into check and handle
    fn check_castles(&mut self, from_pos: Pair, to_pos: Pair) -> bool {
        let colour;
        if let Some(p) = self.board[from_pos] {
            if p.poss_piece() != piece::PossiblePieces::King {
                return false;
            }
            colour = self.board[from_pos].unwrap().colour();
        } else {
            return false;
        }

        let end: usize = match to_pos.1 {
            2 => 0,
            6 => 7,
            _ => usize::MAX,
        };

        let rook = Some(piece::Piece::new(piece::PossiblePieces::Rook, colour));

        let ret = [0, 7].contains(&from_pos.0)
            && from_pos.1 == 4
            && from_pos.0 == to_pos.0
            && end != usize::MAX
            && self.board[to_pos.0][end] == rook
            && self.check_straight_move(from_pos, to_pos);

        if ret {
            let rook_to = match end {
                0 => 2,
                7 => 5,
                _ => unreachable!(),
            };
            self.board.move_piece((to_pos.0, end), (to_pos.0, rook_to));
            if colour == Colour::White {
                self.castles[0] = false;
                self.castles[1] = false;
            } else {
                self.castles[2] = false;
                self.castles[3] = false;
            }
        }

        ret
    }

    fn update_rook_castles(&mut self, pos: Pair) -> bool {
        match pos {
            (0, 0) => {
                self.castles[0] = false;
            }
            (0, 7) => {
                self.castles[1] = false;
            }
            (7, 0) => {
                self.castles[2] = false;
            }
            (7, 7) => {
                self.castles[3] = false;
            }
            _ => {}
        }
        true
    }

    fn player(&self) -> &str {
        match self.curr_player {
            Colour::White => "player 1",
            Colour::Black => "player 2",
        }
    }

    fn check_move_non_castle(&self, from_pos: Pair, to_pos: Pair, p: piece::Piece) -> bool {
        use piece::PossiblePieces::*;
        match p.poss_piece() {
            Rook => self.check_straight_move(from_pos, to_pos),
            Pawn => {
                (self.check_straight_move(from_pos, to_pos) && self.board[to_pos].is_none())
                    || (self.check_diag_move(from_pos, to_pos) && self.board[to_pos].is_some())
            }
            King => {
                (self.check_straight_move(from_pos, to_pos)
                    || self.check_diag_move(from_pos, to_pos))
                    && from_pos.0.abs_diff(to_pos.0) <= 1
                    && from_pos.1.abs_diff(to_pos.1) <= 1
            }
            Queen => {
                self.check_straight_move(from_pos, to_pos) || self.check_diag_move(from_pos, to_pos)
            }
            Bishop => self.check_diag_move(from_pos, to_pos),
            Knight => self.check_knight_move(from_pos, to_pos),
        }
    }

    fn handle_move(&mut self, from_pos: Pair, to_pos: Pair) -> Result<(), ()> {
        let p = self.board[from_pos].ok_or(())?;
        if !p.valid_move(from_pos, to_pos) {
            Err(())
        } else if self.check_move_non_castle(from_pos, to_pos, p) {
            Ok(())
        } else if self.check_castles(from_pos, to_pos) {
            self.handle_castles(from_pos, to_pos);
            Ok(())
        } else {
            Err(())
        }
    }

    fn parse_move(turn: &str) -> Option<(Pair, Pair)> {
        let mut it = turn.split(' ');
        let (from, to): (&str, &str) = (it.next()?, it.skip(1).next()?);
        let parse_pos = |pos: &str| -> Option<Pair> {
            let pos = pos.as_bytes();
            if pos.len() != 2 {
                return None;
            }

            // chess notation is column first, arrs are row first.
            // so we have to swap.
            let p1 = (pos[1] - b'1') as usize;
            let p2 = (pos[0].to_ascii_lowercase() as u8 - b'a') as usize;

            if Self::in_bounds((p1, p2)) {
                Some((p1, p2))
            } else {
                None
            }
        };

        parse_pos(from).zip(parse_pos(to))
    }

    fn change_colour(&mut self) {
        self.curr_player.change_colour();
    }

    fn check_checks(&self) -> bool {
        false
    }

    pub fn new_game() {
        Self::default().play()
    }

    pub fn play(&mut self) {
        let mut turn: String;
        let mut check;
        let mut explanation = "(write in the form 'from - to', e.g. e2 - e4)".to_string();
        while !self.mate {
            if self.curr_move == 2 && self.curr_player == Colour::White {
                explanation = String::new();
            }
            print!(
                "\n{}\n\n{}, what is your move?{}  ",
                self.board,
                self.player(),
                explanation
            );
            let _ = io::stdout().flush().unwrap();

            let invalid_move = || {
                println!("invalid move! please enter a new move");
            };
            let (pos1, pos2) = loop {
                turn = String::new();
                io::stdin()
                    .read_line(&mut turn)
                    .expect("failed to read line");
                if let Some((pos1, pos2)) = Self::parse_move(turn.trim()) {
                    if self.handle_move(pos1, pos2).is_err() {
                        invalid_move()
                    } else {
                        //valid_move = true;
                        break (pos1, pos2);
                    }
                } else {
                    invalid_move();
                }
            };
            self.board.move_piece(pos1, pos2);
            check = self.check_checks();
            if check {
                println!("check!");
            }

            self.change_colour();
            if self.curr_player == Colour::White {
                self.curr_move += 1;
            }
        }
    }
}
