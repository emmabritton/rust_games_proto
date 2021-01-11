use crate::boards::cursor::Cursor;
use crate::boards::idx_coord::BoardCoord;
use crate::boards::set_board_size;
use crate::constants::{AI_MOVE_DELAY, ANIMATION_DURATION};
use crate::orderchaos::ai::process;
use crate::orderchaos::renderer::render;
use crate::orderchaos::Square::Empty;
use crate::orderchaos::{Mode, Square, State};
use crate::system::mesh_helper::MeshHelper;
use crate::system::PlayState::{ComputerWin, HumanWin};
use crate::system::TurnState::{SelectingMove, SelectingPiece};
use crate::system::{PlayState, Player, Scene, Turn, NEW_TURN_COMPUTER, NEW_TURN_HUMAN};
use ggez::event::KeyCode;
use ggez::{Context, GameResult};

pub struct Controller {
    state: State,
}

impl Controller {
    pub fn new() -> Self {
        set_board_size((6, 6));
        Controller {
            state: State {
                board: [Square::Empty; 36],
                player_mode: Mode::Order,
                play_state: PlayState::ModeSelection,
                cursor: Cursor::new(),
                next_move_time: 0.0,
                last_human_cursor_pos: 0,
                last_human_placed: Square::Empty,
                move_cursor: Mode::Order,
            },
        }
    }
}

impl Controller {
    fn process_move(&mut self) {
        let current_player = self.state.play_state.player().unwrap();
        debug_log!(
            "Setting {} to {:?} for {:?}",
            self.state.cursor.idx,
            self.state.move_cursor,
            current_player
        );
        self.state.board[self.state.cursor.idx] = self.state.move_cursor.into();

        match current_player {
            Player::Human => {
                debug_log!("Computers turn");
                self.state.play_state = NEW_TURN_COMPUTER;
            }
            Player::Computer => {
                debug_log!("Humans turn");
                self.state.play_state = NEW_TURN_HUMAN;
            }
        }
        self.state.next_move_time = AI_MOVE_DELAY;

        self.check_game_over();
    }

    fn check_game_over(&mut self) {
        let horz_line = self.has_valid_line(0, 1, 0, Some(5))
            || self.has_valid_line(1, 1, 0, Some(0))
            || self.has_valid_line(6, 1, 0, Some(11))
            || self.has_valid_line(7, 1, 0, Some(6))
            || self.has_valid_line(12, 1, 0, Some(17))
            || self.has_valid_line(13, 1, 0, Some(12))
            || self.has_valid_line(18, 1, 0, Some(23))
            || self.has_valid_line(19, 1, 0, Some(18))
            || self.has_valid_line(24, 1, 0, Some(29))
            || self.has_valid_line(25, 1, 0, Some(24))
            || self.has_valid_line(30, 1, 0, Some(35))
            || self.has_valid_line(31, 1, 0, Some(30));

        let vert_line = self.has_valid_line(0, 0, 1, Some(30))
            || self.has_valid_line(1, 0, 1, Some(31))
            || self.has_valid_line(2, 0, 1, Some(32))
            || self.has_valid_line(3, 0, 1, Some(33))
            || self.has_valid_line(4, 0, 1, Some(34))
            || self.has_valid_line(5, 0, 1, Some(35))
            || self.has_valid_line(6, 0, 1, Some(0))
            || self.has_valid_line(7, 0, 1, Some(1))
            || self.has_valid_line(8, 0, 1, Some(2))
            || self.has_valid_line(9, 0, 1, Some(3))
            || self.has_valid_line(10, 0, 1, Some(4))
            || self.has_valid_line(11, 0, 1, Some(5));

        let diag_line = self.has_valid_line(0, 1, 1, Some(35))
            || self.has_valid_line(1, 1, 1, None)
            || self.has_valid_line(6, 1, 1, None)
            || self.has_valid_line(7, 1, 1, Some(0))
            || self.has_valid_line(4, -1, 1, None)
            || self.has_valid_line(5, -1, 1, Some(30))
            || self.has_valid_line(10, -1, 1, Some(5))
            || self.has_valid_line(11, -1, 1, None);

        if horz_line || vert_line || diag_line {
            debug_log!(
                "Line found    horz: {}  vert: {}  diag: {}",
                horz_line,
                vert_line,
                diag_line
            );
            match self.state.player_mode {
                Mode::Order => {
                    debug_log!("Human wins");
                    self.state.play_state = HumanWin;
                }
                Mode::Chaos => {
                    debug_log!("Computer wins");
                    self.state.play_state = ComputerWin;
                }
            }
        } else {
            let empty_count = self
                .state
                .board
                .iter()
                .filter(|&square| square == &Empty)
                .count();
            if empty_count == 0 {
                debug_log!("No squares left");
                match self.state.player_mode {
                    Mode::Order => {
                        debug_log!("Computer wins");
                        self.state.play_state = ComputerWin;
                    }
                    Mode::Chaos => {
                        debug_log!("Human wins");
                        self.state.play_state = HumanWin;
                    }
                }
            }
        }
    }

