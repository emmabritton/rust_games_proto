use crate::boards::cursor::Cursor;
use crate::boards::idx_coord::BoardCoord;
use crate::boards::set_board_size;
use crate::constants::{AI_MOVE_DELAY, ANIMATION_DURATION};
use crate::senet::ai::process;
use crate::senet::init::INIT_BOARD;
use crate::senet::renderer::render;
use crate::senet::rules::{calc_valid_moves, HOME, HOUSE_REBIRTH, HOUSE_WATER, REPEAT_TURN_ROLL};
use crate::senet::{State, MAX_STICKS_UP};
use crate::system::find_nearest::find_nearest;
use crate::system::ggez_ext::keycode_to_direction;
use crate::system::mesh_helper::MeshHelper;
use crate::system::Turn::Computer;
use crate::system::TurnState::{SelectingMove, SelectingPiece};
use crate::system::{PlayState, Player, Scene, Turn, TurnState, NEW_TURN_COMPUTER, NEW_TURN_HUMAN};
use ggez::event::KeyCode;
use ggez::{Context, GameResult};
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub struct Controller {
    state: State,
}

impl Controller {
    pub fn new() -> Self {
        set_board_size((3, 10));
        let mut cursor = Cursor::new();
        cursor.idx = 9;
        Controller {
            state: State {
                msg: None,
                board: INIT_BOARD,
                play_state: PlayState::Init,
                cursor,
                move_cursor: 0,
                roll: None,
                next_move_time: 0.0,
                last_human_cursor_pos: 0,
                valid_moves: HashMap::new(),
            },
        }
    }
}

impl Controller {
    fn start_new_turn(&mut self, player: Player, repeat_turn: bool) {
        debug_log_start!("Starting new turn for {:?}", player);
        self.state.roll = None;
        self.state.move_cursor = 0;
        self.state.next_move_time = AI_MOVE_DELAY;
        match player {
            Player::Human => {
                self.state.msg = Some(format!(
                    "Your turn{}, press return to throw the sticks",
                    if repeat_turn { " again" } else { "" }
                ));
                self.state.play_state = NEW_TURN_HUMAN
            }
            Player::Computer => {
                self.state.msg = Some(format!(
                    "Computers turn {}",
                    if repeat_turn { "again" } else { "" }
                ));
                self.state.play_state = NEW_TURN_COMPUTER
            }
        }
        debug_log_end!();
    }

    fn update_moves(&mut self, player: Player) {
        self.state.valid_moves =
            calc_valid_moves(&self.state.board, self.state.roll.unwrap(), player)
                .iter()
                .cloned()
                .into_group_map_by(|mov| mov.origin);
        debug_log_end!("{} movable pieces found", self.state.valid_moves.len())
    }

    fn roll(&mut self, player: Player) {
        let sticks = thread_rng().gen_range(0, MAX_STICKS_UP + 1);
        self.state.roll = Some(if sticks == 0 { 5 } else { sticks });
        debug_log!("Rolled {}", self.state.roll.unwrap_or(0));
        self.update_moves(player);
    }

    fn ai_update(&mut self) {
        if self.state.play_state.is_computer(SelectingPiece) {
            if self.state.roll.is_none() {
                self.roll(Player::Computer);
            } else {
                process(&mut self.state);
            }
        } else if self.state.play_state.is_computer(SelectingMove) {
            self.process_move();
            if REPEAT_TURN_ROLL.contains(&self.state.roll.unwrap()) {
                self.start_new_turn(Player::Computer, true);
            } else {
                self.state.cursor.idx = self.state.last_human_cursor_pos;
                self.start_new_turn(Player::Human, false);
            }
        }
    }

    fn process_move(&mut self) {
        let mut mov = self.state.get_selected_move();
        let origin = self.state.board[mov.origin];
        self.state.board[mov.origin] = self.state.board[mov.dest];
        if mov.dest == HOUSE_WATER {
            mov.dest = HOUSE_REBIRTH;
        }
        if mov.dest <= HOME {
            self.state.board[mov.dest] = origin;
        }
        if REPEAT_TURN_ROLL.contains(&self.state.roll.unwrap()) {
            self.start_new_turn(origin.player().unwrap(), true);
        } else {
            self.start_new_turn(origin.player().unwrap().opposite(), false);
        }
    }
}

impl Scene for Controller {
    fn on_key_down(&mut self, key: KeyCode) {
        if self.state.play_state.is_human(SelectingPiece) {
            if self.state.roll.is_none() {
                if key == KeyCode::Return {
                    self.roll(Player::Human);
                    self.state.msg = Some(String::from("Your turn"));
                }
            } else if !self.state.cursor.handle_input(key) {
                if key == KeyCode::Return && !self.state.get_moves_for_selected_piece().is_empty() {
                    self.state.play_state = PlayState::Playing(Turn::Human(SelectingMove));
                }
            }
        } else if self.state.play_state.is_human(SelectingMove) {
            if let Some(dir) = keycode_to_direction(key) {
                if let Some(new) = find_nearest(
                    &self.state.get_moves_for_selected_piece(),
                    self.state.move_cursor,
                    dir,
                    &|mov| BoardCoord::from(mov.origin),
                ) {
                    self.state.move_cursor = new;
                }
            } else if key == KeyCode::Return {
                self.process_move();
            }
        }
    }

    fn on_key_up(&mut self, key: KeyCode) -> bool {
        if key == KeyCode::Escape && self.state.play_state.is_human(SelectingMove) {
            self.state.play_state = NEW_TURN_HUMAN;
            return true;
        }
        false
    }

    fn update(&mut self, delta: f64) -> GameResult<()> {
        if self.state.play_state == PlayState::Init {
            self.start_new_turn(Player::Human, false);
        } else if self.state.play_state.player() == Some(Player::Computer) {
            self.state.next_move_time -= delta;
            if self.state.next_move_time < 0. {
                self.state.next_move_time = AI_MOVE_DELAY;
                if self.state.play_state.is_computer(TurnState::SelectingPiece)
                    && self.state.roll.is_none()
                {
                    self.roll(Player::Computer);
                    self.state.next_move_time = ANIMATION_DURATION;
                } else if self.state.play_state.is_computer(SelectingPiece) {
                    self.ai_update();
                    self.state.next_move_time = ANIMATION_DURATION;
                    self.state.play_state = PlayState::Playing(Computer(SelectingMove))
                } else if self.state.play_state.is_computer(SelectingMove) {
                    self.process_move();
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context, mesh_helper: &mut MeshHelper) -> GameResult<()> {
        render(ctx, mesh_helper, &self.state)
    }

    fn play_state(&self) -> PlayState {
        self.state.play_state
    }
}
