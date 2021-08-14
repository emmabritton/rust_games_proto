use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

use crate::boards::cursor::Cursor;
use crate::chess::game_types::GameType;
use crate::chess::rules::ChessPiece;
use crate::system::Player::{Computer, Human};
use crate::system::{PlayState, Player};

pub mod controller;
mod game_types;
mod renderer;
mod rules;

type Board = Vec<Square>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Square {
    Empty,
    Human(ChessPiece),
    Computer(ChessPiece),
}

impl Square {
    pub(super) fn get_piece(&self) -> Option<ChessPiece> {
        match self {
            Square::Empty => None,
            Square::Human(piece) => Some(*piece),
            Square::Computer(piece) => Some(*piece),
        }
    }

    pub(super) fn get_player(&self) -> Option<Player> {
        match self {
            Square::Empty => None,
            Square::Human(_) => Some(Human),
            Square::Computer(_) => Some(Computer),
        }
    }
}

#[derive(Debug)]
enum MoveFlags {
    EnPassant,
    Castling,
    Promotion(ChessPiece),
    Check,
    CheckMate,
}

#[derive(Debug)]
struct PastMove {
    player: Player,
    start: usize,
    end: usize,
    piece: ChessPiece,
    flags: Vec<MoveFlags>,
}

impl Display for PastMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Not implemented")
    }
}

#[derive(Debug)]
struct State {
    play_state: PlayState,
    piece_cursor: Cursor,
    move_cursor: usize,
    board: Board,
    move_history: Vec<PastMove>,
    game_type: GameType,
    next_move_time: f64,
    last_human_cursor_pos: usize,
    all_possible_moves: HashMap<usize, Vec<Move>>,
    captured: HashMap<Player, Vec<ChessPiece>>,
    moves_left_this_turn: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Move {
    from: usize,
    to: usize,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.from, self.to)
    }
}

impl Move {
    pub fn new(from: usize, to: usize) -> Self {
        Move { from, to }
    }
}

impl State {
    //Safe, will return empty vec for empty/other players squares
    fn get_moves_for_selected_piece(&self) -> Vec<Move> {
        self.all_possible_moves
            .get(&self.piece_cursor.idx)
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
                    self.move_cursor, self.piece_cursor.idx
                )
            })
            .clone()
    }
}
