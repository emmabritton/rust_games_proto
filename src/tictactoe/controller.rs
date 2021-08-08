use crate::boards::cursor::Cursor;
use crate::boards::set_board_size;
use crate::constants::AI_MOVE_DELAY;
use crate::system::mesh_helper::MeshHelper;
use crate::system::TurnState::SelectingPiece;
use crate::system::{PlayState, Scene, NEW_TURN_COMPUTER, NEW_TURN_HUMAN};
use crate::tictactoe::ai::process;
use crate::tictactoe::renderer::render;
use crate::tictactoe::PlayState::{ComputerWin, Draw, HumanWin};
use crate::tictactoe::Square::E;
use crate::tictactoe::{Board, Square, State, COMPUTER_PIECE, PLAYER_PIECE};
use ggez::event::KeyCode;
use ggez::{Context, GameResult};

pub struct Controller {
    state: State,
}

impl Controller {
    pub fn new() -> Self {
        set_board_size((3, 3));
        Controller {
            state: State {
                board: [E; 9],
                next_move_time: 0.,
                cursor: Cursor::new(),
                play_state: NEW_TURN_HUMAN,
            },
        }
    }
}

impl Controller {
    fn select(&mut self) {
        if self.state.board[self.state.cursor.idx] == E {
            self.state.board[self.state.cursor.idx] = PLAYER_PIECE;
            self.state.next_move_time = AI_MOVE_DELAY;
            self.state.play_state = NEW_TURN_COMPUTER;
        }
    }

    fn check_for_win(&mut self) {
        if check_player_win(&self.state.board) {
            self.state.play_state = HumanWin
        } else if check_computer_win(&self.state.board) {
            self.state.play_state = ComputerWin
        } else if check_full(&self.state.board) {
            self.state.play_state = Draw
        }
    }
}

impl Scene for Controller {
    fn on_key_down(&mut self, key: KeyCode) {
        if self.state.play_state.is_human(SelectingPiece) {
            self.state.cursor.handle_input(key);
            if let KeyCode::Return = key {
                self.select();
            }
        }
    }

    fn update(&mut self, delta: f64) -> GameResult<()> {
        self.check_for_win();

        if self.state.play_state.is_computer(SelectingPiece) {
            self.state.next_move_time -= delta;
            if self.state.next_move_time < 0. {
                process(&mut self.state);
                self.state.play_state = NEW_TURN_HUMAN;
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

fn check_player_win(board: &Board) -> bool {
    check_board(board, PLAYER_PIECE)
}

fn check_computer_win(board: &Board) -> bool {
    check_board(board, COMPUTER_PIECE)
}

fn check_full(board: &Board) -> bool {
    board.iter().all(|s| s != &E)
}

fn check_board(board: &Board, piece: Square) -> bool {
    check_all_horz(board, 0, piece)
        || check_all_horz(board, 3, piece)
        || check_all_horz(board, 6, piece)
        || check_all_vert(board, 0, piece)
        || check_all_vert(board, 1, piece)
        || check_all_vert(board, 2, piece)
        || check_all(board, 0, 4, 8, piece)
        || check_all(board, 6, 4, 2, piece)
}

fn check_all(board: &Board, s1: usize, s2: usize, s3: usize, piece: Square) -> bool {
    board[s1] == board[s2] && board[s2] == board[s3] && board[s3] == piece
}

fn check_all_vert(board: &Board, start: usize, piece: Square) -> bool {
    check_all(board, start, start + 3, start + 6, piece)
}

fn check_all_horz(board: &Board, start: usize, piece: Square) -> bool {
    check_all(board, start, start + 1, start + 2, piece)
}
