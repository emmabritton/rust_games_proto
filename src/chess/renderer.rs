use crate::boards::idx_coord::BoardCoord;
use crate::boards::{board_cols, board_rows};
use crate::constants::colors::{
    alpha, APRICOT, BLACK, COPPER, CREAM, DARK_GREEN, LIGHT_BLUE, PIECE_COMPUTER, PIECE_PLAYER,
    TRANSPARENT, WHITE,
};

use crate::chess::State;
use crate::system::letter_mesh::make_letter_mesh;
use crate::system::math::{pt, pt_usize, Offset, OffsetTuple, Point};
use crate::system::mesh_helper::MeshHelper;
use crate::system::Player;
use crate::system::TurnState::{SelectingMove, SelectingPiece};
use ggez::graphics::{Color, DrawMode, Mesh};
use ggez::{Context, GameResult};
use std::rc::Rc;

pub const HUMAN_PIECE: Color = WHITE;
pub const COMPUTER_PIECE: Color = BLACK;

pub(super) fn render(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    state: &State,
) -> GameResult<()> {
    let cell_size = (mesh_helper.height * 0.9) / board_cols().max(board_rows()) as f32;
    let board_width = cell_size * board_cols() as f32;
    let board_height = cell_size * board_rows() as f32;
    let grid = mesh_helper.make_grid(
        ctx,
        board_width,
        board_height,
        board_cols(),
        board_rows(),
        0.,
        TRANSPARENT,
        Some([APRICOT, COPPER]),
    )?;
    let grid_box = mesh_helper.make_rect(ctx, board_width, board_height, DrawMode::stroke(3.))?;

    let board_start = pt(
        mesh_helper.width - board_width - mesh_helper.calc_height(0.05),
        mesh_helper.calc_height(0.05),
    );
    let numbering_start = board_start.offset(-20., board_height + cell_size * 0.44);
    let lettering_start = board_start.offset(-cell_size * 0.5, board_height + 8.);

    mesh_helper.draw_mesh(ctx, grid.as_ref(), board_start);
    mesh_helper.draw_mesh(ctx, grid_box.as_ref(), board_start);

    for i in 1..=board_rows() {
        mesh_helper.draw_white_text(
            ctx,
            &format!("{}", i),
            numbering_start.offsety(-(i as f32 * cell_size)),
            20.,
            true,
        );
    }
    for i in 1..=board_cols() {
        mesh_helper.draw_white_text(
            ctx,
            &format!("{}", (i + 96) as u8 as char),
            lettering_start.offsetx(i as f32 * cell_size),
            20.,
            true,
        );
    }

    for x in 0..board_cols() {
        for y in 0..board_rows() {
            let square = state.board[x + y * board_cols()];
            if let Some(piece) = square.get_piece() {
                let mesh = make_letter_mesh(
                    ctx,
                    mesh_helper,
                    cell_size,
                    state.game_type.get_piece_letter(&piece),
                )?;
                let pt = pt_usize(x, y)
                    .multiply(cell_size, cell_size)
                    .offset_point(board_start);
                let colour = match square.get_player().unwrap() {
                    Player::Human => HUMAN_PIECE,
                    Player::Computer => COMPUTER_PIECE,
                };
                mesh_helper.draw_coloured_mesh(ctx, mesh.as_ref(), pt, colour);
            }
        }
    }

    //TODO purpose?
    // state.board.iter().enumerate().for_each(|(i, item)| {
    //     let xy = Point::from(BoardCoord::from(i))
    //         .multiply(cell_size, cell_size)
    //         .offset_point(board_start);
    // });

    if state.play_state.is_human(SelectingPiece) {
        state
            .piece_cursor
            .render(ctx, mesh_helper, board_start, cell_size)?;

        let move_mesh = mesh_helper.make_rect(ctx, cell_size, cell_size, DrawMode::fill())?;

        for mov in state.get_moves_for_selected_piece() {
            let pt = Point::from(BoardCoord::from(mov.to))
                .multiply(cell_size, cell_size)
                .offset_point(board_start);
            mesh_helper.draw_coloured_mesh(ctx, move_mesh.as_ref(), pt, alpha(LIGHT_BLUE, 0.3));
        }
    }

    if state.play_state.is_either(SelectingMove) {
        let piece_mesh = mesh_helper.make_rect(ctx, cell_size, cell_size, DrawMode::fill())?;
        let move_mesh =
            mesh_helper.make_circle(ctx, cell_size * 0.2, cell_size * 0.2, DrawMode::fill())?;

        let pt = Point::from(BoardCoord::from(state.piece_cursor.idx))
            .multiply(cell_size, cell_size)
            .offset_point(board_start);
        mesh_helper.draw_coloured_mesh(ctx, piece_mesh.as_ref(), pt, alpha(LIGHT_BLUE, 0.3));
        for (i, mov) in state.get_moves_for_selected_piece().iter().enumerate() {
            let pt = Point::from(BoardCoord::from(mov.to))
                .multiply(cell_size, cell_size)
                .offset(cell_size * 0.4, cell_size * 0.4)
                .offset_point(board_start);
            let colour = if i == state.move_cursor {
                LIGHT_BLUE
            } else {
                WHITE
            };
            mesh_helper.draw_coloured_mesh(ctx, move_mesh.as_ref(), pt, colour);
        }
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
