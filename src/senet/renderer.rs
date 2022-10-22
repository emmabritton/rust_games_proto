use crate::boards::idx_coord::BoardCoord;
use crate::boards::{board_cols, board_rows};
use crate::constants::colors::{
    BROWN, CREAM, LIGHT_BLUE, LIGHT_GRAY, PIECE_COMPUTER, PIECE_HUMAN, RED, WHITE,
};
use crate::constants::Direction;
use crate::senet::rules::{HOUSE_BEAUTY, HOUSE_HAPPINESS, HOUSE_REBIRTH, HOUSE_WATER};
use crate::senet::{Move, Square, State};
use crate::system::letter_mesh::make_letter_mesh;
use crate::system::math::{Offset, OffsetTuple, Point, pt};
use crate::system::mesh_helper::MeshHelper;
use crate::system::TurnState::{SelectingMove, SelectingPiece};
use ggez::graphics::DrawMode;
use ggez::{Context, GameResult};

pub(super) fn render(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    state: &State,
) -> GameResult<()> {
    let cell_size = mesh_helper.calc_width(0.09);
    let stick_width = mesh_helper.calc_height(0.015);
    let stick_height = mesh_helper.calc_height(0.1);
    let msg_start = pt(mesh_helper.calc_width(0.05), mesh_helper.calc_height(0.45));
    let stick_start = pt(mesh_helper.calc_width(0.05), mesh_helper.calc_height(0.5));
    let board_start = pt(mesh_helper.calc_width(0.05), mesh_helper.calc_width(0.05));

    let human = mesh_helper.make_circle(ctx, cell_size, cell_size * 0.3, DrawMode::fill())?;
    let computer =
        mesh_helper.make_triangle(ctx, cell_size * 0.6, cell_size * 0.6, Direction::Up)?;
    let grid = mesh_helper.make_grid(
        ctx,
        cell_size * board_cols() as f32,
        cell_size * board_rows() as f32,
        board_cols(),
        board_rows(),
        2.,
        LIGHT_GRAY,
        None,
    )?;
    let rect = mesh_helper.make_rect(
        ctx,
        cell_size * board_cols() as f32,
        cell_size * board_rows() as f32,
        DrawMode::stroke(2.),
    )?;

    mesh_helper.draw_mesh(ctx, grid.as_ref(), board_start);
    mesh_helper.draw_mesh(ctx, rect.as_ref(), board_start);

    draw_cell_text(
        ctx,
        mesh_helper,
        cell_size,
        board_start,
        HOUSE_WATER,
        "WATER",
    );
    draw_cell_text(
        ctx,
        mesh_helper,
        cell_size,
        board_start,
        HOUSE_HAPPINESS,
        "HAPPINESS",
    );
    draw_cell_text(
        ctx,
        mesh_helper,
        cell_size,
        board_start,
        HOUSE_BEAUTY,
        "BEAUTY",
    );
    draw_cell_text(
        ctx,
        mesh_helper,
        cell_size,
        board_start,
        HOUSE_REBIRTH,
        "REBIRTH",
    );

    mesh_helper.draw_white_text(
        ctx,
        "WATER",
        Point::from(BoardCoord::from(HOUSE_WATER))
            .multiply(cell_size, cell_size)
            .offset_point(board_start)
            .offset(cell_size * 0.5, 8.),
        12.,
        true,
    );

    state.board.iter().enumerate().for_each(|(idx, square)| {
        let result = match square {
            Square::Empty => None,
            Square::Human => Some((human.as_ref(), PIECE_HUMAN, board_start)),
            Square::Computer => Some((
                computer.as_ref(),
                PIECE_COMPUTER,
                board_start.offset(cell_size * 0.16, cell_size * 0.15),
            )),
        };
        if let Some((mesh, colour, offset)) = result {
            let pos = Point::from(BoardCoord::from(idx))
                .multiply(cell_size, cell_size)
                .offset_point(offset);
            mesh_helper.draw_coloured_mesh(ctx, mesh, pos, colour);
        }
    });

    if state.play_state.is_either(SelectingPiece) && state.roll.is_some() {
        state
            .cursor
            .render(ctx, mesh_helper, board_start, cell_size)?;

        for mov in state.get_moves_for_selected_piece() {
            draw_move(ctx, mesh_helper, cell_size, board_start, &mov, false)?;
        }
    } else if state.play_state.is_either(SelectingMove) {
        state
            .cursor
            .render_dark(ctx, mesh_helper, board_start, cell_size)?;
        draw_move(
            ctx,
            mesh_helper,
            cell_size,
            board_start,
            &state.get_selected_move(),
            true,
        )?;
    }

    if let Some(roll) = state.roll {
        let stick = mesh_helper.make_rect(ctx, stick_width, stick_height, DrawMode::fill())?;
        for i in 0..4 {
            let colour = if i < roll && roll != 5 { CREAM } else { BROWN };
            mesh_helper.draw_coloured_mesh(
                ctx,
                stick.as_ref(),
                stick_start.offset(stick_width * 1.5 * i as f32, 0.),
                colour,
            );
        }
    }

    if let Some(msg) = &state.msg {
        mesh_helper.draw_white_text(ctx, msg, msg_start, 20., false);
    }

    Ok(())
}

fn draw_cell_text(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    cell_size: f32,
    board_start: Point,
    index: usize,
    text: &str,
) {
    mesh_helper.draw_white_text(
        ctx,
        text,
        Point::from(BoardCoord::from(index))
            .multiply(cell_size, cell_size)
            .offset_point(board_start)
            .offset(cell_size * 0.5, 8.),
        12.,
        true,
    );
}

fn draw_move(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    cell_size: f32,
    board_start: Point,
    mov: &Move,
    highlight: bool,
) -> GameResult<()> {
    let move_mesh =
        mesh_helper.make_circle(ctx, cell_size, cell_size * 0.1, DrawMode::stroke(1.))?;
    let capture_mesh = make_letter_mesh(ctx, mesh_helper, cell_size * 0.3, 'x')?;

    let point = Point::from(BoardCoord::from(mov.dest))
        .multiply(cell_size, cell_size)
        .offset_point(board_start);

    let (mesh, colour, pt) = if mov.exchange {
        (
            capture_mesh.as_ref(),
            RED,
            point.offset(cell_size * 0.35, cell_size * 0.35),
        )
    } else {
        (move_mesh.as_ref(), WHITE, point)
    };

    mesh_helper.draw_coloured_mesh(ctx, mesh, pt, if highlight { LIGHT_BLUE } else { colour });
    Ok(())
}
