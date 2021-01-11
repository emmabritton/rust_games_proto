use crate::boards::idx_coord::BoardCoord;
use crate::boards::{board_cols, board_rows};
use crate::constants::colors::{
    DARK_GRAY, DARK_GREEN, FAINT_BLUE, FAINT_RED, LIGHT_BLUE, LIGHT_GRAY, RED, WHITE,
};
use crate::system::letter_mesh::make_letter_mesh;
use crate::system::math::{pt, Offset, OffsetTuple, Point};
use crate::system::mesh_helper::MeshHelper;
use crate::system::PlayState::ModeSelection;
use crate::system::TurnState::{SelectingMove, SelectingPiece};
use crate::tablut::render_mode_selection::render_mode_selection;
use crate::tablut::{Move, Square, State};
use ggez::graphics::DrawMode;
use ggez::{Context, GameResult};

pub(super) fn render(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    state: &mut State,
) -> GameResult<()> {
    if state.play_state == ModeSelection {
        render_mode_selection(ctx, mesh_helper, state)
    } else {
        render_game(ctx, mesh_helper, state)
    }
}

fn render_game(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    state: &mut State,
) -> GameResult<()> {
    let cell_size = mesh_helper.calc_height(0.09);
    let board_start = pt(cell_size * board_cols() as f32 * 0.5, cell_size);

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
    let cell = mesh_helper.make_rect(ctx, cell_size, cell_size, DrawMode::fill())?;

    mesh_helper.draw_mesh(ctx, grid.as_ref(), board_start);
    mesh_helper.draw_mesh(ctx, rect.as_ref(), board_start);
    mesh_helper.draw_coloured_mesh(ctx, cell.as_ref(), board_start, DARK_GREEN);
    mesh_helper.draw_coloured_mesh(
        ctx,
        cell.as_ref(),
        board_start.offset(cell_size * (board_cols() as f32 - 1.), 0.),
        DARK_GREEN,
    );
    mesh_helper.draw_coloured_mesh(
        ctx,
        cell.as_ref(),
        board_start.offset(
            cell_size * (board_cols() as f32 - 1.),
            cell_size * (board_cols() as f32 - 1.),
        ),
        DARK_GREEN,
    );
    mesh_helper.draw_coloured_mesh(
        ctx,
        cell.as_ref(),
        board_start.offset(0., cell_size * (board_cols() as f32 - 1.)),
        DARK_GREEN,
    );
    mesh_helper.draw_coloured_mesh(
        ctx,
        cell.as_ref(),
        board_start.offset(cell_size * 4., cell_size * 4.),
        DARK_GRAY,
    );

    let a = make_letter_mesh(ctx, mesh_helper, cell_size, 'a')?;
    let d = make_letter_mesh(ctx, mesh_helper, cell_size, 'd')?;
    let k = make_letter_mesh(ctx, mesh_helper, cell_size, 'k')?;

    state.board.iter().enumerate().for_each(|(idx, square)| {
        let mut pt: Point = BoardCoord::from(idx).into();
        pt = pt.multiply(cell_size, cell_size).offset_point(board_start);
        let mesh = match square {
            Square::Empty => None,
            Square::King => Some((k.as_ref(), FAINT_BLUE)),
            Square::Defender => Some((d.as_ref(), FAINT_BLUE)),
            Square::Attacker => Some((a.as_ref(), FAINT_RED)),
        };
        if let Some((mesh, color)) = mesh {
            mesh_helper.draw_coloured_mesh(ctx, mesh, pt, color);
        }
    });

    if state.play_state.is_human(SelectingPiece) {
        state
            .cursor
            .render(ctx, mesh_helper, board_start, cell_size)?;

        for mov in state.get_moves_for_selected_piece() {
            draw_move(ctx, mesh_helper, cell_size, board_start, &mov, false)?;
        }
    } else if state.play_state.is_either(SelectingMove) {
        draw_move(
            ctx,
            mesh_helper,
            cell_size,
            board_start,
            &state.get_selected_move(),
            true,
        )?;
    }

    Ok(())
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

    mesh_helper.draw_coloured_mesh(
        ctx,
        move_mesh.as_ref(),
        Point::from(BoardCoord::from(mov.dest))
            .multiply(cell_size, cell_size)
            .offset_point(board_start),
        if highlight { LIGHT_BLUE } else { WHITE },
    );
    for capture in &mov.capturing {
        mesh_helper.draw_coloured_mesh(
            ctx,
            capture_mesh.as_ref(),
            Point::from(BoardCoord::from(*capture))
                .multiply(cell_size, cell_size)
                .offset_point(board_start)
                .offset(cell_size * 0.35, cell_size * 0.35),
            RED,
        );
    }
    Ok(())
}
