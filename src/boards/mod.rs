pub mod chessboard;
pub mod cursor;
pub mod idx_coord;
pub mod is_in_board;
pub mod single_char_board_converter;

use std::sync::atomic::{AtomicUsize, Ordering};

static BOARD_ROWS: AtomicUsize = AtomicUsize::new(0);
static BOARD_COLS: AtomicUsize = AtomicUsize::new(0);

pub fn set_board_size(size: (usize, usize)) {
    BOARD_ROWS.store(size.0, Ordering::SeqCst);
    BOARD_COLS.store(size.1, Ordering::SeqCst);
}

#[inline]
pub fn board_rows() -> usize {
    BOARD_ROWS.load(Ordering::SeqCst)
}

#[inline]
pub fn board_cols() -> usize {
    BOARD_COLS.load(Ordering::SeqCst)
}

pub trait CoordIdxConverter {
    fn is_valid_coord(&self, alpha: &str, num: &str) -> bool;
    fn coord_to_idx(&self, alpha: &str, num: &str) -> usize;
    fn idx_to_coord(&self, idx: usize) -> (String, String);
}

#[cfg(test)]
mod test {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial(board_size)]
    fn test_board_size() {
        set_board_size((3, 6));
        assert_eq!(board_rows(), 3);
        assert_eq!(board_cols(), 6);
        set_board_size((9, 1));
        assert_eq!(board_rows(), 9);
        assert_eq!(board_cols(), 1);
    }
}
