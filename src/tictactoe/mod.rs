use crate::boards::cursor::Cursor;
use crate::system::PlayState;
use std::fmt;
use std::fmt::{Display, Formatter};

mod ai;
pub mod controller;
mod renderer;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Square {
    X,
    O,
    E,
}

const COMPUTER_PIECE: Square = Square::O;
const PLAYER_PIECE: Square = Square::X;

type Board = [Square; 9];

#[derive(Debug)]
struct State {
    board: Board,
    cursor: Cursor,
    next_move_time: f64,
    play_state: PlayState,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "State: {:?}\nSelected: {}\n{}",
            self.play_state,
            self.cursor.idx,
            board_to_string(&self.board)
        )
    }
}

fn board_to_string(board: &Board) -> String {
    format!(
        r" {}|{}|{}
 -----
 {}|{}|{}
 -----
 {}|{}|{}",
        board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]
    )
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Square::X => write!(f, "X"),
            Square::O => write!(f, "O"),
            Square::E => write!(f, " "),
        }
    }
}
