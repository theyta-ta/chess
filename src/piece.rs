type Pair = (usize, usize);

#[derive(PartialEq, Eq, Copy, Clone, Debug, Default)]
pub enum Colour {
    Black,
    #[default]
    White,
}

impl Colour {
    pub fn change_colour(&mut self) {
        *self = match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        };
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum PossiblePieces {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Piece(PossiblePieces, Colour);

impl Piece {
    // for all the private functions we are assuming both positions are valid in (0..8)^2
    fn diag_move(from_pos: Pair, to_pos: Pair) -> bool {
        (from_pos.0 + to_pos.1) == (from_pos.1 + to_pos.0)
            || (from_pos.0 + from_pos.1 == to_pos.0 + to_pos.1)
    }

    fn rooks_move(from_pos: Pair, to_pos: Pair) -> bool {
        from_pos != to_pos && (from_pos.0 == to_pos.0 || from_pos.1 == to_pos.1)
    }

    fn knights_move(from_pos: Pair, to_pos: Pair) -> bool {
        let hori_diff = from_pos.0.abs_diff(to_pos.0);
        let vert_diff = from_pos.1.abs_diff(to_pos.1);

        (hori_diff == 1 && vert_diff == 2) || (hori_diff == 2 && vert_diff == 1)
    }

    fn pawns_move(from_pos: Pair, to_pos: Pair, colour: Colour) -> bool {
        if from_pos.1 != to_pos.1 {
            if from_pos.1.abs_diff(to_pos.1) != 1 {
                return false;
            } else if colour == Colour::Black {
                return from_pos.1 + 1 == to_pos.1;
            } else {
                return from_pos.1 == to_pos.1 + 1;
            }
        }

        if colour == Colour::Black {
            if from_pos.0 == 6 {
                to_pos.0 >= 4
            } else {
                from_pos.0 == 1 + to_pos.0
            }
        } else if from_pos.0 == 1 {
            to_pos.0 <= 3
        } else {
            return to_pos.0 == 1 + from_pos.0;
        }
    }

    fn kings_move(from_pos: Pair, to_pos: Pair) -> bool {
        from_pos != to_pos
            && ((from_pos.0.abs_diff(to_pos.0) <= 1 && from_pos.1.abs_diff(to_pos.1) <= 1)
                || ([0, 7].contains(&from_pos.0)
                    && from_pos.1 == 4
                    && from_pos.0 == to_pos.0
                    && from_pos.1.abs_diff(to_pos.1) == 2))
    }

    pub fn new(p: PossiblePieces, c: Colour) -> Self {
        Self(p, c)
    }

    // not checking for pieces in the way
    // if either position is invalid then just return false.
    pub fn valid_move(&self, from_pos: Pair, to_pos: Pair) -> bool {
        {
            let valid_pos = |pos: Pair| -> bool { pos.0 < 8 && pos.1 < 8 };
            if !valid_pos(from_pos) || !valid_pos(to_pos) || from_pos == to_pos {
                return false;
            }
        }
        use PossiblePieces::*;
        match self.poss_piece() {
            Bishop => Self::diag_move(from_pos, to_pos),
            Rook => Self::rooks_move(from_pos, to_pos),
            Knight => Self::knights_move(from_pos, to_pos),
            King => Self::kings_move(from_pos, to_pos),
            Queen => Self::rooks_move(from_pos, to_pos) || Self::diag_move(from_pos, to_pos),
            Pawn => Self::pawns_move(from_pos, to_pos, self.colour()),
        }
    }

    fn to_rep(&self) -> char {
        use PossiblePieces::*;
        match self.poss_piece() {
            Bishop => 'B',
            Rook => 'R',
            Knight => 'N',
            King => 'K',
            Queen => 'Q',
            Pawn => 'P',
        }
    }

    pub fn to_fen_rep(&self) -> char {
        let rep = self.to_rep();
        if self.colour() == Colour::Black {
            rep.to_ascii_lowercase()
        } else {
            rep
        }
    }

    pub fn to_alg_rep(&self) -> Option<char> {
        let rep = self.to_rep();
        if rep == 'P' {
            None
        } else {
            Some(rep)
        }
    }

    pub fn change_colour(&self) -> Self {
        let mut s = self.clone();
        match s.colour() {
            Colour::Black => s.1 = Colour::White,
            Colour::White => s.1 = Colour::Black,
        }
        s
    }

    pub fn colour(&self) -> Colour {
        self.1
    }

    pub fn poss_piece(&self) -> PossiblePieces {
        self.0
    }
}
