use crate::boards::idx_coord::BoardCoord;
use crate::boards::is_in_board::IsInBoard;
use crate::chess::game_types::GameType;
use crate::chess::{Board, Move, Square};
use crate::system::Player;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum ChessPiece {
    Pawn,
    Bishop,
    Rook,
    Knight,
    Queen,
    King,
    KnightBishop,
    KnightRook,
}

impl ChessPiece {
    pub(super) fn calc_moves(
        &self,
        game_type: &GameType,
        board: &Board,
        origin: usize,
    ) -> Vec<Move> {
        match self {
            ChessPiece::Pawn => calc_pawn(game_type, board, origin),
            ChessPiece::Bishop => calc_bishop(board, origin),
            ChessPiece::Rook => calc_rook(board, origin),
            ChessPiece::Knight => calc_knight(board, origin),
            ChessPiece::Queen => [
                &calc_bishop(board, origin)[..],
                &calc_rook(board, origin)[..],
            ]
            .concat(),
            ChessPiece::King => calc_king(game_type, board, origin),
            ChessPiece::KnightBishop => [
                &calc_knight(board, origin)[..],
                &calc_bishop(board, origin)[..],
            ]
            .concat(),
            ChessPiece::KnightRook => [
                &calc_knight(board, origin)[..],
                &calc_rook(board, origin)[..],
            ]
            .concat(),
        }
    }
}

fn is_capturable(from: Square, target: Square) -> bool {
    if let Some(origin_player) = from.get_player() {
        target.get_player() == None || target.get_player() == Some(origin_player.opposite())
    } else {
        panic!("Checked capturable from empty square");
    }
}

fn clone_board_with_move(game_type: &GameType, board: &Board, mov: &Move) -> Board {
    let mut board = board.clone();
    game_type.process_move(&mut board, mov);
    board
}

fn does_move_result_in_self_check(game_type: &GameType, board: &Board, mov: &Move) -> bool {
    let player = board[mov.from]
        .get_player()
        .expect("No player on square for move");
    let board = clone_board_with_move(game_type, board, mov);
    game_type.is_king_in_check(&board, player)
}

