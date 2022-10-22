use crate::senet::State;
use crate::system::TurnState::SelectingPiece;
use rand::{thread_rng, Rng};

pub(super) fn process(state: &mut State) {
    if state.play_state.is_computer(SelectingPiece) {
        state.last_human_cursor_pos = state.cursor.idx;
        let piece_idx = state.valid_moves.keys().last().unwrap();
        state.cursor.idx = *piece_idx;
        let move_idx = thread_rng().gen_range(0.. state.get_moves_for_selected_piece().len());
        state.move_cursor = move_idx;
        debug_log!(
            "Selecting {} from {}: {:?}",
            move_idx,
            piece_idx,
            state.get_selected_move()
        );
    }
}
