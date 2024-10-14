use crate::piece::{Colour, Piece, PossiblePieces};
use std::{default, fmt, ops};

type Pair = (usize, usize);

pub struct Board {
    board: [[Option<Piece>; 8]; 8],
}

pub struct IntoIter {
    board: Board,
    row: usize,
}

impl Iterator for IntoIter {
    type Item = [Option<Piece>; 8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= 8 {
            None
        } else {
            self.row += 1;
            Some(self.board[self.row - 1])
        }
    }
}

pub struct Iter<'a> {
    board: &'a Board,
    row: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a [Option<Piece>];

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= 8 {
            None
        } else {
            self.row += 1;
            Some(&self.board[self.row - 1])
        }
    }
}

pub struct IterMut<'a> {
    board: &'a mut Board,
    row: usize,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut [Option<Piece>];

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= 8 {
            None
        } else {
            let ptr = self.board[self.row].as_mut_ptr();
            self.row += 1;
            unsafe { Some(std::slice::from_raw_parts_mut::<'a>(ptr, 8)) }
        }
    }
}

/*
impl ops::Deref for Board {
    type Target = [[Option<Piece>; 8]; 8];
    fn deref(&self) -> &Self::Target {
        &self.board
    }
}
 */
impl ops::Index<Pair> for Board {
    type Output = Option<Piece>;

    fn index(&self, idx: Pair) -> &Self::Output {
        &self.board[idx.0][idx.1]
    }
}

impl ops::IndexMut<Pair> for Board {
    fn index_mut(&mut self, idx: Pair) -> &mut Self::Output {
        &mut self.board[idx.0][idx.1]
    }
}

impl ops::Index<usize> for Board {
    type Output = [Option<Piece>; 8];

    fn index(&self, idx: usize) -> &Self::Output {
        &self.board[idx]
    }
}

impl ops::IndexMut<usize> for Board {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.board[idx]
    }
}

impl default::Default for Board {
    fn default() -> Self {
        let empty_row = [None; 8];
        let pawn_row: [Option<_>; 8] =
            std::iter::repeat(Piece::new(PossiblePieces::Pawn, Colour::White))
                .map(Some)
                .take(8)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
        let piece_row: [Option<_>; 8] = [
            PossiblePieces::Rook,
            PossiblePieces::Knight,
            PossiblePieces::Bishop,
            PossiblePieces::Queen,
            PossiblePieces::King,
            PossiblePieces::Bishop,
            PossiblePieces::Knight,
            PossiblePieces::Rook,
        ]
        .into_iter()
        .map(|p| Piece::new(p, Colour::White))
        .map(Some)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

        Self {
            board: [
                piece_row,
                pawn_row,
                empty_row,
                empty_row,
                empty_row,
                empty_row,
                pawn_row
                    .into_iter()
                    .map(|p| p.expect("pawn_row contains None").change_colour())
                    .map(Some)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                piece_row
                    .into_iter()
                    .map(|p| p.expect("piece_row contains None").change_colour())
                    .map(Some)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            ],
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.board
                .into_iter()
                .rev()
                .map(|row| {
                    row.into_iter()
                        .map(|opt| opt.map(|p| p.to_fen_rep()).unwrap_or(' ').to_string())
                        .collect::<Vec<_>>()
                        .join("│")
                })
                .collect::<Vec<_>>()
                .join("\n─┼─┼─┼─┼─┼─┼─┼─\n")
        )
    }
}

impl Board {
    fn valid_pos(pos: Pair) -> bool {
        pos.0 < 8 && pos.1 < 8
    }

    pub fn set_piece(&mut self, pos: Pair, piece: Option<Piece>) {
        if !Self::valid_pos(pos) {
            return;
        }
        self[pos] = piece;
    }

    pub fn move_piece(&mut self, from_pos: Pair, to_pos: Pair) {
        if !Self::valid_pos(from_pos) || !Self::valid_pos(to_pos) {
            return;
        }
        let p = self[from_pos].take();
        self[to_pos] = p;
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            board: self,
            row: 0,
        }
    }

    pub fn into_iter(self) -> IntoIter {
        IntoIter {
            board: self,
            row: 0,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut {
            board: self,
            row: 0,
        }
    }
}
