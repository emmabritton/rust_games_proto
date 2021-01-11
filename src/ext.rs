pub trait NewLines {
    fn new_lines(&self) -> usize;
}

impl NewLines for &str {
    fn new_lines(&self) -> usize {
        self.chars().filter(|c| c == &'\n').count()
    }
}
