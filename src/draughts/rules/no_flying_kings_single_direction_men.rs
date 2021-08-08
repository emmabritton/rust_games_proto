use crate::boards::board_cols;
use crate::draughts::moves::{Capture, Move};
use crate::draughts::rules::common::check_moves;
use crate::draughts::rules::{common, MoveDir, RuleSet};
use crate::draughts::rules::{CAPTURABLE, VALUE_CAPTURE, VALUE_STEP};
use crate::draughts::Square::*;
use crate::draughts::{Board, PastMove, Square};
use crate::system::math::next_step;
use crate::system::neighbours::get_neighbours;
use crate::system::PlayState;
use crate::system::PlayState::{ComputerWin, Draw, HumanWin};
use crate::system::Player::*;

//English
pub(super) struct NoFlyingKingsSingleDirectionMen;

impl NoFlyingKingsSingleDirectionMen {
    pub(super) fn new() -> Self {
        NoFlyingKingsSingleDirectionMen {}
    }
}

impl NoFlyingKingsSingleDirectionMen {
    const fn calc_step_dir(&self, square: &Square) -> MoveDir {
        match square {
            ComputerMan => MoveDir::Down,
            HumanMan => MoveDir::Up,
            ComputerKing | HumanKing | Empty => MoveDir::Both,
        }
    }

    const fn calc_jump_dir(&self, square: &Square) -> MoveDir {
        match square {
            ComputerMan => MoveDir::Down,
            HumanMan => MoveDir::Up,
            ComputerKing | HumanKing | Empty => MoveDir::Both,
        }
    }

    fn get_moves_for_square(&self, board: &Board, origin: usize, capture_only: bool) -> Vec<Move> {
        let mut step_neighbours = get_neighbours(origin, false, true);
        match self.calc_step_dir(&board[origin]) {
            MoveDir::Up => step_neighbours.retain(|idx| idx < &origin),
            MoveDir::Down => step_neighbours.retain(|idx| idx > &origin),
            MoveDir::Both => {}
        }
        let mut jump_neighbours = get_neighbours(origin, false, true);
        match self.calc_jump_dir(&board[origin]) {
            MoveDir::Up => jump_neighbours.retain(|idx| idx < &origin),
            MoveDir::Down => jump_neighbours.retain(|idx| idx > &origin),
            MoveDir::Both => {}
        }
        let mut steps: Vec<Move> = step_neighbours
            .iter()
            .filter_map(|neighbour| {
                let square = board[*neighbour];
                if square == Square::Empty && !capture_only {
                    debug_log!("Found step from {} to {}", origin, neighbour);
                    Some(Move::Step {
                        origin,
                        dest: *neighbour,
                        value: VALUE_STEP,
                    })
                } else {
                    None
                }
            })
            .collect();
        let mut jumps: Vec<Move> = jump_neighbours
            .iter()
            .filter_map(|neighbour| {
                let square = board[*neighbour];
                if (&CAPTURABLE[&board[origin]]).contains(&square) {
                    if let Some(landing) = next_step(origin, *neighbour) {
                        if board[landing] == Empty {
                            debug_log!(
                                "Found jump from {} to {} (capturing {})",
                                origin,
                                landing,
                                neighbour
                            );
                            Some(Move::Jump {
                                origin,
                                capture: Capture {
                                    dest: landing,
                                    capturing: *neighbour,
                                },
                                value: VALUE_CAPTURE,
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        steps.append(&mut jumps);
        steps
    }
}

impl RuleSet for NoFlyingKingsSingleDirectionMen {
    fn calc_valid_moves(&self, board: &Board, origin: usize) -> Vec<Move> {
        common::calc_valid_moves(board, origin, |board, origin, capture_only| {
            self.get_moves_for_square(board, origin, capture_only)
        })
    }

    fn check_game_over(&self, board: &Board, move_history: &[PastMove]) -> Option<PlayState> {
        let human_count = common::get_piece_count(board, Human);
        let computer_count = common::get_piece_count(board, Computer);
        if human_count == 0 {
            debug_log!("Human has no remaining pieces: computer wins!");
            return Some(ComputerWin);
        } else if computer_count == 0 {
            debug_log!("Computer has no remaining pieces: human wins!");
            return Some(HumanWin);
        } else if move_history.len() > 40 {
            let is_stalement = check_moves(move_history, 25, |mov| {
                mov.king_capture_count == 0 && mov.man_capture_count == 0 && (mov.piece.is_king())
            });
            if is_stalement {
                debug_log!("At least 25 moves with no captures and only kings remain: draw!");
                return Some(Draw);
            }
        }
        None
    }

    fn is_promotion(&self, board: &Board, origin: usize, dest: usize) -> Option<Square> {
        if dest < board_cols() && board[origin] == HumanMan {
            Some(HumanKing)
        } else if dest > board.len() - board_cols() && board[origin] == ComputerMan {
            Some(ComputerKing)
        } else {
            None
        }
    }
}
