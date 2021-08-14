use crate::boards::cursor::Cursor;
use crate::boards::idx_coord::BoardCoord;
use crate::boards::set_board_size;
use crate::chess::game_types::get_chess_game_type;
use crate::chess::renderer::render;
use crate::chess::{Move, State};
use crate::constants::AI_MOVE_DELAY;
use crate::system::find_nearest::find_nearest;
use crate::system::ggez_ext::keycode_to_direction;
use crate::system::mesh_helper::MeshHelper;
use crate::system::Player::{Computer, Human};
use crate::system::TurnState::{SelectingMove, SelectingPiece};
use crate::system::{PlayState, Player, Scene, Turn, NEW_TURN_COMPUTER, NEW_TURN_HUMAN};
use ggez::event::KeyCode;
use ggez::{Context, GameResult};
use std::collections::HashMap;

pub(crate) struct Controller {
    state: State,
}

impl Controller {
    pub fn new(game: &str) -> Self {
        let game_type = get_chess_game_type(game);
        set_board_size(game_type.get_board_size());

        debug_log!("Game board set to {:?}", game_type.get_board_size());

        let mut captured = HashMap::new();
        captured.insert(Player::Human, Vec::new());
        captured.insert(Player::Computer, Vec::new());

        Controller {
            state: State {
                play_state: PlayState::Init,
                piece_cursor: Cursor::new(),
                move_cursor: 0,
                board: game_type.get_init_board(),
                move_history: vec![],
                all_possible_moves: HashMap::new(),
                game_type,
                next_move_time: AI_MOVE_DELAY,
                last_human_cursor_pos: 0,
                captured,
                moves_left_this_turn: 0,
            },
        }
    }
}

impl Controller {
    fn start_new_turn(&mut self, player: Player, full_turn: bool) {
        match player {
            Human => self.state.play_state = NEW_TURN_HUMAN,
            Computer => self.state.play_state = NEW_TURN_COMPUTER,
        }
        self.state.next_move_time = AI_MOVE_DELAY;
        self.state.move_cursor = 0;
        if full_turn {
            self.state.moves_left_this_turn = self.state.game_type.get_moves_for_turn(&self.state);
        }
        self.state.all_possible_moves = self
            .state
            .game_type
            .calc_valid_moves(&self.state.board, player);
    }

    fn process_move(&mut self, mov: &Move) {
        let player = self.state.board[mov.from]
            .get_player()
            .expect("Attempted to process move for non existent piece");

        self.state
            .game_type
            .process_move(&mut self.state.board, mov);

        self.state.moves_left_this_turn -= 1;
        if self.state.moves_left_this_turn == 0 {
            self.start_new_turn(player.opposite(), true)
        } else {
            self.start_new_turn(player, false)
        }
    }

    fn select_piece(&mut self) {
        let highlighted_piece = self.state.board[self.state.piece_cursor.idx];
        if highlighted_piece.get_player() == Some(Human)
            && !self.state.get_moves_for_selected_piece().is_empty()
        {
            self.state.play_state = PlayState::Playing(Turn::Human(SelectingMove));
        }
    }

    fn check_board_size(&self) {
        //This is necessary has the board had to be a list
        //And the length might accidentally be changed
        if self.state.game_type.get_board_cell_count() != self.state.board.len() {
            panic!(
                "Game board corrupted, now has {} cells",
                self.state.board.len()
            );
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
                        &|mov| BoardCoord::from(mov.to),
                    );
                    if let Some(nearest) = nearest {
                        debug_log!("Found: {:?}", nearest);
                        self.state.move_cursor = nearest;
                    } else {
                        debug_log!("Nothing found");
                    }
                }
                KeyCode::Return => {} //self.select_move(),
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

    fn update(&mut self, _: f64) -> GameResult<()> {
        self.check_board_size();

        if self.state.play_state == PlayState::Init {
            self.start_new_turn(Human, true);
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
