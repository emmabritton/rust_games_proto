use crate::draughts::controller::Controller;
use crate::system::PlayState::Playing;
use crate::system::Player;
use crate::system::Turn::Computer;
use crate::system::TurnState::{SelectingMove, SelectingPiece};
use itertools::Itertools;
use rand::{thread_rng, Rng};

pub(super) fn process(controller: &mut Controller, delta: f64) {
    if controller.state.play_state.is_computer(SelectingPiece) {
        controller.state.next_move_time -= delta;
        if controller.state.next_move_time < 0. {
            controller.state.last_human_cursor_pos = controller.state.piece_cursor.idx;
            debug_log!("Starting computer move");
            let map = controller
                .state
                .all_possible_moves
                .values()
                .flatten()
                .into_group_map_by(|mov| mov.value());
            let highest_value = map.keys().sorted().last().unwrap();
            debug_log!("Highest value: {}", highest_value);
            let best_moves = map.get(highest_value).unwrap();

            debug_log!("{} best moves available", best_moves.len());

            let move_idx = thread_rng().gen_range(0, best_moves.len());
            let mov = best_moves[move_idx];
            controller.state.piece_cursor.idx = mov.origin();
            let (idx, _) = controller
                .state
                .get_moves_for_selected_piece()
                .iter()
                .find_position(|&mov| mov == best_moves[move_idx])
                .unwrap();

            debug_log!("Using piece {}", mov.origin());
            debug_log!("Using move {}", idx);
            controller.state.move_cursor = idx;
            debug_log!("Will play {}", mov);
            controller.state.next_move_time = 0.5;
            controller.state.play_state = Playing(Computer(SelectingMove))
        }
    }
    if controller.state.play_state.is_computer(SelectingMove) {
        controller.state.next_move_time -= delta;
        if controller.state.next_move_time < 0. {
            debug_log!("Now playing {}", controller.state.get_selected_move());
            controller.process_move(controller.state.get_selected_move());
            controller.state.piece_cursor.idx = controller.state.last_human_cursor_pos;
            controller.start_new_turn(Player::Human);
        }
    }
}
