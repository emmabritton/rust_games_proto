use ggez::mint::Point2;
use crate::boards::idx_coord::BoardCoord;
use crate::boards::is_in_board::IsInBoard;

pub type Point = Point2<f32>;

pub fn pt(x: f32, y: f32) -> Point {
    Point2{x, y}
}
pub fn pt_usize(x: usize, y: usize) -> Point {
    Point2{x:x as f32,y: y as f32}
}

//Adds the difference between origin and mid to mid and returns it
//If origin = idx(3,4) and mid = idx(3,3) then returns idx(3,2)
//If origin = idx(1,3) and mid = idx(3,4) then returns idx(5,5)
pub fn next_step(origin: usize, mid: usize) -> Option<usize> {
    let origin: (isize, isize) = BoardCoord::from(origin).into();
    let mid: (isize, isize) = BoardCoord::from(mid).into();
    let diff = (mid.0 - origin.0, mid.1 - origin.1);
    let dest = (origin.0 + diff.0 * 2, origin.1 + diff.1 * 2);
    if dest.is_in_board() {
        Some(BoardCoord::from(dest).idx())
    } else {
        None
    }
}

pub fn idx_to_coord(idx: usize, cols: usize) -> (usize, usize) {
    let x = idx % cols;
    let y = idx / cols;
    (x, y)
}

pub trait OffsetTuple<T> {
    fn offset_point(&self, amount: T) -> Self;
    fn multiply_point(&self, amount: T) -> Self;
}

impl OffsetTuple<Point> for Point {
    fn offset_point(&self, amount: Point) -> Self {
        pt(self.x + amount.x, self.y + amount.y)
    }

    fn multiply_point(&self, amount: Point) -> Self {
        pt(self.x * amount.x, self.y * amount.y)
    }
}

pub trait Offset<T> {
    fn offsetx(&self, amount: T) -> Self;
    fn offsety(&self, amount: T) -> Self;
    fn offset(&self, amount_x: T, amount_y: T) -> Self;
    fn multiply(&self, amount_x: T, amount_y: T) -> Self;
}

impl Offset<i32> for Point {
    fn offsetx(&self, amount: i32) -> Self {
        pt(self.x + amount as f32, self.y)
    }

    fn offsety(&self, amount: i32) -> Self {
        pt(self.x, self.y + amount as f32)
    }

    fn offset(&self, amount_x: i32, amount_y: i32) -> Self {
        pt(self.x + amount_x as f32, self.y + amount_y as f32)
    }

    fn multiply(&self, amount_x: i32, amount_y: i32) -> Self {
        pt(self.x * amount_x as f32, self.y * amount_y as f32)
    }
}

impl Offset<isize> for Point {
    fn offsetx(&self, amount: isize) -> Self {
        pt(self.x + amount as f32, self.y)
    }

    fn offsety(&self, amount: isize) -> Self {
        pt(self.x, self.y + amount as f32)
    }

    fn offset(&self, amount_x: isize, amount_y: isize) -> Self {
        pt(self.x + amount_x as f32, self.y + amount_y as f32)
    }

    fn multiply(&self, amount_x: isize, amount_y: isize) -> Self {
        pt(self.x * amount_x as f32, self.y * amount_y as f32)
    }
}

impl Offset<usize> for Point {
    fn offsetx(&self, amount: usize) -> Self {
        pt(self.x + amount as f32, self.y)
    }

    fn offsety(&self, amount: usize) -> Self {
        pt(self.x, self.y + amount as f32)
    }

    fn offset(&self, amount_x: usize, amount_y: usize) -> Self {
        pt(self.x + amount_x as f32, self.y + amount_y as f32)
    }

    fn multiply(&self, amount_x: usize, amount_y: usize) -> Self {
        pt(self.x * amount_x as f32, self.y * amount_y as f32)
    }
}

impl Offset<f32> for Point {
    fn offsetx(&self, amount: f32) -> Self {
        pt(self.x + amount, self.y)
    }

    fn offsety(&self, amount: f32) -> Self {
        pt(self.x, self.y + amount)
    }

    fn offset(&self, amount_x: f32, amount_y: f32) -> Self {
        pt(self.x + amount_x, self.y + amount_y)
    }

    fn multiply(&self, amount_x: f32, amount_y: f32) -> Self {
        pt(self.x * amount_x, self.y * amount_y)
    }
}

//Value can be between min (inclusive) and max (exclusive)
//Use set_bounds() if increasing min or decreasing max
//Use set_value() to force any value to nearest limit
#[derive(Debug)]
pub struct WrappedUsize {
    pub value: usize,
    min: usize,
    max: usize,
}

impl WrappedUsize {
    pub fn new(value: usize, min: usize, mut max: usize) -> Self {
        if max > 0 {
            max -= 1;
        }
        WrappedUsize { value, min, max }
    }

    pub fn new_zero_based(mut max: usize) -> Self {
        if max > 0 {
            max -= 1;
        }
        WrappedUsize {
            value: 0,
            min: 0,
            max,
        }
    }
}

impl WrappedUsize {
    pub fn inc(&mut self) {
        if self.value < self.max {
            self.value += 1;
        } else {
            self.value = self.min;
        }
    }

    pub fn dec(&mut self) {
        if self.value > self.min {
            self.value -= 1;
        } else {
            self.value = self.max;
        }
    }

    pub fn set_bounds(&mut self, min: usize, mut max: usize) {
        if max > 0 {
            max -= 1;
        }
        self.min = min;
        self.max = max;
        self.set_value(self.value)
    }

    pub fn set_value(&mut self, value: usize) {
        if value < self.min {
            self.value = 0;
        } else if value > self.max {
            self.value = self.max;
        } else {
            self.value = value;
        }
    }
}
