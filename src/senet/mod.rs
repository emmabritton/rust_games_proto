use crate::boards::cursor::Cursor;
use crate::system::{PlayState, Player};
use std::collections::HashMap;

mod ai;
pub mod controller;
mod renderer;
mod rules;

const MAX_STICKS_UP: usize = 4;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
enum Square {
    Empty,
    Human,
    Computer,
}

impl From<Player> for Square {
    fn from(player: Player) -> Self {
        match player {
            Player::Human => Square::Human,
            Player::Computer => Square::Computer,
        }
    }
}

impl Square {
    fn player(&self) -> Option<Player> {
        match self {
            Square::Empty => None,
            Square::Human => Some(Player::Human),
            Square::Computer => Some(Player::Computer),
        }
    }
}

// 0  1  2  3  4  5  6  7  8  9
//19 18 17 16 15 14 13 12 11 10
//20 21 22 23 24 25 26 27 28 29
type Board = [Square; 30];

#[derive(Debug, Clone, Eq, PartialEq)]
struct Move {
    origin: usize,
    dest: usize,
    exchange: bool,
    value: usize,
}

impl Move {
    pub fn new(origin: usize, dest: usize, exchange: bool) -> Self {
        Move {
            origin,
            dest,
            exchange,
            value: 0,
        }
    }
}

struct State {
    play_state: PlayState,
    msg: Option<String>,
    cursor: Cursor,
    board: Board,
    move_cursor: usize,
    roll: Option<usize>,
    next_move_time: f64,
    last_human_cursor_pos: usize,
    valid_moves: HashMap<usize, Vec<Move>>,
}

impl State {
    //Safe, will return empty vec for empty/other players squares
    fn get_moves_for_selected_piece(&self) -> Vec<Move> {
        self.valid_moves
            .get(&self.cursor.idx)
            .unwrap_or(&vec![])
            .clone()
    }

    //Not safe, will panic if called when no available moves
    fn get_selected_move(&self) -> Move {
        self.get_moves_for_selected_piece()
            .get(self.move_cursor)
            .unwrap_or_else(|| {
                panic!(
                    "No move {} for square {}",
                    self.move_cursor, self.cursor.idx
                )
            })
            .clone()
    }
}

mod init {
    use super::Square::Computer as C;
    use super::Square::Empty as E;
    use super::Square::Human as H;
    use crate::senet::Board;

    #[rustfmt::skip]
    pub(super) const INIT_BOARD: Board = [
        C, H, C, H, C, H, C, H, C, H, 
        E, E, E, E, E, E, E, E, E, E, 
        E, E, E, E, E, E, E, E, E, E,
    ];
}
