use crate::boards::idx_coord::BoardCoord;
use crate::boards::is_in_board::IsInBoard;

pub fn get_neighbours(origin: usize, plus: bool, cross: bool) -> Vec<usize> {
    let coord = BoardCoord::from(origin);
    let mut neighbours = vec![];
    if cross {
        neighbours.push(offset(&coord, -1, -1));
        neighbours.push(offset(&coord, 1, -1));
        neighbours.push(offset(&coord, -1, 1));
        neighbours.push(offset(&coord, 1, 1));
    }
    if plus {
        neighbours.push(offset(&coord, 0, 1));
        neighbours.push(offset(&coord, 0, -1));
        neighbours.push(offset(&coord, 1, 0));
        neighbours.push(offset(&coord, -1, 0));
    }
    neighbours
        .iter()
        .filter_map(|pair| {
            if pair.is_in_board() {
                Some(BoardCoord::from(*pair).idx())
            } else {
                None
            }
        })
        .collect()
}

fn offset(coord: &BoardCoord, x: isize, y: isize) -> (isize, isize) {
    let mut xy: (isize, isize) = coord.clone().into();
    xy.0 += x;
    xy.1 += y;
    xy
}
