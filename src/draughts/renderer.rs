use crate::boards::idx_coord::BoardCoord;
use crate::boards::{board_cols, board_rows};
use crate::constants::colors::{
    alpha, CREAM, DARK_GREEN, LIGHT_BLUE, PIECE_COMPUTER, PIECE_PLAYER, TRANSPARENT,
};
use crate::draughts::moves::Move::*;
use crate::draughts::{board_index_to_pdn_num, Square, State};
use crate::system::letter_mesh::make_letter_mesh;
use crate::system::math::{pt, Offset, OffsetTuple, Point};
use crate::system::mesh_helper::MeshHelper;
use crate::system::TurnState::{SelectingMove, SelectingPiece};
use ggez::graphics::{Color, DrawMode, Mesh};
use ggez::{Context, GameResult};
use std::rc::Rc;

pub const PLAYER_MOVE_PIECE: Color = Color {
    r: 0.65,
    g: 0.8,
    b: 0.6,
    a: 1.,
};
pub const COMPUTER_MOVE_PIECE: Color = Color {
    r: 0.45,
    g: 0.5,
    b: 0.25,
    a: 1.,
};

pub(super) fn render(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    state: &mut State,
) -> GameResult<()> {
    let board_size = mesh_helper.calc_height(0.9);
    let cell_size = board_size / (state.board_calc.rows as f32);
    let grid = mesh_helper.make_grid(
        ctx,
        board_size,
        board_size,
        state.board_calc.cols,
        state.board_calc.rows,
        0.,
        TRANSPARENT,
        Some([CREAM, DARK_GREEN]),
    )?;
    let grid_box = mesh_helper.make_rect(ctx, board_size, board_size, DrawMode::stroke(3.))?;

    let board_start = pt(
        mesh_helper.width - board_size - mesh_helper.calc_height(0.05),
        mesh_helper.calc_height(0.05),
    );
    let numbering_start = board_start.offset(-24., board_size + cell_size * 0.45);
    let lettering_start = board_start.offset(-cell_size * 0.5, board_size + 8.);

    let piece = mesh_helper.make_circle(ctx, cell_size, cell_size * 0.4, DrawMode::fill())?;
    let king = make_letter_mesh(ctx, mesh_helper, cell_size * 0.8, 'k')?;

    mesh_helper.draw_mesh(ctx, grid.as_ref(), board_start);
    mesh_helper.draw_mesh(ctx, grid_box.as_ref(), board_start);

    for i in 1..=board_cols() {
        mesh_helper.draw_white_text(
            ctx,
            &format!("{}", i),
            numbering_start.offsety(-(i as f32 * cell_size)),
            20.,
            false,
        );
        mesh_helper.draw_white_text(
            ctx,
            &format!("{}", (i + 96) as u8 as char),
            lettering_start.offsetx(i as f32 * cell_size),
            20.,
            false,
        );
    }

    for x in 0..board_cols() {
        for y in 0..board_rows() {
            mesh_helper.draw_text(
                ctx,
                &format!("{}", board_index_to_pdn_num(x + y * board_cols())),
                board_start
                    .offset(x as f32 * cell_size, y as f32 * cell_size)
                    .offset(3, 3),
                CREAM,
                14.,
                false,
            );
        }
    }

    state.board.iter().enumerate().for_each(|(i, item)| {
        let xy = Point::from(BoardCoord::from(i))
            .multiply(cell_size, cell_size)
            .offset_point(board_start);

        match item {
            Square::ComputerMan => {
                mesh_helper.draw_coloured_mesh(ctx, piece.as_ref(), xy, PIECE_COMPUTER)
            }
            Square::ComputerKing => {
                mesh_helper.draw_coloured_mesh(ctx, piece.as_ref(), xy, PIECE_COMPUTER);
                mesh_helper.draw_mesh(
                    ctx,
                    king.as_ref(),
                    xy.offset(cell_size * 0.1, cell_size * 0.1),
                );
            }
            Square::HumanMan => {
                mesh_helper.draw_coloured_mesh(ctx, piece.as_ref(), xy, PIECE_PLAYER)
            }
            Square::HumanKing => {
                mesh_helper.draw_coloured_mesh(ctx, piece.as_ref(), xy, PIECE_PLAYER);
                mesh_helper.draw_mesh(
                    ctx,
                    king.as_ref(),
                    xy.offset(cell_size * 0.1, cell_size * 0.1),
                );
            }
            Square::Empty => {}
        }
    });

    if state.play_state.is_human(SelectingPiece) {
        state
            .piece_cursor
            .render(ctx, mesh_helper, board_start, cell_size)?;

        let color = match state.board[state.piece_cursor.idx] {
            Square::ComputerMan | Square::ComputerKing => COMPUTER_MOVE_PIECE,
            Square::HumanMan | Square::HumanKing => PLAYER_MOVE_PIECE,
            Square::Empty => TRANSPARENT,
        };

        for moves in state.get_moves_for_selected_piece() {
            match moves {
                Step {
                    origin: _,
                    dest,
                    value: _,
                } => {
                    draw_piece(
                        ctx,
                        mesh_helper,
                        dest,
                        cell_size,
                        alpha(color, 0.5),
                        piece.clone(),
                        board_start,
                    );
                }
                Jump {
                    origin: _,
                    capture,
                    value: _,
                } => {
                    draw_piece(
                        ctx,
                        mesh_helper,
                        capture.dest,
                        cell_size,
                        alpha(color, 0.5),
                        piece.clone(),
                        board_start,
                    );
                }
                MultiJump {
                    origin: _,
                    captures,
                    value: _,
                } => {
                    for capture in captures {
                        draw_piece(
                            ctx,
                            mesh_helper,
                            capture.dest,
                            cell_size,
                            alpha(color, 0.5),
                            piece.clone(),
                            board_start,
                        );
                    }
                }
            }
        }
    }

    if state.play_state.is_either(SelectingMove) {
        let mut pts: Vec<Point> = vec![];
        match state.get_selected_move() {
            Step {
                origin,
                dest,
                value: _,
            } => {
                pts.push(BoardCoord::from(origin).into());
                pts.push(BoardCoord::from(dest).into());
            }
            Jump {
                origin,
                capture,
                value: _,
            } => {
                pts.push(BoardCoord::from(origin).into());
                pts.push(BoardCoord::from(capture.dest).into());
            }
            MultiJump {
                origin,
                captures,
                value: _,
            } => {
                pts.push(BoardCoord::from(origin).into());
                for cap in captures {
                    pts.push(BoardCoord::from(cap.dest).into());
                }
            }
        }

        pts = pts
            .iter()
            .map(|pt| {
                pt.multiply(cell_size, cell_size)
                    .offset(cell_size * 0.5, cell_size * 0.5)
            })
            .collect();

        let mesh = mesh_helper.make_one_time_polyline(ctx, pts, 3.)?;
        mesh_helper.draw_coloured_mesh(ctx, mesh.as_ref(), board_start, LIGHT_BLUE);
    }

    let moves = state
        .move_history
        .iter()
        .enumerate()
        .map(|(idx, mov)| format!("{: >2}. {}", idx, mov))
        .collect::<Vec<String>>()
        .join("\n");

    let offset = if state.move_history.len() > 80 {
        (state.move_history.len() - 80) as f32 * 12.
    } else {
        0.
    };

    mesh_helper.draw_white_text(ctx, &moves, pt(8., 8. - offset), 12., false);

    Ok(())
}

fn draw_piece(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    idx: usize,
    cell_size: f32,
    color: Color,
    mesh: Rc<Mesh>,
    board_start: Point,
) {
    let xy = Point::from(BoardCoord::from(idx))
        .multiply(cell_size, cell_size)
        .offset_point(board_start);
    mesh_helper.draw_coloured_mesh(ctx, mesh.as_ref(), xy, color);
}
