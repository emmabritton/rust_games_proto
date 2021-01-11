use crate::boards::cursor::Cursor;
use crate::boards::idx_coord::BoardCoord;
use crate::boards::set_board_size;
use crate::constants::{AI_MOVE_DELAY, ANIMATION_DURATION};
use crate::system::find_nearest::find_nearest;
use crate::system::ggez_ext::keycode_to_direction;
use crate::system::mesh_helper::MeshHelper;
use crate::system::PlayState::{ComputerWin, Draw, HumanWin};
use crate::system::TurnState::{SelectingMove, SelectingPiece};
use crate::system::{PlayState, Player, Scene, Turn, NEW_TURN_COMPUTER, NEW_TURN_HUMAN};
use crate::tablut::ai::process;
use crate::tablut::init::INIT_BOARD;
use crate::tablut::renderer::render;
use crate::tablut::rules::calc_valid_moves;
use crate::tablut::{Mode, Square, State, CORNERS};
use ggez::event::KeyCode;
use ggez::{Context, GameResult};

pub struct Controller {
    state: State,
}

impl Controller {
    pub fn new() -> Self {
        set_board_size((9, 9));
        Controller {
            state: State {
                board: INIT_BOARD,
                cursor: Cursor::new(),
                play_state: PlayState::ModeSelection,
                last_human_cursor_pos: 0,
                valid_moves: vec![],
                move_cursor: 0,
                player_mode: Mode::Attacker,
                next_move_time: AI_MOVE_DELAY,
            },
        }
    }
}

impl Controller {
    fn start_new_turn(&mut self, player: Player) {
        debug_log_start!("Starting new game for {:?}", player);
        match player {
            Player::Human => self.state.play_state = NEW_TURN_HUMAN,
            Player::Computer => self.state.play_state = NEW_TURN_COMPUTER,
        }
        self.calc_valid_moves(self.state.get_mode_for_player(player));
        debug_log_end!("{} possible moves", self.state.valid_moves.len());
        self.check_for_game_over();
        self.state.next_move_time = AI_MOVE_DELAY;
        self.state.move_cursor = 0;
    }

    fn check_for_game_over(&mut self) {
        for corner in CORNERS.iter() {
            if self.state.board[*corner] == Square::King {
                self.state.play_state = match self.state.player_mode {
                    Mode::Attacker => {
                        debug_log!("King in corner, human was attacker: Computer wins!");
                        ComputerWin
                    }
                    Mode::Defender => {
                        debug_log!("King in corner, human was defender: Human wins!");
                        HumanWin
                    }
                };
                return;
            }
        }
        if !self
            .state
            .board
            .iter()
            .any(|square| square == &Square::King)
        {
            self.state.play_state = match self.state.player_mode {
                Mode::Attacker => {
                    debug_log!("No king on board, human was attacker: Human wins!");
                    HumanWin
                }
                Mode::Defender => {
                    debug_log!("No king on board, human was defender: Computer wins!");
                    ComputerWin
                }
            };
            return;
        }
        if self.state.valid_moves.is_empty() {
            debug_log!("No moves possible: Draw");
            self.state.play_state = Draw
        }
    }

    fn calc_valid_moves(&mut self, mode: Mode) {
        self.state.valid_moves = calc_valid_moves(&self.state.board, mode);
    }

    fn process_move(&mut self) {
        let mov = self.state.get_selected_move();
        self.state.board[mov.dest] = self.state.board[mov.origin];
        self.state.board[mov.origin] = Square::Empty;
        for capture in mov.capturing {
            self.state.board[capture] = Square::Empty;
        }
    }
}

impl Scene for Controller {
    fn on_key_down(&mut self, key: KeyCode) {
        if self.state.play_state == PlayState::ModeSelection {
            match key {
                KeyCode::Left | KeyCode::Right => {
                    if self.state.player_mode == Mode::Attacker {
                        self.state.player_mode = Mode::Defender;
                    } else {
                        self.state.player_mode = Mode::Attacker;
                    }
                }
                KeyCode::Return => self.state.play_state = PlayState::Init,
                _ => {}
            }
        } else if self.state.play_state.is_human(SelectingPiece) {
            if !self.state.cursor.handle_input(key)
                && key == KeyCode::Return
                && !self.state.get_moves_for_selected_piece().is_empty()
            {
                self.state.play_state = PlayState::Playing(Turn::Human(SelectingMove));
            }
        } else if self.state.play_state.is_human(SelectingMove) {
            match key {
                KeyCode::Left | KeyCode::Up | KeyCode::Right | KeyCode::Down => {
                    let dir = keycode_to_direction(key).unwrap();
                    let nearest = find_nearest(
                        &self.state.get_moves_for_selected_piece(),
                        self.state.move_cursor,
                        dir,
                        &|mov| BoardCoord::from(mov.dest),
                    );
                    if let Some(nearest) = nearest {
                        self.state.move_cursor = nearest;
                    }
                }
                KeyCode::Return => {
                    self.process_move();
                    self.start_new_turn(Player::Computer);
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
        if self.state.play_state == PlayState::Init {
            match self.state.player_mode {
                Mode::Attacker => self.start_new_turn(Player::Human),
                Mode::Defender => self.start_new_turn(Player::Computer),
            }
        } else if self.state.play_state == NEW_TURN_COMPUTER {
            self.state.next_move_time -= delta;
            if self.state.next_move_time < 0. {
                self.state.last_human_cursor_pos = self.state.cursor.idx;
                process(&mut self.state);
                self.state.next_move_time = ANIMATION_DURATION;
                self.state.play_state = PlayState::Playing(Turn::Computer(SelectingMove));
            }
        } else if self.state.play_state.is_computer(SelectingMove) {
            self.state.next_move_time -= delta;
            if self.state.next_move_time < 0. {
                self.process_move();
                self.state.cursor.idx = self.state.last_human_cursor_pos;
                self.start_new_turn(Player::Human);
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