    fn has_valid_line(
        &mut self,
        start: usize,
        x_diff: isize,
        y_diff: isize,
        diff: Option<usize>,
    ) -> bool {
        let start_square = self.state.board[start];
        if start_square == Empty {
            return false;
        }
        let (mut x, mut y): (isize, isize) = BoardCoord::from(start).into();
        for _ in 1..5 {
            x += x_diff;
            y += y_diff;
            if self.state.board[BoardCoord::from((x, y)).idx()] != start_square {
                return false;
            }
        }
        if let Some(diff) = diff {
            if start_square == self.state.board[diff] {
                return false;
            }
        }
        debug_log!(
            "Line found starting at {} incrementing by {},{}",
            start,
            x_diff,
            y_diff
        );
        true
    }
}

impl Scene for Controller {
    fn on_key_down(&mut self, key: KeyCode) {
        if self.state.play_state == PlayState::ModeSelection {
            match key {
                KeyCode::Left | KeyCode::Right => {
                    if self.state.player_mode == Mode::Order {
                        self.state.player_mode = Mode::Chaos;
                    } else {
                        self.state.player_mode = Mode::Order;
                    }
                }
                KeyCode::Return => {
                    self.state.play_state = PlayState::Playing(Turn::Human(SelectingPiece));
                }
                _ => {}
            }
        } else if self.state.play_state.is_human(SelectingPiece) {
            if !self.state.cursor.handle_input(key)
                && key == KeyCode::Return
                && self.state.board[self.state.cursor.idx] == Empty
            {
                self.state.play_state = PlayState::Playing(Turn::Human(SelectingMove));
            }
        } else if self.state.play_state.is_human(SelectingMove) {
            match key {
                KeyCode::Left | KeyCode::Right => {
                    if self.state.move_cursor == Mode::Order {
                        self.state.move_cursor = Mode::Chaos;
                    } else {
                        self.state.move_cursor = Mode::Order;
                    }
                }
                KeyCode::Return => {
                    self.process_move();
                }
                _ => {}
            }
        }
    }

    fn on_key_up(&mut self, key: KeyCode) -> bool {
        if self.state.play_state.is_human(SelectingMove) && key == KeyCode::Escape {
            self.state.play_state = NEW_TURN_HUMAN;
            return true;
        }
        false
    }

    fn update(&mut self, delta: f64) -> GameResult<()> {
        if self.state.play_state.is_computer(SelectingPiece) {
            self.state.next_move_time -= delta;
            if self.state.next_move_time < 0. {
                self.state.next_move_time = ANIMATION_DURATION;
                self.state.last_human_cursor_pos = self.state.cursor.idx;
                self.state.last_human_placed = self.state.board[self.state.cursor.idx];
                process(&mut self.state);
                self.state.play_state = PlayState::Playing(Turn::Computer(SelectingMove))
            }
        } else if self.state.play_state.is_computer(SelectingMove) {
            self.state.next_move_time -= delta;
            if self.state.next_move_time < 0. {
                self.process_move();
                self.state.cursor.idx = self.state.last_human_cursor_pos;
                self.state.move_cursor = self.state.last_human_placed.into();
            }
        }
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context, mesh_helper: &mut MeshHelper) -> GameResult<()> {
        render(ctx, mesh_helper, &mut self.state)
    }

    fn play_state(&self) -> PlayState {
        self.state.play_state
    }
}
