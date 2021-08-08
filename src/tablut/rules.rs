use crate::boards::idx_coord::BoardCoord;
use crate::boards::{board_cols, board_rows};
use crate::system::math::next_step;
use crate::system::neighbours::get_neighbours;
use crate::tablut::CORNERS;
use crate::tablut::{Board, Mode, Move, Square};
use rand::{thread_rng, Rng};
use std::collections::HashMap;

const CASTLE: usize = 40;

lazy_static! {
    static ref FORBIDDEN: HashMap<Square, Vec<usize>> = {
        let mut map = HashMap::<Square, Vec<usize>>::new();

        let mut non_king = CORNERS.to_vec();
        non_king.push(CASTLE);

        map.insert(Square::Defender, non_king.clone());
        map.insert(Square::Attacker, non_king);
        map.insert(Square::King, vec![CASTLE]);

        map
    };
    static ref ALLY: HashMap<Square, Vec<Square>> = {
        let mut map = HashMap::<Square, Vec<Square>>::new();

        map.insert(Square::Defender, vec![Square::Defender]);
        map.insert(Square::King, vec![]);
        map.insert(Square::Attacker, vec![Square::Attacker]);

        map
    };
    static ref CAPTURABLE: HashMap<Square, Vec<Square>> = {
        let mut map = HashMap::<Square, Vec<Square>>::new();

        map.insert(Square::Defender, vec![Square::Attacker]);
        map.insert(Square::King, vec![]);
        map.insert(Square::Attacker, vec![Square::King, Square::Defender]);

        map
    };
}

pub(super) fn calc_valid_moves(board: &Board, mode: Mode) -> Vec<Move> {
    debug_log_start!("Calcing valid moves for {:?}", mode);
    let valid_pieces = match mode {
        Mode::Defender => vec![Square::Defender, Square::King],
        Mode::Attacker => vec![Square::Attacker],
    };
    let pieces: Vec<usize> = board
        .iter()
        .enumerate()
        .filter_map(|(idx, square)| {
            if valid_pieces.contains(square) {
                Some(idx)
            } else {
                None
            }
        })
        .collect();
    debug_log!("{} pieces available", pieces.len());
    let result = pieces
        .iter()
        .map(|idx| moves_for_square(board, *idx))
        .flatten()
        .collect();
    debug_log_end!();
    result
}

fn moves_for_square(board: &Board, origin: usize) -> Vec<Move> {
    let mut moves = vec![];
    debug_log_start!("Finding moves for {}", origin);
    let allies = &ALLY[&board[origin]];
    get_neighbours(origin, true, false)
        .iter()
        .for_each(|neighbour| {
            let mut current = *neighbour;
            let mut next = next_step(origin, current);
            loop {
                let square = board[current];
                if square == Square::Empty {
                    let captures: Vec<usize> = get_neighbours(current, true, false)
                        .iter()
                        .filter_map(|neighbour| {
                            let next = next_step(current, *neighbour);
                            if let Some(next) = next {
                                if (allies.contains(&board[next])
                                    || next == CASTLE
                                    || CORNERS.contains(&next))
                                    && CAPTURABLE[&board[origin]].contains(&board[*neighbour])
                                {
                                    let neighbours = get_neighbours(*neighbour, true, false);
                                    let surrounded_count = neighbours
                                        .iter()
                                        .filter(|&square| {
                                            allies.contains(&board[*square])
                                                || CORNERS.contains(square)
                                                || CASTLE == *square
                                        })
                                        .count();
                                    let surrounded = surrounded_count == 3
                                        || (surrounded_count == 2 && neighbours.len() == 3);
                                    if board[*neighbour] != Square::King || surrounded {
                                        return Some(*neighbour);
                                    }
                                }
                            }
                            None
                        })
                        .collect();

                    if !FORBIDDEN[&board[origin]].contains(&current) {
                        let value = calc_value(board, origin, current, &captures);
                        debug_log!(
                            "Found move from {} to {} with {} captures (score: {})",
                            origin,
                            current,
                            captures.len(),
                            value
                        );
                        moves.push(Move {
                            origin,
                            dest: current,
                            capturing: captures,
                            value,
                        });
                    }
                    if let Some(next_square) = next {
                        next = next_step(current, next_square);
                        current = next_square;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        });
    debug_log_end!("Found {} moves for {}", moves.len(), origin);
    moves
}

fn calc_value(board: &Board, origin: usize, current: usize, captures: &[usize]) -> usize {
    debug_log_start!("Calculating value");
    let king_coord = board
        .iter()
        .enumerate()
        .filter_map(|(i, square)| {
            if square == &Square::King {
                Some(BoardCoord::from(i))
            } else {
                None
            }
        })
        .next();
    let origin_coord = BoardCoord::from(origin);
    let dest_coord = BoardCoord::from(current);
    let castle_coord = BoardCoord::from(CASTLE);
    let mut value = 0;
    if CORNERS.contains(&current) {
        debug_log!("Corner in one move +10000");
        value += 10000;
    }
    if board[origin] == Square::King
        && (dest_coord.0 == 0
            || dest_coord.1 == 0
            || dest_coord.0 == board_cols()
            || dest_coord.1 == board_rows())
    {
        debug_log!("Edge in one move +1000");
        value += 1000;
    }
    debug_log!("Captures +{}", captures.len() * 10);
    value += captures.len() * 10;
    for capture in captures.iter() {
        if board[*capture] == Square::King {
            debug_log!("Capture king +10000");
            value += 10000;
        }
    }
    for neighbour in get_neighbours(current, true, false) {
        if board[neighbour] == Square::King && board[origin] == Square::Attacker {
            debug_log!("Move next to king +20");
            value += 20;
        }
        if CORNERS.contains(&neighbour) {
            let neighbour_coord = BoardCoord::from(neighbour);
            value += if board[origin] == Square::King {
                debug_log!("Move next to corner (king) +100");
                100
            } else if let Some(king_coord) = king_coord {
                if (king_coord.0 < castle_coord.0 && neighbour_coord.0 < castle_coord.0)
                    && (king_coord.0 > castle_coord.0 && neighbour_coord.0 > castle_coord.0)
                    && (king_coord.1 < castle_coord.1 && neighbour_coord.1 < castle_coord.1)
                    && (king_coord.1 > castle_coord.1 && neighbour_coord.1 > castle_coord.1)
                {
                    debug_log!("Move next to corner (attacker) +10");
                    10
                } else {
                    debug_log!("Move next to corner (attacker) +1");
                    1
                }
            } else {
                0
            };
        }
    }
    if board[origin] == Square::King {
        let dist = (origin_coord.dist(dest_coord) as f32 * thread_rng().gen::<f32>()) as usize;
        value += dist;
        debug_log!("Distance +{}", dist);
        if (origin_coord.1 < castle_coord.1 && dest_coord.1 < origin_coord.1)
            || (origin_coord.1 > castle_coord.1 && dest_coord.1 > origin_coord.1)
            || (origin_coord.0 < castle_coord.0 && dest_coord.0 < origin_coord.0)
            || (origin_coord.0 > castle_coord.0 && dest_coord.0 > origin_coord.0)
        {
            debug_log!("Moving away from castle +1");
            value += 1;
        }
    }

    debug_log_end!();
    value
}
