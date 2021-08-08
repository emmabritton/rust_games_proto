use crate::boards::chessboard::ChessBoard;
use crate::boards::cursor::Cursor;
use crate::boards::idx_coord::BoardCoord;
use crate::boards::single_char_board_converter::SingleCharBoardConverter;
use crate::boards::{board_cols, board_rows, set_board_size};
use crate::constants::{games, AI_MOVE_DELAY};
use crate::debug_log;
use crate::draughts::ai::process;
use crate::draughts::moves::Move;
use crate::draughts::renderer::render;
use crate::draughts::rules::{GameVariant, RuleSet};
use crate::draughts::Square::{ComputerKing, ComputerMan, Empty, HumanKing, HumanMan};
use crate::draughts::{PastMove, Square, State};
use crate::system::find_nearest::find_nearest;
use crate::system::ggez_ext::keycode_to_direction;
use crate::system::mesh_helper::MeshHelper;
use crate::system::PlayState::{ComputerWin, HumanWin, Playing};
use crate::system::Turn::Human;
use crate::system::TurnState::{SelectingMove, SelectingPiece};
use crate::system::{PlayState, Player, Scene, NEW_TURN_COMPUTER, NEW_TURN_HUMAN};
use ggez::event::KeyCode;
use ggez::{Context, GameResult};
use itertools::Itertools;
use std::collections::HashMap;

pub struct Controller {
    pub(super) state: State,
    rules: Box<dyn RuleSet>,
}

impl Controller {
    pub fn new(game: &str) -> Controller {
        let variant = match game {
            games::DRAUGHTS_ENGLISH => GameVariant::English,
            games::DRAUGHTS_INTERNATIONAL => GameVariant::International,
            games::DRAUGHTS_CANADIAN => GameVariant::Canadian,
            games::DRAUGHTS_BRAZILIAN => GameVariant::Brazilian,
            _ => panic!("Invalid game for draughts controller: {}", game),
        };
        set_board_size(variant.get_board_size());
        let converter = SingleCharBoardConverter::new(board_rows(), board_cols());
        let calc = ChessBoard::new(Box::new(converter), board_rows(), board_cols());
        Controller {
            state: State {
                board: variant.get_init_board(),
                board_calc: calc,
                play_state: PlayState::Init,
                piece_cursor: Cursor::new(),
                all_possible_moves: HashMap::new(),
                move_cursor: 0,
                move_history: vec![],
                next_move_time: 0.,
                last_human_cursor_pos: 0,
            },
            rules: variant.get_rules(),
        }
    }
}

impl Controller {
    fn select_piece(&mut self) {
        let highlighted_piece = self.state.board[self.state.piece_cursor.idx];
        if (highlighted_piece == Square::HumanKing || highlighted_piece == Square::HumanMan)
            && !self.state.get_moves_for_selected_piece().is_empty()
        {
            self.state.play_state = Playing(Human(SelectingMove));
        }
    }

    fn select_move(&mut self) {
        self.process_move(self.state.get_selected_move());
        self.start_new_turn(Player::Computer);
    }

    pub(super) fn start_new_turn(&mut self, player: Player) {
        debug_log_start!("\n\n\n----\nStarting new turn for {:?}", player);
        match player {
            Player::Human => self.state.play_state = NEW_TURN_HUMAN,
            Player::Computer => self.state.play_state = NEW_TURN_COMPUTER,
        }
        self.state.next_move_time = AI_MOVE_DELAY;
        self.update_valid_moves(player);
        self.check_for_game_over();
        debug_log_end!();
    }

    fn check_for_game_over(&mut self) {
        if self.state.all_possible_moves.is_empty() {
            if self.state.play_state.is_human(SelectingPiece) {
                debug_log!("No moves possible for human: computer wins!");
                self.state.play_state = ComputerWin;
            } else {
                debug_log!("No moves possible for computer: human wins!");
                self.state.play_state = HumanWin;
            }
            return;
        }
        if let Some(new_state) = self
            .rules
            .check_game_over(&self.state.board, &self.state.move_history)
        {
            debug_log!("Game over: {:?}", new_state);
            self.state.play_state = new_state;
        }
    }