fn check_line(board: &Board, origin: usize, x_diff: isize, y_diff: isize) -> Vec<Move> {
    let mut result = vec![];
    let mut target: (isize, isize) = BoardCoord::from(origin).into();
    loop {
        target.0 += x_diff;
        target.1 += y_diff;
        if target.is_in_board() {
            let idx = BoardCoord::from(target).idx();
            if is_capturable(board[origin], board[idx]) {
                result.push(Move::new(origin, idx));
                if board[idx] != Square::Empty {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    result
}

fn calc_king(game_type: &GameType, board: &Board, origin_idx: usize) -> Vec<Move> {
    debug_log_start!("Checking king moves for {}", origin_idx);
    let mut results = vec![];
    let origin: (isize, isize) = BoardCoord::from(origin_idx).into();
    for x in origin.0 - 1..=origin.0 + 1 {
        for y in origin.1 - 1..=origin.1 + 1 {
            if (x, y) != origin && (x, y).is_in_board() {
                let idx = BoardCoord::from((x, y)).idx();
                if is_capturable(board[origin_idx], board[idx]) {
                    let mov = Move::new(origin_idx, idx);
                    debug_log_start!("Found {}", mov);
                    if !does_move_result_in_self_check(game_type, board, &mov) {
                        results.push(mov);
                    } else {
                        debug_log!("But would self check");
                    }
                    debug_log_end!();
                } else {
                    debug_log!("{} blocked", idx);
                }
            }
        }
    }
    debug_log_end!();
    results
}

fn calc_pawn(game_type: &GameType, board: &Board, origin: usize) -> Vec<Move> {
    //TODO En passant
    let mut results = vec![];
    let origin_pos: (isize, isize) = BoardCoord::from(origin).into();
    match board[origin].get_player().expect("No player for calc pawn") {
        Player::Human => {
            let step_idx = BoardCoord::from((origin_pos.0, 5)).idx();
            let long_idx = BoardCoord::from((origin_pos.0, 4)).idx();
            if is_capturable(board[origin], board[step_idx]) {
                results.push(Move::new(origin, step_idx));
                if board[step_idx] == Square::Empty
                    && origin_pos.1 == 6
                    && is_capturable(board[origin], board[long_idx])
                {
                    results.push(Move::new(origin, long_idx));
                }
            }
        }
        Player::Computer => {
            let step_idx = BoardCoord::from((origin_pos.0, 2)).idx();
            let long_idx = BoardCoord::from((origin_pos.0, 3)).idx();
            if is_capturable(board[origin], board[step_idx]) {
                results.push(Move::new(origin, step_idx));
                if board[step_idx] == Square::Empty
                    && origin_pos.1 == 1
                    && is_capturable(board[origin], board[long_idx])
                {
                    results.push(Move::new(origin, long_idx));
                }
            }
        }
    }
    results
}

fn calc_kirin(board: &Board, origin: usize) -> Vec<Move> {
    vec![
        can_jump(board, origin, 0, 2),
        can_jump(board, origin, 1, 1),
        can_jump(board, origin, 2, 0),
        can_jump(board, origin, 1, -1),
        can_jump(board, origin, 0, -2),
        can_jump(board, origin, -1, -1),
        can_jump(board, origin, -2, 0),
        can_jump(board, origin, -1, 1),
    ]
    .into_iter()
    .filter_map(|item| item.map(|to| Move::new(origin, to)))
    .collect()
}

fn calc_elephant(game_type: &GameType, board: &Board, origin: usize) -> Vec<Move> {
    vec![
        can_jump(board, origin, 2, 2),
        can_jump(board, origin, -2, 2),
        can_jump(board, origin, 2, -2),
        can_jump(board, origin, -2, -2),
    ]
    .into_iter()
    .filter_map(|item| item.map(|to| Move::new(origin, to)))
    .chain(calc_king(game_type, board, origin).into_iter())
    .collect()
}

fn calc_superknight(board: &Board, origin: usize) -> Vec<Move> {
    check_jump_pattern(board, origin, 2, 1)
        .into_iter()
        .chain(check_jump_pattern(board, origin, 3, 1).into_iter())
        .chain(check_jump_pattern(board, origin, 3, 2).into_iter())
        .map(|to| Move::new(origin, to))
        .collect()
}

fn calc_rose(board: &Board, origin: usize) -> Vec<Move> {
    todo!("Not implemented as it requires another move type")
}

fn calc_bishop(board: &Board, origin: usize) -> Vec<Move> {
    let mut result = vec![];

    result.append(&mut check_line(board, origin, -1, -1));
    result.append(&mut check_line(board, origin, 1, 1));
    result.append(&mut check_line(board, origin, 1, -1));
    result.append(&mut check_line(board, origin, -1, 1));

    result
}

fn calc_rook(board: &Board, origin: usize) -> Vec<Move> {
    let mut result = vec![];

    result.append(&mut check_line(board, origin, -1, 0));
    result.append(&mut check_line(board, origin, 1, 0));
    result.append(&mut check_line(board, origin, 0, -1));
    result.append(&mut check_line(board, origin, 0, 1));

    result
}

fn calc_knight(board: &Board, origin: usize) -> Vec<Move> {
    check_jump_pattern(board, origin, 2, 1)
        .into_iter()
        .map(|to| Move::new(origin, to))
        .collect()
}

fn check_jump_pattern(board: &Board, origin: usize, vert: usize, horz: usize) -> Vec<usize> {
    let vert = vert as isize;
    let horz = horz as isize;

    vec![
        can_jump(board, origin, vert, horz),
        can_jump(board, origin, -vert, horz),
        can_jump(board, origin, -vert, -horz),
        can_jump(board, origin, vert, -horz),
        can_jump(board, origin, horz, vert),
        can_jump(board, origin, -horz, vert),
        can_jump(board, origin, -horz, -vert),
        can_jump(board, origin, horz, -vert),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn can_jump(board: &Board, origin: usize, vert: isize, horz: isize) -> Option<usize> {
    let mut pos: (isize, isize) = BoardCoord::from(origin).into();
    pos.0 += horz;
    pos.1 += vert;

    if pos.is_in_board() {
        let idx = BoardCoord::from(pos).idx();
        if is_capturable(board[origin], board[idx]) {
            return Some(idx);
        }
    }
    None
}
