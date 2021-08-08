use crate::draughts::moves::Move;
use crate::draughts::rules::flying_kings_both_direction_jumping::FlyingKingsBothDirectionJumping;
use crate::draughts::rules::init::*;
use crate::draughts::rules::no_flying_kings_single_direction_men::NoFlyingKingsSingleDirectionMen;
use crate::draughts::Square;
use crate::draughts::Square::*;
use crate::draughts::{Board, PastMove};
use crate::system::PlayState;
use std::collections::HashMap;

const VALUE_CAPTURE: usize = 10;
const VALUE_STEP: usize = 1;

lazy_static! {
    pub(super) static ref CAPTURABLE: HashMap<Square, Vec<Square>> = {
        let mut map: HashMap<Square, Vec<Square>> = HashMap::new();

        map.insert(HumanMan, vec![ComputerMan, ComputerKing]);
        map.insert(HumanKing, vec![ComputerMan, ComputerKing]);
        map.insert(ComputerMan, vec![HumanMan, HumanKing]);
        map.insert(ComputerKing, vec![HumanMan, HumanKing]);

        map
    };
}

mod common;
mod flying_kings_both_direction_jumping;
mod no_flying_kings_single_direction_men;

enum MoveDir {
    Up,
    Down,
    Both,
}

pub(super) trait RuleSet {
    fn calc_valid_moves(&self, board: &Board, origin: usize) -> Vec<Move>;
    fn check_game_over(&self, board: &Board, move_history: &Vec<PastMove>) -> Option<PlayState>;
    fn is_promotion(&self, board: &Board, origin: usize, dest: usize) -> Option<Square>;
}

mod init {
    use crate::draughts::Board;
    use crate::draughts::Square::ComputerMan as C;
    use crate::draughts::Square::Empty as E;
    use crate::draughts::Square::HumanMan as P;

    #[rustfmt::skip]
    pub(super) fn get_8x8_init_board() -> Board {
        vec![
            E, C, E, C, E, C, E, C, 
            C, E, C, E, C, E, C, E, 
            E, C, E, C, E, C, E, C, 
            E, E, E, E, E, E, E, E, 
            E, E, E, E, E, E, E, E, 
            P, E, P, E, P, E, P, E, 
            E, P, E, P, E, P, E, P, 
            P, E, P, E, P, E, P, E,
        ]
    }

    #[rustfmt::skip]
    pub(super) fn get_10x10_init_board() -> Board {
        vec![
            E, C, E, C, E, C, E, C, E, C, 
            C, E, C, E, C, E, C, E, C, E, 
            E, C, E, C, E, C, E, C, E, C, 
            C, E, C, E, C, E, C, E, C, E, 
            E, E, E, E, E, E, E, E, E, E, 
            E, E, E, E, E, E, E, E, E, E, 
            E, P, E, P, E, P, E, P, E, P, 
            P, E, P, E, P, E, P, E, P, E, 
            E, P, E, P, E, P, E, P, E, P, 
            P, E, P, E, P, E, P, E, P, E,
        ]
    }

    #[rustfmt::skip]
    pub(super) fn get_12x12_init_board() -> Board {
        vec![
            E, C, E, C, E, C, E, C, E, C, E, C, 
            C, E, C, E, C, E, C, E, C, E, C, E, 
            E, C, E, C, E, C, E, C, E, C, E, C, 
            C, E, C, E, C, E, C, E, C, E, C, E, 
            E, C, E, C, E, C, E, C, E, C, E, C, 
            E, E, E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E, E, E, 
            P, E, P, E, P, E, P, E, P, E, P, E, 
            E, P, E, P, E, P, E, P, E, P, E, P, 
            P, E, P, E, P, E, P, E, P, E, P, E, 
            E, P, E, P, E, P, E, P, E, P, E, P, 
            P, E, P, E, P, E, P, E, P, E, P, E,
        ]
    }
}

pub(super) enum GameVariant {
    English,
    International,
    Canadian,
    Brazilian,
}

impl GameVariant {
    pub(super) fn get_board_size(&self) -> (usize, usize) {
        match self {
            GameVariant::English => (8, 8),
            GameVariant::International => (10, 10),
            GameVariant::Canadian => (12, 12),
            GameVariant::Brazilian => (8, 8),
        }
    }

    pub(super) fn get_init_board(&self) -> Board {
        match self {
            GameVariant::English | GameVariant::Brazilian => get_8x8_init_board(),
            GameVariant::International => get_10x10_init_board(),
            GameVariant::Canadian => get_12x12_init_board(),
        }
    }

