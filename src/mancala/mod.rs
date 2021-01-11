use crate::mancala::board::Board;
use crate::mancala::board::DropMove;
use crate::mancala::Hole::Home;
use crate::system::math::WrappedUsize;
use crate::system::{PlayState, Player};

mod ai;
mod board;
pub mod controller;
mod render;

const HOME_COUNT: usize = 6;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Hole {
    Home(usize),
    End,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Square {
    player: Player,
    hole: Hole,
}

impl Square {
    fn new(player: Player, hole: Hole) -> Self {
        Square { player, hole }
    }
}

impl Square {
    fn is_home(&self) -> bool {
        matches!(self.hole, Home(_))
    }
}

struct State {
    play_state: PlayState,
    cursor: WrappedUsize,
    computer_cursor: usize,
    board: Board,
    drop_move: Option<DropMove>,
    next_move_time: f64,
    animation_time: f64,
    message: Option<(String, bool)>,
}
