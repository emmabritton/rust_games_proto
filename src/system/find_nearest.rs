use crate::boards::idx_coord::BoardCoord;
use crate::constants::Direction;
use std::fmt::Debug;

//Return the index of the nearest item in the list to 'from' in the direction of 'search'
pub fn find_nearest<T: Debug>(
    list: &[T],
    from: usize,
    search: Direction,
    transform: &dyn Fn(&T) -> BoardCoord,
) -> Option<usize> {
    debug_log!(
        "Finding nearest {:?} from {:?} out of {:?}",
        search,
        from,
        list
    );
    let from = transform(&list[from]);
    let nearest = list
        .iter()
        .enumerate()
        .filter(|(_, mov)| {
            let pt = transform(mov);
            match search {
                Direction::Up => pt.1 < from.1,
                Direction::Left => pt.0 < from.0,
                Direction::Down => pt.1 > from.1,
                Direction::Right => pt.0 > from.0,
            }
        })
        .min_by_key(|(_, mov)| {
            let pt = transform(mov);
            match search {
                Direction::Up => {
                    (from.1 - pt.1) + ((from.0 as isize - pt.0 as isize).abs() * 2) as usize
                }
                Direction::Left => {
                    ((from.1 as isize - pt.1 as isize).abs() * 2) as usize + (from.0 - pt.0)
                }
                Direction::Down => {
                    (pt.1 - from.1) + ((from.0 as isize - pt.0 as isize).abs() * 2) as usize
                }
                Direction::Right => {
                    ((from.1 as isize - pt.1 as isize).abs() * 2) as usize + (pt.0 - from.0)
                }
            }
        });

    if let Some((idx, _)) = nearest {
        Some(idx)
    } else {
        None
    }
}
