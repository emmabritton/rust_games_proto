use crate::constants::{AI_MOVE_DELAY, ANIMATION_DURATION};
use crate::mancala::ai::process;
use crate::mancala::board::Board;
use crate::mancala::render::render;
use crate::mancala::{Hole, Square, State, HOME_COUNT};
use crate::system::math::WrappedUsize;
use crate::system::mesh_helper::MeshHelper;
use crate::system::PlayState::{ComputerWin, Draw, HumanWin, Playing};
use crate::system::TurnState::{Animating, SelectingPiece};
use crate::system::{PlayState, Player, Scene, NEW_TURN_COMPUTER, NEW_TURN_HUMAN};
use crate::system::{Turn, TurnState};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

pub struct Controller {
    state: State,
}

impl Controller {
    pub fn new() -> Self {
        let mut board = Board::new();
        board.computer.fill_holes(6);
        board.human.fill_holes(6);
        Controller {
            state: State {
                play_state: PlayState::Init,
                cursor: WrappedUsize::new_zero_based(6),
                computer_cursor: 0,
                drop_move: None,
                board,
                next_move_time: 0.0,
                animation_time: 0.0,
                message: None,
            },
        }
    }
}

impl Controller {
    pub(super) fn start_new_turn(&mut self, player: Player) {
        let old_state = self.state.play_state;
        debug_log_start!("\n\n----\nStarting new turn for {:?}", player);
        let (pronoun, again) = match player {
            Player::Human => {
                self.state.play_state = NEW_TURN_HUMAN;
                ("Your", old_state.is_human(TurnState::Animating))
            }
            Player::Computer => {
                self.state.play_state = NEW_TURN_COMPUTER;
                ("Computers", old_state.is_computer(TurnState::Animating))
            }
        };
        let again = if again { " again" } else { "" };
        self.state.message = Some((format!("{} turn{}", pronoun, again), false));
        self.state.next_move_time = AI_MOVE_DELAY;
        self.check_for_game_over();
        debug_log_end!();
    }

    fn check_for_game_over(&mut self) {
        if self.state.board.human.home_total() == 0 || self.state.board.computer.home_total() == 0 {
            debug_log!("A player has no stones left");
            debug_log_start!("Pre collection:");
            debug_log!(
                "Human: End {} Homes {}",
                self.state.board.human.end,
                self.state.board.human.home_total()
            );
            debug_log!(
                "Computer: End {} Homes {}",
                self.state.board.computer.end,
                self.state.board.computer.home_total()
            );
            debug_log_end!();
            self.state.board.collect_remaining();
            debug_log!(
                "Score: Human {}, Computer {}",
                self.state.board.human.score(),
                self.state.board.computer.score()
            );
            self.state.message = None;
            self.state.play_state = match self
                .state
                .board
                .human
                .score()
                .cmp(&self.state.board.computer.score())
            {
                Ordering::Greater => {
                    debug_log!("Human wins");
                    HumanWin
                }
                Ordering::Less => {
                    debug_log!("Computer wins");
                    ComputerWin
                }
                Ordering::Equal => {
                    debug_log!("Draw");
                    Draw
                }
            };
        }
    }

    fn process_turn(&mut self, player: Player) {
        let cursor = match player {
            Player::Human => self.state.cursor.value,
            Player::Computer => self.state.computer_cursor,
        };
        let square = self.state.board.idx_to_square(cursor);
        self.state.drop_move = Some(self.state.board.create_drop_move(&square));
        debug_log!(
            "Starting from {:?} with {} stones",
            square,
            self.state.board.get_count(&square)
        );
        self.state.board.set_count(&square, 0);
        self.state.play_state = Playing(Turn::new(player, Animating));
        self.state.animation_time = ANIMATION_DURATION;
    }
}

impl Scene for Controller {
    fn on_key_down(&mut self, key: KeyCode) {
        if self.state.play_state.is_human(SelectingPiece) {
            match key {
                KeyCode::Left => self.state.cursor.dec(),
                KeyCode::Right => self.state.cursor.inc(),
                KeyCode::Return => {
                    let square = self.state.board.idx_to_square(self.state.cursor.value);
                    if self.state.board.get_count(&square) > 0 {
                        self.process_turn(Player::Human);
                    }
                }
                _ => {}
            }
        }
    }

    fn update(&mut self, delta: f64) -> GameResult<()> {
        if self.state.play_state == PlayState::Init {
            if thread_rng().gen::<f32>() > 0.3 {
                self.start_new_turn(Player::Human);
            } else {
                self.start_new_turn(Player::Computer);
                self.state.next_move_time = 2.;
            }
        }
        if self.state.play_state.is_computer(SelectingPiece) {
            self.state.next_move_time -= delta;
            if self.state.next_move_time < 0.0 {
                process(&mut self.state);
                self.process_turn(Player::Computer);
            }
        }
        if self.state.play_state.is_computer(Animating) || self.state.play_state.is_human(Animating)
        {
            self.state.animation_time -= delta;
            if self.state.animation_time <= 0.0 {
                let mut drop_move = self.state.drop_move.as_mut().unwrap();
                if drop_move.remaining > 0 {
                    drop_move.remaining -= 1;
                    debug_log!(
                        "Deposit at {:?}, {} left",
                        drop_move.current_square,
                        drop_move.remaining
                    );
                    self.state.board.add_count(&drop_move.current_square, 1);
                    self.state.animation_time = ANIMATION_DURATION;
                    if drop_move.remaining > 0 {
                        drop_move.move_to_next_square();
                    } else if let Hole::Home(idx) = drop_move.current_square.hole {
                        if drop_move.current_square.player == drop_move.origin.player
                            && self.state.board.get_count(&drop_move.current_square) == 1
                        {
                            let opposite_square = Square::new(
                                drop_move.current_square.player.opposite(),
                                Hole::Home(HOME_COUNT - idx - 1),
                            );
                            if self.state.board.get_count(&opposite_square) > 0 {
                                self.state.message = Some((String::from("Capture!"), true));
                                self.state.animation_time = ANIMATION_DURATION * 2.5;
                                let count = self.state.board.get_count(&opposite_square)
                                    + self.state.board.get_count(&drop_move.current_square);
                                let end = Square::new(drop_move.origin.player, Hole::End);
                                self.state.board.set_count(&opposite_square, 0);
                                self.state.board.set_count(&drop_move.current_square, 0);
                                self.state.board.add_count(&end, count);
                            }
                        }
                    }
                } else {
                    let player = if !drop_move.current_square.is_home()
                        && drop_move.current_square.player == drop_move.origin.player
                    {
                        debug_log!("Ended in 'End' so having another turn");
                        drop_move.origin.player
                    } else {
                        drop_move.origin.player.opposite()
                    };
                    self.start_new_turn(player);
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
