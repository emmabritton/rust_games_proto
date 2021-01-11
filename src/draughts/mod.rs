use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

use crate::boards::chessboard::ChessBoard;
use crate::boards::cursor::Cursor;
use crate::draughts::moves::Move;
use crate::system::Player::{Computer, Human};
use crate::system::{PlayState, Player};

mod ai;
pub mod controller;
mod moves;
mod renderer;
mod rules;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Square {
    ComputerMan,
    ComputerKing,
    HumanMan,
    HumanKing,
    Empty,
}

impl Square {
    const fn is_king(&self) -> bool {
        matches!(self, Square::ComputerKing | Square::HumanKing)
    }
}

#[derive(Debug)]
struct PastMove {
    player: Player,
    start: usize,
    hops: Vec<usize>,
    king_capture_count: usize,
    man_capture_count: usize,
    piece: Square,
    promotion: bool,
}

impl PastMove {
    pub fn new(
        player: Player,
        start: usize,
        hops: Vec<usize>,
        king_capture_count: usize,
        man_capture_count: usize,
        piece: Square,
        promotion: bool,
    ) -> Self {
        PastMove {
            player,
            start: board_index_to_pdn_num(start),
            hops: hops
                .iter()
                .map(|num| board_index_to_pdn_num(*num))
                .collect(),
            king_capture_count,
            man_capture_count,
            piece,
            promotion,
        }
    }
}

impl Display for PastMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let player = match self.player {
            Player::Human => "H",
            Player::Computer => "C",
        };
        let mov = if self.man_capture_count + self.king_capture_count == 0 {
            format!("-{}", self.hops[0])
        } else if self.man_capture_count + self.king_capture_count == 1 {
            format!("x{}", self.hops[0])
        } else {
            self.hops
                .iter()
                .map(|cap| format!("x{}", cap))
                .collect::<Vec<String>>()
                .join("")
        };
        let promo = if self.promotion { "^" } else { "" };
        write!(f, "{} {}{}{}", player, self.start, mov, promo)
    }
}

type Board = Vec<Square>;

impl From<Square> for Player {
    fn from(square: Square) -> Self {
        match square {
            Square::ComputerMan | Square::ComputerKing => Computer,
            Square::HumanMan | Square::HumanKing => Human,
            Square::Empty => panic!("Can not convert empty to player"),
        }
    }
}

pub(super) fn board_index_to_pdn_num(idx: usize) -> usize {
    let mut result = (idx as f32 / 2.0).floor();
    if idx % 2 == 0 {
        result -= 1.0;
    }
    idx - result as usize
}

#[derive(Debug)]
struct State {
    board: Board,
    board_calc: ChessBoard,
    play_state: PlayState,
    piece_cursor: Cursor,
    all_possible_moves: HashMap<usize, Vec<Move>>,
    move_cursor: usize,
    move_history: Vec<PastMove>,
    next_move_time: f64,
    last_human_cursor_pos: usize,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_board_index_to_pdn_num() {
        assert_eq!(board_index_to_pdn_num(1), 1);
        assert_eq!(board_index_to_pdn_num(62), 32);
        assert_eq!(board_index_to_pdn_num(33), 17);
        assert_eq!(board_index_to_pdn_num(5), 3);
        assert_eq!(board_index_to_pdn_num(8), 5);
        assert_eq!(board_index_to_pdn_num(35), 18);
        assert_eq!(board_index_to_pdn_num(53), 27);
    }
}
