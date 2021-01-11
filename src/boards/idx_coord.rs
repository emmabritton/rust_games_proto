//These convert from an single index to a two dimensional index
//For example an array[5][5] can be represented as array[25]
//0 is 0,0; 24 is [4][4]; 4 is [4][0]
//4x4 board showing single dimen index:
// 0  1  2  3
// 4  5  6  7
// 8  9 10 11
//12 13 14 15

use crate::boards::board_cols;
use crate::boards::is_in_board::IsInBoard;
use crate::system::math::{idx_to_coord, pt_usize, Point};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BoardCoord(pub usize, pub usize);

impl BoardCoord {
    pub(crate) fn idx(&self) -> usize {
        self.0 + self.1 * board_cols()
    }
}

impl BoardCoord {
    pub(crate) fn dist(&self, other: BoardCoord) -> usize {
        let this = (self.0 as f32, self.1 as f32);
        let other = (other.0 as f32, other.1 as f32);
        ((this.0 - other.0).powf(2.) + (this.1 - other.1).powf(2.))
            .sqrt()
            .abs() as usize
    }
}

impl Display for BoardCoord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl From<(usize, usize)> for BoardCoord {
    fn from(coord: (usize, usize)) -> Self {
        BoardCoord(coord.0, coord.1)
    }
}

impl From<(isize, isize)> for BoardCoord {
    fn from(coord: (isize, isize)) -> Self {
        if !coord.is_in_board() {
            panic!("BoardIndex would be outside board: {},{}", coord.0, coord.1);
        }
        BoardCoord(coord.0 as usize, coord.1 as usize)
    }
}

impl From<usize> for BoardCoord {
    fn from(idx: usize) -> Self {
        let (x, y) = idx_to_coord(idx, board_cols());
        BoardCoord(x, y)
    }
}

impl From<BoardCoord> for Point {
    fn from(idx: BoardCoord) -> Self {
        pt_usize(idx.0, idx.1)
    }
}

impl From<BoardCoord> for (isize, isize) {
    fn from(coord: BoardCoord) -> Self {
        (coord.0 as isize, coord.1 as isize)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::boards::set_board_size;
    use serial_test::serial;

    #[test]
    fn test_basic_math() {
        assert_eq!(idx_to_coord(0, 4), (0, 0));
        assert_eq!(idx_to_coord(3, 4), (3, 0));
        assert_eq!(idx_to_coord(4, 4), (0, 1));
        assert_eq!(idx_to_coord(11, 4), (3, 2));
        assert_eq!(idx_to_coord(15, 4), (3, 3));
    }

    #[test]
    #[serial(board_size)]
    fn test_5x5() {
        set_board_size((5, 5));
        //    x 0  1  2  3  4
        // y  ---------------
        // 0 |  0  1  2  3  4
        // 1 |  5  6  7  8  9
        // 2 | 10 11 12 13 14
        // 3 | 15 16 17 18 19
        // 4 | 20 21 22 23 24

        //usize
        assert_eq!(BoardCoord::from(0), BoardCoord(0, 0));
        assert_eq!(BoardCoord::from(24), BoardCoord(4, 4));
        assert_eq!(BoardCoord(0, 0).idx(), 0);
        assert_eq!(BoardCoord(4, 4).idx(), 24);

        assert_eq!(BoardCoord::from(4), BoardCoord(4, 0));
        assert_eq!(BoardCoord::from(20), BoardCoord(0, 4));
        assert_eq!(BoardCoord(4, 0).idx(), 4);
        assert_eq!(BoardCoord(0, 4).idx(), 20);

        assert_eq!(BoardCoord::from(11), BoardCoord(1, 2));
        assert_eq!(BoardCoord(1, 2).idx(), 11);
    }

    #[test]
    #[serial(board_size)]
    fn test_3x6() {
        set_board_size((6, 3));
        //    x 0  1  2
        // y  ---------
        // 0 |  0  1  2
        // 1 |  3  4  5
        // 2 |  6  7  8
        // 3 |  9 10 11
        // 4 | 12 13 14
        // 5 | 15 16 17

        //usize
        assert_eq!(BoardCoord::from(0), BoardCoord(0, 0));
        assert_eq!(BoardCoord::from(17), BoardCoord(2, 5));
        assert_eq!(BoardCoord(0, 0).idx(), 0);
        assert_eq!(BoardCoord(2, 5).idx(), 17);

        assert_eq!(BoardCoord::from(2), BoardCoord(2, 0));
        assert_eq!(BoardCoord::from(15), BoardCoord(0, 5));
        assert_eq!(BoardCoord(2, 0).idx(), 2);
        assert_eq!(BoardCoord(0, 5).idx(), 15);

        assert_eq!(BoardCoord::from(10), BoardCoord(1, 3));
        assert_eq!(BoardCoord(1, 3).idx(), 10);
    }
}
