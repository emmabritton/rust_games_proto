use crate::mancala::{Hole, Square, HOME_COUNT};
use crate::system::Player;

#[derive(Debug)]
pub(super) struct Board {
    pub(super) human: SubBoard,
    pub(super) computer: SubBoard,
}

#[derive(Default, Debug)]
pub(super) struct SubBoard {
    pub(super) end: usize,
    pub(super) homes: [usize; 6],
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct DropMove {
    pub(super) origin: Square,
    pub(super) current_square: Square,
    pub(super) remaining: usize,
}

impl DropMove {
    fn new(current_square: Square, remaining: usize) -> Self {
        let mut drop_move = DropMove {
            origin: current_square.clone(),
            current_square,
            remaining,
        };
        drop_move.move_to_next_square();
        drop_move
    }
}

impl DropMove {
    pub(super) fn move_to_next_square(&mut self) {
        self.current_square = match self.current_square.hole {
            Hole::End => Square::new(self.current_square.player.opposite(), Hole::Home(0)),
            Hole::Home(idx) => {
                if idx < HOME_COUNT - 1 {
                    Square::new(self.current_square.player, Hole::Home(idx + 1))
                } else if self.current_square.player == self.origin.player {
                    Square::new(self.current_square.player, Hole::End)
                } else {
                    Square::new(self.current_square.player.opposite(), Hole::Home(0))
                }
            }
        };
    }
}

//square order:
//  |12|11|10| 9| 8| 7|
//13|-----------------| 6
//  | 0| 1| 2| 3| 4| 5|

//**Home number**
//| 5| 4| 3| 2| 1| 0|
//|-----------------|
//| 0| 1| 2| 3| 4| 5|

impl Board {
    pub(super) fn new() -> Self {
        Board {
            human: SubBoard::default(),
            computer: SubBoard::default(),
        }
    }
}

impl Board {
    pub(super) fn add_count(&mut self, square: &Square, amount: usize) {
        match square.player {
            Player::Human => match square.hole {
                Hole::Home(idx) => self.human.homes[idx] += amount,
                Hole::End => self.human.end += amount,
            },
            Player::Computer => match square.hole {
                Hole::Home(idx) => self.computer.homes[idx] += amount,
                Hole::End => self.computer.end += amount,
            },
        }
    }

    pub(super) fn collect_remaining(&mut self) {
        self.add_count(
            &Square::new(Player::Human, Hole::End),
            self.human.home_total(),
        );
        self.add_count(
            &Square::new(Player::Computer, Hole::End),
            self.computer.home_total(),
        );
        self.human.fill_homes(0);
        self.computer.fill_homes(0);
    }

    pub(super) fn set_count(&mut self, square: &Square, amount: usize) {
        match square.player {
            Player::Human => match square.hole {
                Hole::Home(idx) => self.human.homes[idx] = amount,
                Hole::End => self.human.end = amount,
            },
            Player::Computer => match square.hole {
                Hole::Home(idx) => self.computer.homes[idx] = amount,
                Hole::End => self.computer.end = amount,
            },
        }
    }

    pub(super) fn get_count(&self, square: &Square) -> usize {
        match square.player {
            Player::Human => match square.hole {
                Hole::Home(idx) => self.human.homes[idx],
                Hole::End => self.human.end,
            },
            Player::Computer => match square.hole {
                Hole::Home(idx) => self.computer.homes[idx],
                Hole::End => self.computer.end,
            },
        }
    }

    pub(super) fn create_drop_move(&self, start: &Square) -> DropMove {
        DropMove::new(start.clone(), self.get_count(start))
    }

    pub(super) fn home_idx_to_square(&self, player: Player, idx: usize) -> Square {
        match player {
            Player::Human => self.idx_to_square(idx),
            Player::Computer => self.idx_to_square(7 + idx),
        }
    }

