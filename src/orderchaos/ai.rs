use crate::orderchaos::Square::Empty;
use crate::orderchaos::{Board, Mode, State};
use crate::system::neighbours::get_neighbours;
use rand::{thread_rng, Rng};

fn get_random_empty_square(board: &Board) -> usize {
    let empties: Vec<usize> = board
        .iter()
        .enumerate()
        .filter_map(|(idx, square)| if square == &Empty { Some(idx) } else { None })
        .collect();
    empties[thread_rng().gen_range(0.. empties.len())]
}

pub(super) fn process(state: &mut State) {
    match state.player_mode {
        Mode::Order => {
            let mut neighbours = get_neighbours(state.cursor.idx, true, true);
            neighbours = neighbours
                .iter()
                .filter(|&idx| state.board[*idx] == Empty)
                .cloned()
                .collect();
            let (idx, square) = if neighbours.is_empty() {
                (
                    get_random_empty_square(&state.board),
                    if thread_rng().gen::<f32>() > 0.9 {
                        state.board[state.cursor.idx].opposite().unwrap()
                    } else {
                        state.board[state.cursor.idx]
                    },
                )
            } else {
                (
                    neighbours[thread_rng().gen_range(0.. neighbours.len())],
                    state.board[state.cursor.idx].opposite().unwrap(),
                )
            };
            state.cursor.idx = idx;
            state.move_cursor = square.into();
        }
        Mode::Chaos => {
            let already_placed: Vec<usize> = state
                .board
                .iter()
                .enumerate()
                .filter_map(|(idx, square)| if square != &Empty { Some(idx) } else { None })
                .collect();
            let mut count = 0;
            let mut placed = false;
            loop {
                let idx = already_placed[thread_rng().gen_range(0.. already_placed.len())];
                let neighbours: Vec<usize> = get_neighbours(idx, true, true)
                    .iter()
                    .filter(|&square| state.board[*square] == Empty)
                    .cloned()
                    .collect();
                if !neighbours.is_empty() {
                    let square = state.board[idx];
                    let idx = neighbours[thread_rng().gen_range(0.. neighbours.len())];
                    state.cursor.idx = idx;
                    state.move_cursor = square.into();
                    placed = true;
                    break;
                }
                count += 1;
                if count > 10 {
                    break;
                }
            }
            if !placed {
                debug_log!("Unable to place neighbour");
                state.cursor.idx = get_random_empty_square(&state.board);
                state.move_cursor = if thread_rng().gen::<bool>() {
                    Mode::Order
                } else {
                    Mode::Chaos
                };
            }
        }
    }
}
