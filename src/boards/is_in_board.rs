use crate::boards::idx_coord::BoardCoord;
use crate::boards::{board_cols, board_rows};

pub trait IsInBoard {
    fn is_in_board(&self) -> bool;
}

impl IsInBoard for (usize, usize) {
    fn is_in_board(&self) -> bool {
        BoardCoord::from(*self).is_in_board()
    }
}

impl IsInBoard for BoardCoord {
    fn is_in_board(&self) -> bool {
        self.0 < board_cols() && self.1 < board_rows()
    }
}

impl IsInBoard for (isize, isize) {
    fn is_in_board(&self) -> bool {
        self.0 >= 0
            && self.1 >= 0
            && self.0 < board_cols() as isize
            && self.1 < board_rows() as isize
    }
}