    pub(super) fn idx_to_square(&self, idx: usize) -> Square {
        match idx {
            0..=5 => Square::new(Player::Human, Hole::Home(idx)),
            6 => Square::new(Player::Human, Hole::End),
            7..=12 => Square::new(Player::Computer, Hole::Home(idx - 7)),
            13 => Square::new(Player::Computer, Hole::End),
            _ => panic!("Invalid idx for square: {}", idx),
        }
    }

    pub(super) fn square_to_idx(&self, square: &Square) -> usize {
        match square.player {
            Player::Human => match square.hole {
                Hole::Home(idx) => idx,
                Hole::End => 6,
            },
            Player::Computer => match square.hole {
                Hole::Home(idx) => 7 + idx,
                Hole::End => 13,
            },
        }
    }
}

impl SubBoard {
    pub(super) fn score(&self) -> usize {
        self.end
    }

    pub(super) fn home_total(&self) -> usize {
        self.homes.iter().sum::<usize>()
    }

    pub(super) fn fill_homes(&mut self, num: usize) {
        self.homes.iter_mut().for_each(|home| *home = num)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subboard_fill_holes() {
        let mut subboard = SubBoard {
            end: 0,
            homes: [0, 0, 0, 0, 0, 0],
        };

        subboard.fill_homes(2);

        assert_eq!(subboard.homes[0], 2);
        assert_eq!(subboard.homes[1], 2);
        assert_eq!(subboard.homes[2], 2);
        assert_eq!(subboard.homes[3], 2);
        assert_eq!(subboard.homes[4], 2);
        assert_eq!(subboard.homes[5], 2);
    }

    #[test]
    fn test_board_square_to_idx() {
        let board = Board::new();

        assert_eq!(
            board.square_to_idx(&Square::new(Player::Human, Hole::Home(0))),
            0
        );
        assert_eq!(
            board.square_to_idx(&Square::new(Player::Human, Hole::Home(1))),
            1
        );
        assert_eq!(
            board.square_to_idx(&Square::new(Player::Human, Hole::Home(2))),
            2
        );
        assert_eq!(
            board.square_to_idx(&Square::new(Player::Human, Hole::Home(3))),
            3
        );
        assert_eq!(
            board.square_to_idx(&Square::new(Player::Human, Hole::Home(4))),
            4
        );
        assert_eq!(
            board.square_to_idx(&Square::new(Player::Human, Hole::Home(5))),
            5
        );
        assert_eq!(
            board.square_to_idx(&Square::new(Player::Human, Hole::End)),
            6
        );
        assert_eq!(
            board.square_to_idx(&Square::new(Player::Computer, Hole::Home(0))),
            7
        );
        assert_eq!(
            board.square_to_idx(&Square::new(Player::Computer, Hole::Home(1))),
            8
        );
        assert_eq!(
            board.square_to_idx(&Square::new(Player::Computer, Hole::Home(2))),
            9
        );
        assert_eq!(
            board.square_to_idx(&Square::new(Player::Computer, Hole::Home(3))),
            10
        );
        assert_eq!(
            board.square_to_idx(&Square::new(Player::Computer, Hole::Home(4))),
            11
        );
        assert_eq!(
            board.square_to_idx(&Square::new(Player::Computer, Hole::Home(5))),
            12
        );
        assert_eq!(
            board.square_to_idx(&Square::new(Player::Computer, Hole::End)),
            13
        );
    }

    #[test]
    fn test_board_idx_to_square() {
        let board = Board::new();

        assert_eq!(
            board.idx_to_square(0),
            Square::new(Player::Human, Hole::Home(0))
        );
        assert_eq!(
            board.idx_to_square(1),
            Square::new(Player::Human, Hole::Home(1))
        );
        assert_eq!(
            board.idx_to_square(2),
            Square::new(Player::Human, Hole::Home(2))
        );
        assert_eq!(
            board.idx_to_square(3),
            Square::new(Player::Human, Hole::Home(3))
        );
        assert_eq!(
            board.idx_to_square(4),
            Square::new(Player::Human, Hole::Home(4))
        );
        assert_eq!(
            board.idx_to_square(5),
            Square::new(Player::Human, Hole::Home(5))
        );
        assert_eq!(
            board.idx_to_square(6),
            Square::new(Player::Human, Hole::End)
        );
        assert_eq!(
            board.idx_to_square(7),
            Square::new(Player::Computer, Hole::Home(0))
        );
        assert_eq!(
            board.idx_to_square(8),
            Square::new(Player::Computer, Hole::Home(1))
        );
        assert_eq!(
            board.idx_to_square(9),
            Square::new(Player::Computer, Hole::Home(2))
        );
        assert_eq!(
            board.idx_to_square(10),
            Square::new(Player::Computer, Hole::Home(3))
        );
        assert_eq!(
            board.idx_to_square(11),
            Square::new(Player::Computer, Hole::Home(4))
        );
        assert_eq!(
            board.idx_to_square(12),
            Square::new(Player::Computer, Hole::Home(5))
        );
        assert_eq!(
            board.idx_to_square(13),
            Square::new(Player::Computer, Hole::End)
        );
    }

    #[test]
    fn test_board_home_idx_to_square() {
        let board = Board::new();

        assert_eq!(
            board.home_idx_to_square(Player::Human, 0),
            Square::new(Player::Human, Hole::Home(0))
        );
        assert_eq!(
            board.home_idx_to_square(Player::Human, 1),
            Square::new(Player::Human, Hole::Home(1))
        );
        assert_eq!(
            board.home_idx_to_square(Player::Human, 2),
            Square::new(Player::Human, Hole::Home(2))
        );
        assert_eq!(
            board.home_idx_to_square(Player::Human, 3),
            Square::new(Player::Human, Hole::Home(3))
        );
        assert_eq!(
            board.home_idx_to_square(Player::Human, 4),
            Square::new(Player::Human, Hole::Home(4))
        );
        assert_eq!(
            board.home_idx_to_square(Player::Human, 5),
            Square::new(Player::Human, Hole::Home(5))
        );

        assert_eq!(
            board.home_idx_to_square(Player::Computer, 0),
            Square::new(Player::Computer, Hole::Home(0))
        );
        assert_eq!(
            board.home_idx_to_square(Player::Computer, 1),
            Square::new(Player::Computer, Hole::Home(1))
        );
        assert_eq!(
            board.home_idx_to_square(Player::Computer, 2),
            Square::new(Player::Computer, Hole::Home(2))
        );
        assert_eq!(
            board.home_idx_to_square(Player::Computer, 3),
            Square::new(Player::Computer, Hole::Home(3))
        );
        assert_eq!(
            board.home_idx_to_square(Player::Computer, 4),
            Square::new(Player::Computer, Hole::Home(4))
        );
        assert_eq!(
            board.home_idx_to_square(Player::Computer, 5),
            Square::new(Player::Computer, Hole::Home(5))
        );
    }

    #[test]
    fn test_drop_move_to_next_square_for_human() {
        let mut drop_move = DropMove::new(Square::new(Player::Human, Hole::Home(0)), 0);

        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Human, Hole::Home(1))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Human, Hole::Home(2))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Human, Hole::Home(3))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Human, Hole::Home(4))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Human, Hole::Home(5))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Human, Hole::End)
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Computer, Hole::Home(0))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Computer, Hole::Home(1))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Computer, Hole::Home(2))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Computer, Hole::Home(3))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Computer, Hole::Home(4))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Computer, Hole::Home(5))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Human, Hole::Home(0))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Human, Hole::Home(1))
        );
    }

    #[test]
    fn test_drop_move_to_next_square_for_computer() {
        let mut drop_move = DropMove::new(Square::new(Player::Computer, Hole::Home(0)), 0);

        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Computer, Hole::Home(1))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Computer, Hole::Home(2))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Computer, Hole::Home(3))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Computer, Hole::Home(4))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Computer, Hole::Home(5))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Computer, Hole::End)
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Human, Hole::Home(0))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Human, Hole::Home(1))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Human, Hole::Home(2))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Human, Hole::Home(3))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Human, Hole::Home(4))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Human, Hole::Home(5))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Computer, Hole::Home(0))
        );

        drop_move.move_to_next_square();
        assert_eq!(
            drop_move.current_square,
            Square::new(Player::Computer, Hole::Home(1))
        );
    }

    #[test]
    fn test_board_create_drop_move() {
        let mut board = Board::new();

        let square1 = Square::new(Player::Human, Hole::Home(0));
        let square2 = Square::new(Player::Human, Hole::Home(5));
        let square3 = Square::new(Player::Computer, Hole::Home(0));
        let square4 = Square::new(Player::Computer, Hole::Home(4));

        board.set_count(&square1, 3);
        board.set_count(&square2, 8);
        board.set_count(&square3, 5);
        board.set_count(&square4, 1);

        let drop_move_1 = board.create_drop_move(&square1);
        let drop_move_2 = board.create_drop_move(&square2);
        let drop_move_3 = board.create_drop_move(&square3);
        let drop_move_4 = board.create_drop_move(&square4);

        assert_eq!(
            drop_move_1,
            DropMove {
                origin: Square::new(Player::Human, Hole::Home(0)),
                current_square: Square::new(Player::Human, Hole::Home(1)),
                remaining: 3
            }
        );
        assert_eq!(
            drop_move_2,
            DropMove {
                origin: Square::new(Player::Human, Hole::Home(5)),
                current_square: Square::new(Player::Human, Hole::End),
                remaining: 8
            }
        );
        assert_eq!(
            drop_move_3,
            DropMove {
                origin: Square::new(Player::Computer, Hole::Home(0)),
                current_square: Square::new(Player::Computer, Hole::Home(1)),
                remaining: 5
            }
        );
        assert_eq!(
            drop_move_4,
            DropMove {
                origin: Square::new(Player::Computer, Hole::Home(4)),
                current_square: Square::new(Player::Computer, Hole::Home(5)),
                remaining: 1
            }
        );
    }

    #[test]
    fn test_board_collect_remaining() {
        let mut board1 = Board::new();
        let mut board2 = Board::new();
        let mut board3 = Board::new();

        board1.human.homes[3] = 2;
        board1.human.homes[4] = 1;
        board1.human.end = 1;
        board2.computer.homes[3] = 2;
        board2.computer.homes[4] = 10;
        board2.computer.end = 5;
        board3.human.homes[0] = 2;
        board3.human.homes[1] = 3;
        board3.computer.end = 4;

        board1.collect_remaining();
        board2.collect_remaining();
        board3.collect_remaining();

        assert_eq!(board1.human.score(), 4);
        assert_eq!(board1.human.end, 4);
        assert_eq!(board1.human.home_total(), 0);
        assert_eq!(board1.computer.score(), 0);
        assert_eq!(board1.computer.end, 0);
        assert_eq!(board1.computer.home_total(), 0);

        assert_eq!(board2.human.score(), 0);
        assert_eq!(board2.human.end, 0);
        assert_eq!(board2.human.home_total(), 0);
        assert_eq!(board2.computer.score(), 17);
        assert_eq!(board2.computer.end, 17);
        assert_eq!(board2.computer.home_total(), 0);

        assert_eq!(board3.human.score(), 5);
        assert_eq!(board3.human.end, 5);
        assert_eq!(board3.human.home_total(), 0);
        assert_eq!(board3.computer.score(), 4);
        assert_eq!(board3.computer.end, 4);
        assert_eq!(board3.computer.home_total(), 0);
    }
}
