use crate::boards::CoordIdxConverter;
use regex::Regex;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct ChessBoard {
    converter: Box<dyn CoordIdxConverter>,
    pub rows: usize,
    pub cols: usize,
    row_char_count: usize,
    col_char_count: usize,
    regex: Regex,
}

impl Debug for ChessBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ChessBoard")
    }
}

impl ChessBoard {
    pub fn new(converter: Box<dyn CoordIdxConverter>, rows: usize, cols: usize) -> Self {
        let row_char_count = if rows >= 26 { 2 } else { 1 };
        let col_char_count = if rows >= 10 { 2 } else { 1 };
        let regex = Regex::new(&format!(
            "([a-zA-Z]{{1,{}}})([0-9]{{1,{}}})",
            row_char_count, col_char_count
        ))
        .expect("Failed to make board regex");
        ChessBoard {
            converter,
            rows,
            cols,
            row_char_count,
            col_char_count,
            regex,
        }
    }
}

impl ChessBoard {
    #[allow(clippy::iter_nth_zero)] //as there is also an nth(1) I think nth(0) looks nicer than next()
    fn coord_to_idx(&self, coord: &str) -> Option<usize> {
        if !(coord.is_ascii()) {
            return None;
        }

        let matches = self.regex.captures(coord);
        if let Some(matches) = matches {
            if matches.len() == 2 {
                let alpha = matches.iter().nth(0).unwrap().unwrap().as_str();
                let num = matches.iter().nth(1).unwrap().unwrap().as_str();
                if self.converter.is_valid_coord(alpha, num) {
                    return Some(self.converter.coord_to_idx(alpha, num));
                }
            }
        }
        None
    }

    fn idx_to_coord(&self, idx: usize) -> Option<String> {
        if idx >= self.cols * self.rows {
            return None;
        }
        let (a, n) = self.converter.idx_to_coord(idx);
        Some(format!("{}{}", a, n))
    }
}
