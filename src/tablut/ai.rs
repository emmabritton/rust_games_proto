use crate::tablut::State;
use itertools::Itertools;
use rand::{thread_rng, Rng};

pub(super) fn process(state: &mut State) {
    let moves = state.valid_moves.iter().into_group_map_by(|mov| mov.value);
    let highest_value = moves.keys().sorted().last().unwrap();
    debug_log!("Highest value: {}", highest_value);
    let best_moves = moves.get(highest_value).unwrap();

    debug_log!("{} best moves available", best_moves.len());

    let idx = thread_rng().gen_range(0, best_moves.len());
    let mov = best_moves[idx];
    debug_log!("Playing {:?}", mov);
    state.cursor.idx = mov.origin;
    for (i, piece_mov) in state.get_moves_for_selected_piece().iter().enumerate() {
        if piece_mov == mov {
            state.move_cursor = i;
            break;
        }
    }
}
