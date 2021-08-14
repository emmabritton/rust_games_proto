use crate::senet::{Board, Move, Square};
use crate::system::Player;

pub(super) const REPEAT_TURN_ROLL: [usize; 3] = [1, 4, 5];
pub(super) const HOUSE_HAPPINESS: usize = 27;
pub(super) const HOUSE_REBIRTH: usize = 15;
pub(super) const HOUSE_BEAUTY: usize = 25;
pub(super) const HOUSE_WATER: usize = 26;
pub(super) const HOME: usize = 29;

pub(super) fn calc_valid_moves(board: &Board, roll: usize, player: Player) -> Vec<Move> {
    debug_log_start!("Calculating for {:?} with {}", player, roll);
    board
        .iter()
        .enumerate()
        .filter_map(|(idx, square)| {
            if square.player() == player.into() {
                Some(idx)
            } else {
                None
            }
        })
        .filter_map(|piece_idx| {
            let target = (piece_idx + roll).min(HOME);
            debug_log_start!("Calculating for {} to {}", piece_idx, target);
            let result = if can_jump(board, piece_idx, target, player) {
                if piece_idx < HOUSE_BEAUTY && target > HOUSE_BEAUTY {
                    if board[HOUSE_BEAUTY] == Square::Empty
                        || board[HOUSE_BEAUTY] == player.opposite().into()
                    {
                        debug_log!("Beauty blocking");
                        Some(Move::new(
                            piece_idx,
                            HOUSE_BEAUTY,
                            board[HOUSE_BEAUTY] == player.opposite().into(),
                        ))
                    } else {
                        debug_log!("Beauty blocking but occupied");
                        None
                    }
                } else if board[target] == player.opposite().into() {
                    debug_log!("Exchange found");
                    Some(Move::new(piece_idx, target, true))
                } else {
                    debug_log!("Jump found");
                    Some(Move::new(piece_idx, target, false))
                }
            } else {
                debug_log!("No move");
                None
            };
            debug_log_end!();
            result
        })
        .collect()
}

fn can_jump(board: &Board, current: usize, target: usize, player: Player) -> bool {
    if board[target] != player.into() {
        let mut consecutive_count = 0;
        for slot in board.iter().take(target).skip(current + 1) {
            //TODO maybe also reset count after changing row
            if slot == Square::Empty || slot == player.into() {
                consecutive_count = 0
            } else {
                consecutive_count += 1;
                if consecutive_count >= 2 {
                    break;
                }
            }
        }
        if consecutive_count < 2 {
            return true;
        }
    }
    false
}
