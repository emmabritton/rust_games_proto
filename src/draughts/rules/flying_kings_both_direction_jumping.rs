use crate::boards::board_cols;
use crate::draughts::moves::{Capture, Move};
use crate::draughts::rules::common::check_moves;
use crate::draughts::rules::{common, MoveDir, RuleSet, VALUE_STEP};
use crate::draughts::rules::{CAPTURABLE, VALUE_CAPTURE};
use crate::draughts::Square::*;
use crate::draughts::{Board, PastMove, Square};
use crate::system::math::next_step;
use crate::system::neighbours::get_neighbours;
use crate::system::PlayState;
use crate::system::PlayState::{ComputerWin, Draw, HumanWin};
use crate::system::Player::*;

//International, Canadian
pub(super) struct FlyingKingsBothDirectionJumping;

impl FlyingKingsBothDirectionJumping {
    pub(super) fn new() -> Self {
        FlyingKingsBothDirectionJumping {}
    }
}

impl FlyingKingsBothDirectionJumping {
    const fn calc_step_dir(&self, square: &Square) -> MoveDir {
        match square {
            ComputerMan => MoveDir::Down,
            HumanMan => MoveDir::Up,
            ComputerKing | HumanKing | Empty => MoveDir::Both,
        }
    }

    const fn supports_long_step(&self, square: &Square) -> bool {
        square.is_king()
    }

    fn get_moves_for_square(&self, board: &Board, origin: usize, capture_only: bool) -> Vec<Move> {
        let mut step_neighbours = get_neighbours(origin, false, true);
        match self.calc_step_dir(&board[origin]) {
            MoveDir::Up => step_neighbours.retain(|idx| idx < &origin),
            MoveDir::Down => step_neighbours.retain(|idx| idx > &origin),
            MoveDir::Both => {}
        }
        let jump_neighbours = get_neighbours(origin, false, true);
        let mut steps: Vec<Move> = step_neighbours
            .iter()
            .map(|neighbour| {
                let mut moves = vec![];
                let mut current = *neighbour;
                let mut next = if self.supports_long_step(&board[origin]) {
                    next_step(origin, current)
                } else {
                    None
                };
                loop {
                    let square = board[current];
                    if square == Square::Empty && !capture_only {
                        debug_log!("Found step from {} to {}", origin, current);
                        moves.push(Move::Step {
                            origin,
                            dest: current,
                            value: VALUE_STEP,
                        });
                        if let Some(next_square) = next {
                            next = next_step(current, next_square);
                            current = next_square;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                moves
            })
            .flatten()
            .collect();
        let mut jumps: Vec<Move> = jump_neighbours
            .iter()
            .filter_map(|neighbour| {
                let mut current = *neighbour;
                let mut next = next_step(origin, current);
                loop {
                    let square = board[current];
                    if square == Empty && self.supports_long_step(&board[origin]) {
                        if let Some(dest) = next {
                            next = next_step(current, dest);
                            current = dest;
                        } else {
                            return None;
                        }
                    } else if (&CAPTURABLE[&board[origin]]).contains(&square) {
                        if let Some(landing) = next {
                            if board[landing] == Empty {
                                debug_log!(
                                    "Found jump from {} to {} (capturing {})",
                                    origin,
                                    landing,
                                    current
                                );
                                return Some(Move::Jump {
                                    origin,
                                    capture: Capture {
                                        dest: landing,
                                        capturing: current,
                                    },
                                    value: VALUE_CAPTURE,
                                });
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            })
            .collect();
        steps.append(&mut jumps);
        steps
    }
}

impl RuleSet for FlyingKingsBothDirectionJumping {
    fn calc_valid_moves(&self, board: &Board, origin: usize) -> Vec<Move> {
        common::calc_valid_moves(board, origin, |board, origin, capture_only| {
            self.get_moves_for_square(board, origin, capture_only)
        })
    }

    fn check_game_over(&self, board: &Board, move_history: &Vec<PastMove>) -> Option<PlayState> {
        let human_king_count = common::get_king_count(board, Human);
        let human_man_count = common::get_man_count(board, Human);
        let computer_king_count = common::get_king_count(board, Computer);
        let computer_man_count = common::get_man_count(board, Computer);
        if human_king_count == 0 && human_man_count == 0 {
            debug_log!("Human has no remaining pieces: computer wins!");
            return Some(ComputerWin);
        } else if computer_king_count == 0 && computer_man_count == 0 {
            debug_log!("Computer has no remaining pieces: human wins!");
            return Some(HumanWin);
        } else if computer_king_count == 1
            && human_king_count == 1
            && human_man_count == 0
            && computer_man_count == 0
        {
            debug_log!("Only one king per player remains: draw!");
            return Some(Draw);
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
