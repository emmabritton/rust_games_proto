use crate::boards::idx_coord::BoardCoord;
use crate::boards::{board_cols, board_rows};
use crate::constants::colors::{FILTER_BLACK, LIGHT_BLUE, LIGHT_GRAY, RED, TRANSPARENT, WHITE};
use crate::orderchaos::render_mode_selection::render_mode_selection;
use crate::orderchaos::{Mode, Square, State};
use crate::system::math::{pt, Offset};
use crate::system::mesh_helper::MeshHelper;
use crate::system::PlayState;
use crate::system::TurnState::SelectingMove;
use ggez::graphics::{Color, DrawMode};
use ggez::{Context, GameResult};

pub(super) fn render(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    state: &mut State,
) -> GameResult<()> {
    if state.play_state == PlayState::ModeSelection {
        render_mode_selection(ctx, mesh_helper, state)
    } else {
        render_game(ctx, mesh_helper, state)
    }
}

pub(super) fn square_to_color(square: &Square) -> Color {
    match square {
        Square::Red => RED,
        Square::White => WHITE,
        Square::Empty => TRANSPARENT,
    }
}

pub(super) fn render_game(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    state: &mut State,
) -> GameResult<()> {
    let cell_size = mesh_helper.calc_height(0.13);
    let board_start = pt(
        (mesh_helper.width * 0.5) - (cell_size * (board_cols() as f32 / 2.)),
        cell_size,
    );

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

    let disc = mesh_helper.make_circle(ctx, cell_size, cell_size * 0.4, DrawMode::fill())?;
    let highlight =
        mesh_helper.make_circle(ctx, cell_size, cell_size * 0.4, DrawMode::stroke(4.))?;

    mesh_helper.draw_mesh(ctx, grid.as_ref(), board_start);
    mesh_helper.draw_mesh(ctx, rect.as_ref(), board_start);

    state.board.iter().enumerate().for_each(|(idx, square)| {
        let coord = BoardCoord::from(idx);
        mesh_helper.draw_coloured_mesh(
            ctx,
            disc.as_ref(),
            board_start.offset(coord.0 as f32 * cell_size, coord.1 as f32 * cell_size),
            square_to_color(square),
        );
    });

    if state.play_state.is_playing() {
        state
            .cursor
            .render(ctx, mesh_helper, board_start, cell_size)?;
        if state.play_state.is_human(SelectingMove) {
            let fullscreen = mesh_helper.make_rect(
                ctx,
                mesh_helper.width,
                mesh_helper.height,
                DrawMode::fill(),
            )?;
            mesh_helper.draw_coloured_mesh(ctx, fullscreen.as_ref(), pt(0., 0.), FILTER_BLACK);
            let xy = state.cursor.point(board_start, cell_size);
            xy.offset(cell_size * 0.5, -cell_size);
            let order_pos = xy.offset(-(cell_size * 0.5), 0.);
            let chaos_pos = xy.offset(cell_size * 0.5, 0.);
            mesh_helper.draw_coloured_mesh(
                ctx,
                disc.as_ref(),
                order_pos,
                square_to_color(&Mode::Order.into()),
            );
            mesh_helper.draw_coloured_mesh(
                ctx,
                disc.as_ref(),
                chaos_pos,
                square_to_color(&Mode::Chaos.into()),
            );
            let highlight_pos = match state.move_cursor {
                Mode::Order => order_pos,
                Mode::Chaos => chaos_pos,
            };
            mesh_helper.draw_coloured_mesh(ctx, highlight.as_ref(), highlight_pos, LIGHT_BLUE);
        }
    }

    Ok(())
}
