use crate::chess::rules::ChessPiece;
use crate::chess::{Board, Move, MoveFlags, PastMove, Square, State};
use crate::constants::games::*;
use crate::system::Player;
use std::collections::HashMap;

pub(super) fn get_chess_game_type(game: &str) -> GameType {
    match game {
        CHESS_STANDARD => GameType::Standard,
        CHESS_PROGRESSIVE => GameType::Progressive,
        CHESS_CAPABLANCA => GameType::Capablanca,
        CHESS_MODERN => GameType::Modern,
        CHESS_ANDERNACH => GameType::Andernach,
        CHESS_CHECKLESS => GameType::Checkless,
        CHESS_HOSTAGE => GameType::Hostage,
        _ => panic!("Unsupported game type: {}", game),
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum GameType {
    Standard,
    Progressive,
    Capablanca,
    Modern,
    Checkless,
    Andernach,
    Hostage,
}

impl GameType {
    pub(super) fn get_init_board(&self) -> Board {
        match self {
            GameType::Capablanca => init::CHESS_CAPABLANCA.clone(),
            GameType::Modern => init::CHESS_MODERN.clone(),
            _ => init::CHESS_STANDARD.clone(),
        }
    }

    pub(super) fn get_board_size(&self) -> (usize, usize) {
        match self {
            GameType::Modern => (9, 9),
            GameType::Capablanca => (8, 10),
            _ => (8, 8),
        }
    }

    pub(super) fn get_piece_value(&self, piece: &ChessPiece) -> usize {
        default_piece_value_conversion(piece)
    }

    pub(super) fn get_piece_letter(&self, piece: &ChessPiece) -> char {
        match self {
            GameType::Modern => {
                if piece == &ChessPiece::KnightBishop {
                    'm'
                } else {
                    default_piece_letter_conversion(piece)
                }
            }
            _ => default_piece_letter_conversion(piece),
        }
    }

    pub(super) fn get_moves_for_turn(&self, state: &State) -> usize {
        match self {
            GameType::Progressive => state.move_history.len(),
            _ => 1,
        }
    }

    pub(super) fn get_board_cell_count(&self) -> usize {
        match self {
            GameType::Capablanca => 80,
            GameType::Modern => 81,
            _ => 64,
        }
    }

    pub(super) fn calc_valid_moves(
        &self,
        board: &Board,
        player: Player,
    ) -> HashMap<usize, Vec<Move>> {
        let mut map = HashMap::new();

        for i in 0..self.get_board_cell_count() {
            if board[i].get_player() == Some(player) {
                map.insert(i, self.calc_moves(board, player, i));
            }
        }

        map
    }

    pub(super) fn play_move(&self, state: &mut State, mov: &Move) {
        let player = state.board[mov.from]
            .get_player()
            .expect("Attempted to play move for non existent piece");
        let piece = state.board[mov.from]
            .get_piece()
            .expect("Attempted to play move for non existent piece");

        // if self == &GameType::HOSTAGE {
        //     if let Some(piece) = state.board[mov.to].get_piece() {
        //         state
        //             .captured
        //             .get(&player.opposite())
        //             .expect("Missing capture for player")
        //             .push(piece)
        //     }
        // }

        self.process_move(&mut state.board, mov);

        //TODO delete?
        // let in_check = self.is_king_in_check(&state.board, player.opposite());
        // let mut in_checkmate = false;
        // if in_check {
        //     in_checkmate = self.is_king_in_checkmate(&state.board, player.opposite());
        // }

        let mut flags = vec![];
        if self.is_king_in_check(&state.board, player.opposite()) {
            if self.is_king_in_checkmate(&state.board, player.opposite()) {
                flags.push(MoveFlags::CheckMate)
            } else {
                flags.push(MoveFlags::Check)
            }
        }

        state.move_history.push(PastMove {
            player,
            start: mov.from,
            end: mov.to,
            piece,
            flags,
        })
    }

    pub(super) fn process_move(&self, board: &mut Board, mov: &Move) {
        match self {
            GameType::Andernach => {
                if let Some(opposite_player) = board[mov.to].get_player() {
                    let square = match opposite_player {
                        Player::Human => Square::Human(board[mov.from].get_piece().unwrap()),
                        Player::Computer => Square::Computer(board[mov.from].get_piece().unwrap()),
                    };
                    board[mov.to] = square;
                    board[mov.from] = Square::Empty;
                } else {
                    board[mov.to] = board[mov.from];
                    board[mov.from] = Square::Empty;
                }
            }
            _ => {
                board[mov.to] = board[mov.from];
                board[mov.from] = Square::Empty;
            }
        }
    }

    pub(super) fn is_king_in_check(&self, board: &Board, player_to_check: Player) -> bool {
        if self == &GameType::Checkless {
            false
        } else {
            false //TODO calc
        }
    }

    pub(super) fn is_king_in_checkmate(&self, board: &Board, player_to_check: Player) -> bool {
        false
    }

    fn calc_moves(&self, board: &Board, player: Player, origin: usize) -> Vec<Move> {
        return board[origin]
            .get_piece()
            .unwrap_or_else(|| panic!("Square has player but no piece: {}", origin))
            .calc_moves(self, board, origin);
    }
}

fn default_piece_value_conversion(piece: &ChessPiece) -> usize {
    match piece {
        ChessPiece::Pawn => 1,
        ChessPiece::Knight => 10,
        ChessPiece::Bishop | ChessPiece::Rook => 20,
        ChessPiece::KnightRook | ChessPiece::KnightBishop => 50,
        ChessPiece::Queen => 80,
        ChessPiece::King => 100,
    }
}

fn default_piece_letter_conversion(piece: &ChessPiece) -> char {
    match piece {
        ChessPiece::Pawn => 'p',
        ChessPiece::Rook => 'r',
        ChessPiece::Knight => 'n',
        ChessPiece::Bishop => 'b',
        ChessPiece::Queen => 'q',
        ChessPiece::King => 'k',
        ChessPiece::KnightBishop => 'a',
        ChessPiece::KnightRook => 'c',
    }
}

mod init {
    use crate::chess::rules::ChessPiece::Bishop as B;
    use crate::chess::rules::ChessPiece::King as K;
    use crate::chess::rules::ChessPiece::Knight as N;
    use crate::chess::rules::ChessPiece::KnightBishop as KB;
    use crate::chess::rules::ChessPiece::KnightRook as KR;
    use crate::chess::rules::ChessPiece::Pawn as P;
    use crate::chess::rules::ChessPiece::Queen as Q;
    use crate::chess::rules::ChessPiece::Rook as R;
    use crate::chess::Board;
    use crate::chess::Square::Computer as C;
    use crate::chess::Square::Empty as E;
    use crate::chess::Square::Human as H;

    #[rustfmt::skip]
    lazy_static! {
        pub(super) static ref CHESS_STANDARD: Board = vec![
            E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E,
            E, C(Q), E, E, E, E, E, E,
            E, E, E, E, E, E, E, E,
            E, E, E, H(Q), E, E, E, E,
            E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E,
        ];
        
        // pub(super) static ref CHESS_STANDARD: Board = vec![
        //     C(R), C(N), C(B), C(Q), C(K), C(B), C(N), C(R),
        //     C(P), C(P), C(P), C(P), C(P), C(P), C(P), C(P),
        //     E, E, E, E, E, E, E, E,
        //     E, E, E, E, E, E, E, E,
        //     E, E, E, E, E, E, E, E,
        //     E, E, E, E, E, E, E, E,
        //     H(P), H(P), H(P), H(P), H(P), H(P), H(P), H(P),
        //     H(R), H(N), H(B), H(Q), H(K), H(B), H(N), H(R)
        // ];

        pub(super) static ref CHESS_MODERN: Board = vec![
            C(R), C(N), C(B), C(Q), C(K), C(KB), C(B), C(N), C(R),
            C(P), C(P), C(P), C(P), C(P), C(P), C(P), C(P), C(P),
            E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E,
            H(P), H(P), H(P), H(P), H(P), H(P), H(P), H(P), H(P),
            H(R), H(N), H(B), H(KB), H(K), H(Q), H(B), H(N), H(R)
        ];

        pub(super) static ref CHESS_CAPABLANCA: Board = vec![
            C(R), C(N), C(KB), C(B), C(Q), C(K), C(B), C(KR), C(N), C(R),
            C(P), C(P), C(P), C(P), C(P), C(P), C(P), C(P), C(P), C(P),
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            E, E, E, E, E, E, E, E, E, E,
            H(P), H(P), H(P), H(P), H(P), H(P), H(P), H(P), H(P), H(P),
            H(R), H(N), H(KB), H(B), H(Q), H(K), H(B), H(KR), H(N), H(R)
        ];
    }
}
