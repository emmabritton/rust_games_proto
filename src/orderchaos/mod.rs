use crate::boards::cursor::Cursor;
use crate::system::PlayState;

mod ai;
pub mod controller;
mod render_mode_selection;
mod renderer;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Square {
    Red,
    White,
    Empty,
}

impl Square {
    fn opposite(&self) -> Option<Square> {
        match self {
            Square::Red => Some(Square::White),
            Square::White => Some(Square::Red),
            Square::Empty => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Mode {
    Order,
    Chaos,
}

impl From<Mode> for Square {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Order => Square::White,
            Mode::Chaos => Square::Red,
        }
    }
}

impl From<Square> for Mode {
    fn from(square: Square) -> Self {
        match square {
            Square::Red => Mode::Chaos,
            Square::White => Mode::Order,
            Square::Empty => panic!("Can't convert Empty to Mode"),
        }
    }
}

type Board = [Square; 36];

struct State {
    board: Board,
    play_state: PlayState,
    cursor: Cursor,
    last_human_cursor_pos: usize,
    last_human_placed: Square,
    move_cursor: Mode,
    player_mode: Mode,
    next_move_time: f64,
}
