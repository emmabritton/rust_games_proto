use crate::debug_log;
use crate::tictactoe::Square::E;
use crate::tictactoe::{State, COMPUTER_PIECE};
use rand::{thread_rng, Rng};

//Currently just places a random piece
pub(super) fn process(state: &mut State) {
    debug_log!("--AI turn starting--");
    debug_log!("{}", state);

    let empty_squares: Vec<usize> = state
        .board
        .iter()
        .enumerate()
        .filter_map(|(idx, square)| if square == &E { Some(idx) } else { None })
        .collect();

    debug_log!("{} empty squares: {:?}", empty_squares.len(), empty_squares);

    let square = empty_squares[thread_rng().gen_range(0.. empty_squares.len())];

    debug_log!("Selected {}", square);

    state.board[square] = COMPUTER_PIECE;
    debug_log!("--AI turn finished--");
    debug_log!("{}", state);
}