    pub(super) fn get_rules(&self) -> Box<dyn RuleSet> {
        match self {
            GameVariant::English => Box::new(NoFlyingKingsSingleDirectionMen::new()),
            GameVariant::International | GameVariant::Canadian | GameVariant::Brazilian => {
                Box::new(FlyingKingsBothDirectionJumping::new())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::boards::set_board_size;
    use crate::system::math::next_step;
    use crate::system::neighbours::get_neighbours;
    use serial_test::serial;

    #[test]
    #[serial(board_size)]
    fn test_next_step() {
        set_board_size((8, 8));

        assert_eq!(next_step(27, 18), Some(9));
        assert_eq!(next_step(27, 20), Some(13));
        assert_eq!(next_step(27, 36), Some(45));
        assert_eq!(next_step(27, 34), Some(41));
    }

    #[test]
    #[serial(board_size)]
    fn test_invalid_next_step() {
        set_board_size((8, 8));

        assert_eq!(next_step(10, 1), None);
        assert_eq!(next_step(14, 7), None);
        assert_eq!(next_step(49, 56), None);
        assert_eq!(next_step(46, 55), None);
    }

    #[test]
    #[serial(board_size)]
    fn test_get_neighbours() {
        set_board_size((8, 8));

        let top_left_neighbours = get_neighbours(0, false, true);
        assert_eq!(top_left_neighbours.len(), 1);
        assert_eq!(top_left_neighbours[0], 9);

        let bottom_left_neighbours = get_neighbours(56, false, true);
        assert_eq!(bottom_left_neighbours.len(), 1);
        assert_eq!(bottom_left_neighbours[0], 49);

        let top_right_neighbours = get_neighbours(7, false, true);
        assert_eq!(top_right_neighbours.len(), 1);
        assert_eq!(top_right_neighbours[0], 14);

        let bottom_right_neighbours = get_neighbours(63, false, true);
        assert_eq!(bottom_right_neighbours.len(), 1);
        assert_eq!(bottom_right_neighbours[0], 54);

        let bottom_right_neighbours = get_neighbours(35, false, true);
        assert_eq!(bottom_right_neighbours.len(), 4);
        assert_eq!(bottom_right_neighbours[0], 26);
        assert_eq!(bottom_right_neighbours[1], 28);
        assert_eq!(bottom_right_neighbours[2], 42);
        assert_eq!(bottom_right_neighbours[3], 44);
    }

    #[test]
    #[serial(board_size)]
    fn test_next_step_10() {
        set_board_size((10, 10));

        assert_eq!(next_step(27, 16), Some(5));
        assert_eq!(next_step(27, 18), Some(9));
        assert_eq!(next_step(27, 36), Some(45));
        assert_eq!(next_step(27, 38), Some(49));
    }

    #[test]
    #[serial(board_size)]
    fn test_invalid_next_step_10() {
        set_board_size((10, 10));

        assert_eq!(next_step(12, 1), None);
        assert_eq!(next_step(18, 7), None);
        assert_eq!(next_step(18, 9), None);
        assert_eq!(next_step(61, 70), None);
        assert_eq!(next_step(78, 89), None);
    }

    #[test]
    #[serial(board_size)]
    fn test_get_neighbours_10() {
        set_board_size((10, 10));

        let top_left_neighbours = get_neighbours(1, false, true);
        assert_eq!(top_left_neighbours.len(), 2);
        assert_eq!(top_left_neighbours[0], 10);
        assert_eq!(top_left_neighbours[1], 12);

        let bottom_left_neighbours = get_neighbours(58, false, true);
        assert_eq!(bottom_left_neighbours.len(), 4);
        assert_eq!(bottom_left_neighbours[0], 47);
        assert_eq!(bottom_left_neighbours[1], 49);
        assert_eq!(bottom_left_neighbours[2], 67);
        assert_eq!(bottom_left_neighbours[3], 69);

        let top_right_neighbours = get_neighbours(9, false, true);
        assert_eq!(top_right_neighbours.len(), 1);
        assert_eq!(top_right_neighbours[0], 18);

        let bottom_right_neighbours = get_neighbours(70, false, true);
        assert_eq!(bottom_right_neighbours.len(), 2);
        assert_eq!(bottom_right_neighbours[0], 61);
        assert_eq!(bottom_right_neighbours[1], 81);

        let bottom_right_neighbours = get_neighbours(35, false, true);
        assert_eq!(bottom_right_neighbours.len(), 4);
        assert_eq!(bottom_right_neighbours[0], 24);
        assert_eq!(bottom_right_neighbours[1], 26);
        assert_eq!(bottom_right_neighbours[2], 44);
        assert_eq!(bottom_right_neighbours[3], 46);
    }
}