    fn update_valid_moves(&mut self, player: Player) {
        debug_log_start!("Calculating all possible moves for {:?}", player);
        let squares = match player {
            Player::Human => vec![HumanMan, HumanKing],
            Player::Computer => vec![ComputerKing, ComputerMan],
        };
        let mut all_moves = self
            .state
            .board
            .iter()
            .enumerate()
            .filter_map(|(idx, square)| {
                if squares.contains(square) {
                    Some(self.rules.calc_valid_moves(&self.state.board, idx))
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<Move>>();

        let has_multijump = all_moves.iter().any(|mov| mov.is_multi_jump());
        let has_jump = all_moves.iter().any(|mov| mov.is_jump());

        debug_log!(
            "Found {} moves, any multijumps: {}, any jumps: {}",
            all_moves.len(),
            has_multijump,
            has_jump
        );

        if has_multijump {
            all_moves = all_moves
                .iter()
                .filter(|mov| mov.is_multi_jump())
                .cloned()
                .collect();
            let longest = all_moves
                .iter()
                .map(|mov| mov.len())
                .max()
                .expect("Failed to find longest multijump");
            all_moves = all_moves
                .iter()
                .filter(|mov| mov.len() == longest)
                .cloned()
                .collect();
        } else if has_jump {
            all_moves = all_moves
                .iter()
                .filter(|mov| mov.is_jump())
                .cloned()
                .collect();
        }

        self.state.all_possible_moves = all_moves
            .iter()
            .map(|mov| (mov.origin(), mov.clone()))
            .into_group_map();
        debug_log_end!(
            "All possible moves for player {:?}:\n{:?}",
            player,
            self.state.all_possible_moves
        );
    }

    pub(super) fn process_move(&mut self, mov: Move) {
        let board = &mut self.state.board;
        match mov {
            Move::Step {
                origin,
                dest,
                value: _,
            } => {
                let (is_promotion, end_piece) =
                    if let Some(king) = self.rules.is_promotion(board, origin, dest) {
                        (true, king)
                    } else {
                        (false, board[origin])
                    };
                self.state.move_history.push(PastMove::new(
                    board[origin].into(),
                    origin,
                    vec![dest],
                    0,
                    0,
                    board[origin],
                    is_promotion,
                ));
                board[dest] = end_piece;
                board[origin] = Empty;
            }
            Move::Jump {
                origin,
                capture,
                value: _,
            } => {
                let (is_promotion, end_piece) =
                    if let Some(king) = self.rules.is_promotion(board, origin, capture.dest) {
                        (true, king)
                    } else {
                        (false, board[origin])
                    };
                let (king, man) = if board[capture.capturing].is_king() {
                    (1, 0)
                } else {
                    (0, 1)
                };
                self.state.move_history.push(PastMove::new(
                    board[origin].into(),
                    origin,
                    vec![capture.dest],
                    king,
                    man,
                    board[origin],
                    is_promotion,
                ));
                board[capture.dest] = end_piece;
                board[capture.capturing] = Empty;
                board[origin] = Empty;
            }
            Move::MultiJump {
                origin,
                captures,
                value: _,
            } => {
                let mut king = 0;
                let mut man = 0;
                for capture in &captures {
                    if board[capture.capturing].is_king() {
                        king += 1;
                    } else {
                        man += 1;
                    }
                }
                let last = captures.iter().last().unwrap();
                let (is_promotion, end_piece) =
                    if let Some(king) = self.rules.is_promotion(board, origin, last.dest) {
                        (true, king)
                    } else {
                        (false, board[origin])
                    };
                self.state.move_history.push(PastMove::new(
                    board[origin].into(),
                    origin,
                    captures.iter().map(|cap| cap.dest).collect(),
                    king,
                    man,
                    board[origin],
                    is_promotion,
                ));
                board[last.dest] = end_piece;
                board[last.capturing] = Empty;
                for cap in captures {
                    board[cap.capturing] = Empty;
                }
                board[origin] = Empty;
            }
        }
        #[allow(clippy::needless_range_loop)] //looks awful in comparison
        for idx in 0..board_cols() {
            if board[idx] == HumanMan {
                board[idx] = HumanKing
            }
        }
        for idx in (board.len() - board_cols())..(board.len()) {
            if board[idx] == ComputerMan {
                board[idx] = ComputerKing
            }
        }
    }
}

impl Scene for Controller {
    fn on_key_down(&mut self, key: KeyCode) {
        if self.state.play_state.is_human(SelectingPiece) {
            if self.state.piece_cursor.handle_input(key) {
                self.state.move_cursor = 0;
            } else if let KeyCode::Return = key {
                self.select_piece()
            }
            debug_log!("State updated: {:?}", self.state);
        } else if self.state.play_state.is_human(SelectingMove) {
            match key {
                KeyCode::Up | KeyCode::Left | KeyCode::Down | KeyCode::Right => {
                    let dir = keycode_to_direction(key).unwrap();
                    debug_log!("Selecting nearest move to the {:?}", dir);
                    debug_log!(
                        "{} possible moves for square {}, currently selected: {}",
                        self.state.get_moves_for_selected_piece().len(),
                        self.state.piece_cursor.idx,
                        self.state.move_cursor
                    );
                    let nearest = find_nearest(
                        &self.state.get_moves_for_selected_piece(),
                        self.state.move_cursor,
                        dir,
                        &|mov| BoardCoord::from(mov.dest()),
                    );
                    if let Some(nearest) = nearest {
                        debug_log!("Found: {:?}", nearest);
                        self.state.move_cursor = nearest;
                    } else {
                        debug_log!("Nothing found");
                    }
                }
                KeyCode::Return => self.select_move(),
                _ => {}
            }
            debug_log!("State updated: {:?}", self.state);
        }
    }

    fn on_key_up(&mut self, key: KeyCode) -> bool {
        if self.state.play_state.is_human(SelectingMove) {
            if let KeyCode::Escape = key {
                self.state.play_state = NEW_TURN_HUMAN;
                debug_log!("State updated: {:?}", self.state);
                return true;
            }
        }
        false
    }

    fn update(&mut self, delta: f64) -> GameResult<()> {
        if self.state.play_state == PlayState::Init {
            self.start_new_turn(Player::Human)
        }
        process(self, delta);
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context, mesh_helper: &mut MeshHelper) -> GameResult<()> {
        render(ctx, mesh_helper, &self.state)
    }

    fn play_state(&self) -> PlayState {
        self.state.play_state
    }
}
