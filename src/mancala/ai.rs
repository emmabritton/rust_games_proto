use crate::mancala::State;
use crate::system::Player;
use rand::{thread_rng, Rng};

pub(super) fn process(state: &mut State) {
    debug_log!("Homes: {:?}", state.board.computer.homes);
    //if any home slot will allow another turn play it
    for i in (0..=5).rev() {
        if state.board.computer.homes[i] == 6 - i {
            debug_log!("Another turn chance found, picking up from {}", i);
            state.computer_cursor = state
                .board
                .square_to_idx(&state.board.home_idx_to_square(Player::Computer, i));
            return;
        }
    }
    //if any captures are possible then play
    let mut capturables = vec![];
    for i in (0..=5).rev() {
        if state.board.computer.homes[i] == 0 && state.board.human.homes[5 - i] > 0 {
            capturables.push(i);
        }
    }
    debug_log!("Capturables: {:?}", capturables);
    for capturable in capturables {
        for i in (0..=5).rev() {
            let dest = if capturable > i {
                capturable - i
            } else {
                capturable + 8
            };
            if state.board.computer.homes[i] == dest {
                debug_log!("Capture chance found, picking up from {}", i);
                state.computer_cursor = state
                    .board
                    .square_to_idx(&state.board.home_idx_to_square(Player::Computer, i));
                return;
            }
        }
    }
    //otherwise pick a random move
    let valid_squares: Vec<usize> = state
        .board
        .computer
        .homes
        .iter()
        .enumerate()
        .filter_map(|(idx, num)| if num > &0 { Some(idx) } else { None })
        .collect();
    let computer_square = state.board.home_idx_to_square(
        Player::Computer,
        valid_squares[thread_rng().gen_range(0, valid_squares.len())],
    );
    state.computer_cursor = state.board.square_to_idx(&computer_square);
    debug_log!("Going with random move from {}", state.computer_cursor);
}
