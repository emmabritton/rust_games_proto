mod ai;
pub mod controller;
mod render_mode_selection;
mod renderer;
mod rules;

use crate::boards::cursor::Cursor;
use crate::system::{PlayState, Player};
use itertools::Itertools;
use std::fmt;
use std::fmt::{Display, Formatter};

const CORNERS: [usize; 4] = [0, 8, 72, 80];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Mode {
    Attacker,
    Defender,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Square {
    Empty,
    King,
    Defender,
    Attacker,
}

type Board = [Square; 81];

#[derive(Debug, Clone, Eq, PartialEq)]
struct Move {
    origin: usize,
    dest: usize,
    capturing: Vec<usize>,
    value: usize,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let captures = if self.capturing.is_empty() {
            String::new()
        } else {
            format!(
                "x{}",
                self.capturing.iter().map(|num| num.to_string()).join("x")
            )
        };
        write!(f, "{}-{}{}", self.origin, self.dest, captures)
    }
}

struct State {
    board: Board,
    cursor: Cursor,
    play_state: PlayState,
    last_human_cursor_pos: usize,
    valid_moves: Vec<Move>,
    player_mode: Mode,
    next_move_time: f64,
    move_cursor: usize,
}

impl State {
    fn get_moves_for_selected_piece(&self) -> Vec<Move> {
        self.valid_moves
            .iter()
            .filter(|mov| mov.origin == self.cursor.idx)
            .cloned()
            .collect()
    }

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

    fn get_mode_for_player(&self, player: Player) -> Mode {
        match player {
            Player::Human => self.player_mode,
            Player::Computer => {
                if self.player_mode == Mode::Attacker {
                    Mode::Defender
                } else {
                    Mode::Attacker
                }
            }
        }
    }
}

mod init {
    use super::Square::Attacker as A;
    use super::Square::Defender as D;
    use super::Square::Empty as E;
    use super::Square::King as K;
    use crate::tablut::Board;

    #[rustfmt::skip]
    pub(super) const INIT_BOARD: Board = [
        E, E, E, A, A, A, E, E, E,
        E, E, E, E, A, E, E, E, E,
        E, E, E, E, D, E, E, E, E,
        A, E, E, E, D, E, E, E, A,
        A, A, D, D, K, D, D, A, A,
        A, E, E, E, D, E, E, E, A,
        E, E, E, E, D, E, E, E, E,
        E, E, E, E, A, E, E, E, E,
        E, E, E, A, A, A, E, E, E,
    ];
}
