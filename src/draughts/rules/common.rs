use crate::draughts::moves::{Capture, Move};
use crate::draughts::rules::VALUE_CAPTURE;
use crate::draughts::Square::*;
use crate::draughts::{Board, PastMove, Square};
use crate::system::Player;
use crate::tree::Leaf;

pub(super) fn calc_valid_moves<F>(board: &Board, origin: usize, moves_for_square: F) -> Vec<Move>
where
    F: Fn(&Board, usize, bool) -> Vec<Move>,
{
    debug_log_start!("Calculating valid moves for {}", origin);
    if board[origin] == Square::Empty {
        debug_log_end!("Board square empty");
        return vec![];
    }

    let mut first_leaf = create_move_tree(board, origin, &moves_for_square);
    let mut move_lists = vec![];
    first_leaf.print(0, "");
    loop {
        let move_list = first_leaf.list();
        debug_log!("Pruned and got {:?}", move_list);
        if !move_list.is_empty() {
            move_lists.push(move_list);
        }
        if first_leaf.prune() {
            break;
        }
    }
    if move_lists.is_empty() {
        debug_log_end!("No moves found");
        return vec![];
    }
    let longest = move_lists.iter().map(|list| list.len()).max().unwrap();
    debug_log!(
        "Found {} potential moves, longest: {}",
        move_lists.len(),
        longest
    );
    move_lists = move_lists
        .into_iter()
        .filter(|list| list.len() == longest)
        .collect();

    if longest > 1 {
        debug_log_end!("Found {} multijumps", move_lists.len());
        move_lists
            .into_iter()
            .map(|list| {
                let mut captures: Vec<Capture> = list
                    .iter()
                    .map(|mov| {
                        if let Move::Jump {
                            origin: _,
                            capture,
                            value: _,
                        } = mov
                        {
                            capture
                        } else {
                            panic!("Invalid move");
                        }
                    })
                    .cloned()
                    .collect();
                captures.reverse();
                let value = captures.len() * VALUE_CAPTURE;
                Move::MultiJump {
                    origin,
                    captures,
                    value,
                }
            })
            .collect()
    } else {
        debug_log_end!("Found {} steps/jumps", move_lists.len());
        move_lists
            .into_iter()
            .map(|mut list| list.remove(0))
            .collect()
    }
}

pub(super) fn create_move_tree<F>(board: &Board, origin: usize, moves_for_square: &F) -> Leaf<Move>
where
    F: Fn(&Board, usize, bool) -> Vec<Move>,
{
    debug_log!("Creating move tree from {}", origin);
    let mut first_leaf = make_leaf(board, origin, false, moves_for_square);
    let any_captures = first_leaf.contents.iter().any(|(mov, _)| mov.is_jump());
    if any_captures {
        debug_log!("At least one capture found");
        first_leaf.contents = first_leaf
            .contents
            .into_iter()
            .filter(|(mov, _)| mov.is_jump())
            .collect();
        debug_log!(
            "{} moves remaining after filtering",
            first_leaf.contents.len()
        );
    }
    first_leaf
}

pub(super) fn make_leaf<F>(
    board: &Board,
    origin: usize,
    capture_only: bool,
    moves_for_square: &F,
) -> Leaf<Move>
where
    F: Fn(&Board, usize, bool) -> Vec<Move>,
{
    debug_log!(
        "Creating leaf from {}, capture_only: {}",
        origin,
        capture_only
    );
    let potential_moves = moves_for_square(board, origin, capture_only);
    debug_log!("Found potential {} moves", potential_moves.len());
    let moves: Vec<(Move, Option<Leaf<Move>>)> = potential_moves
        .iter()
        .map(|mov| {
            let leaf = match mov {
                Move::Step { .. } => None,
                Move::Jump {
                    origin: _,
                    capture,
                    value: _,
                } => {
                    let mut new_board = board.clone();
                    new_board[capture.capturing] = Square::Empty;
                    let piece = new_board[origin];
                    new_board[origin] = Square::Empty;
                    new_board[capture.dest] = piece;
                    Some(make_leaf(&new_board, capture.dest, true, moves_for_square))
                }
                Move::MultiJump { .. } => panic!("Invalid move found at {}", origin),
            };
            (mov.clone(), leaf)
        })
        .collect();
    debug_log!(
        "Result: {} moves with {} leaves",
        moves.len(),
        moves.iter().filter(|pair| pair.1.is_some()).count()
    );
    Leaf::new(moves)
}

pub(super) fn check_moves<F>(move_history: &Vec<PastMove>, count: usize, predicate: F) -> bool
where
    F: Fn(&PastMove) -> bool,
{
    if move_history.len() >= count {
        move_history.iter().rev().take(count).all(predicate)
    } else {
        false
    }
}

pub(super) fn get_man_count(board: &Board, player: Player) -> usize {
    let piece = match player {
        Player::Human => HumanMan,
        Player::Computer => ComputerMan,
    };
    get_count(board, piece)
}

pub(super) fn get_king_count(board: &Board, player: Player) -> usize {
    let piece = match player {
        Player::Human => HumanKing,
        Player::Computer => ComputerKing,
    };
    get_count(board, piece)
}

pub(super) fn get_piece_count(board: &Board, player: Player) -> usize {
    get_man_count(board, player) + get_king_count(board, player)
}

fn get_count(board: &Board, piece: Square) -> usize {
    board.iter().filter(|&square| square == &piece).count()
}
