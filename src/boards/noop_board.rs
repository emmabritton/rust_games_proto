pub struct NoopBoard {
    pub rows: usize,
    pub cols: usize,
}

impl NoopBoard {
    pub fn new(rows: usize, cols: usize) -> Self {
        NoopBoard { rows, cols }
    }
}
