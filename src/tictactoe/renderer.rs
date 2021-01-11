use crate::boards::idx_coord::BoardCoord;
use crate::constants::colors::{DARK_GRAY, GRAY, WHITE};
use crate::system::letter_mesh::make_letter_mesh;
use crate::system::math::{pt, Offset, OffsetTuple, Point};
use crate::system::mesh_helper::MeshHelper;
use crate::system::PlayState::*;
use crate::system::Turn::{Computer, Human};
use crate::system::TurnState::SelectingPiece;
use crate::tictactoe::{Square, State};
use ggez::{Context, GameResult};

pub(super) fn render(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    state: &mut State,
) -> GameResult<()> {
    let board_size = mesh_helper.calc_height(0.8);
    let cell_size = board_size / 3.;
    let grid = mesh_helper.make_grid(ctx, board_size, board_size, 3, 3, 2., GRAY, None)?;

    let board_start = pt(
        mesh_helper.width * 0.5 - board_size * 0.5,
        mesh_helper.calc_height(0.1),
    );

    let x_mesh = make_letter_mesh(ctx, mesh_helper, cell_size, 'x')?;
    let o_mesh = make_letter_mesh(ctx, mesh_helper, cell_size, 'o')?;

    mesh_helper.draw_mesh(ctx, grid.as_ref(), board_start);

    let (p_color, c_color) = match state.play_state {
        Playing(Human(_)) | HumanWin => (WHITE, DARK_GRAY),
        Playing(Computer(_)) | ComputerWin => (DARK_GRAY, WHITE),
        ModeSelection | Init | Draw => (DARK_GRAY, DARK_GRAY),
    };

    mesh_helper.draw_text(
        ctx,
        "X - Player",
        pt(14., mesh_helper.height - 36.),
        p_color,
        20.,
        false,
    );
    mesh_helper.draw_text(
        ctx,
        "O - Computer",
        pt(14., mesh_helper.height - 58.),
        c_color,
        20.,
        false,
    );

    state.board.iter().enumerate().for_each(|(i, square)| {
        let xy = Point::from(BoardCoord::from(i))
            .multiply(cell_size, cell_size)
            .offset_point(board_start);
        match square {
            Square::X => mesh_helper.draw_mesh(ctx, x_mesh.as_ref(), xy),
            Square::O => mesh_helper.draw_mesh(ctx, o_mesh.as_ref(), xy),
            Square::E => {}
        }
    });

    if state.play_state.is_human(SelectingPiece) {
        state
            .cursor
            .render(ctx, mesh_helper, board_start, cell_size)?;
    }

    Ok(())
}
