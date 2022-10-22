use crate::constants::colors::{BLUE, LIGHT_BLUE};
use crate::mancala::Hole::End;
use crate::mancala::{Hole, Square, State};
use crate::system::math::{pt, Point, Offset};
use crate::system::mesh_helper::MeshHelper;
use crate::system::Player;
use crate::system::TurnState::{Animating, SelectingPiece};
use ggez::graphics::DrawMode;
use ggez::{Context, GameResult};

fn square_pos(start: Point, width: f32, height: f32, spacing: f32, square: &Square) -> Point {
    match square.player {
        Player::Human => match square.hole {
            Hole::Home(idx) => start.offset(
                (width + spacing) * (idx + 1) as f32,
                height + spacing + spacing,
            ),
            Hole::End => start.offset((width + spacing) * 7., 0.),
        },
        Player::Computer => match square.hole {
            Hole::Home(idx) => start.offset((width + spacing) * (5 - idx + 1) as f32, 0.),
            Hole::End => start,
        },
    }
}

pub(super) fn render(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    state: &State,
) -> GameResult<()> {
    let padding = mesh_helper.calc_width(0.03);
    let board_start = pt(padding, padding * 3.);
    let hole_width = mesh_helper.calc_width(0.1);
    let hole_height = mesh_helper.calc_height(0.2);
    let spacing = mesh_helper.calc_height(0.02);
    let note_pos = pt(mesh_helper.calc_width(0.03), mesh_helper.calc_height(0.6));
    let event_pos = pt(mesh_helper.calc_width(0.5), mesh_helper.calc_height(0.05));

    let score = mesh_helper.make_rect(
        ctx,
        hole_width,
        hole_height + hole_height + spacing + spacing,
        DrawMode::stroke(2.),
    )?;
    let home = mesh_helper.make_rect(ctx, hole_width, hole_height, DrawMode::stroke(2.))?;

    let mut draw_hole = |square: &Square, count: usize| {
        let pos = square_pos(board_start, hole_width, hole_height, spacing, square);
        let (mesh, max) = if square.is_home() {
            (home.as_ref(), 24)
        } else {
            (score.as_ref(), 56)
        };
        mesh_helper.draw_mesh(ctx, mesh, pos);
        draw_stones(ctx, mesh_helper, pos, count, max).unwrap();
    };

    for i in 0..6 {
        draw_hole(
            &state.board.home_idx_to_square(Player::Computer, i),
            state.board.computer.homes[i],
        );
        draw_hole(
            &state.board.home_idx_to_square(Player::Human, i),
            state.board.human.homes[i],
        );
    }
    draw_hole(
        &Square::new(Player::Computer, End),
        state.board.computer.end,
    );
    draw_hole(&Square::new(Player::Human, End), state.board.human.end);

    let dropping = if state.play_state.is_human(SelectingPiece) {
        Some((
            state.board.idx_to_square(state.cursor.value),
            LIGHT_BLUE,
            &None,
        ))
    } else if state.play_state.is_human(Animating) {
        Some((
            state.board.idx_to_square(state.cursor.value),
            BLUE,
            &state.drop_move,
        ))
    } else if state.play_state.is_computer(Animating) {
        Some((
            state.board.idx_to_square(state.computer_cursor),
            BLUE,
            &state.drop_move,
        ))
    } else {
        None
    };

    if let Some((square, color, drop_move)) = dropping {
        let cursor_pos = square_pos(board_start, hole_width, hole_height, spacing, &square);
        mesh_helper.draw_coloured_mesh(ctx, home.as_ref(), cursor_pos, color);
        if let Some(drop_move) = drop_move {
            if drop_move.remaining > 0 {
                draw_drop_moves(
                    ctx,
                    mesh_helper,
                    square_pos(
                        board_start,
                        hole_width,
                        hole_height,
                        spacing,
                        &drop_move.current_square,
                    )
                    .offset(hole_width * 0.5, -20.),
                    drop_move.remaining,
                );
            }
        }
    }

    if let Some((message, event)) = &state.message {
        mesh_helper.draw_white_text(
            ctx,
            message,
            if *event { event_pos } else { note_pos },
            if *event { 30. } else { 20. },
            *event,
        );
    }

    Ok(())
}

fn draw_drop_moves(ctx: &mut Context, mesh_helper: &mut MeshHelper, pos: Point, count: usize) {
    mesh_helper.draw_white_text(ctx, &format!("▼ {} ▼", count), pos, 18., true);
}

fn draw_stones(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    start: Point,
    count: usize,
    max: usize,
) -> GameResult<()> {
    let stone_size = mesh_helper.calc_width(0.02);
    let padding = mesh_helper.calc_width(0.004);
    let stone = mesh_helper.make_circle(ctx, stone_size, stone_size * 0.5, DrawMode::fill())?;
    let start = start.offset(padding, padding);

    if count > max {
        let y = (max as f32 / 7.) * stone_size;
        let stone_x = 1. * stone_size;
        let text_x = 2.2 * stone_size;
        mesh_helper.draw_mesh(ctx, stone.as_ref(), start.offset(stone_x, y));
        mesh_helper.draw_white_text(
            ctx,
            &format!("x {}", count),
            start.offset(text_x, y + 8.),
            20.,
            false,
        );
    } else {
        let mut x = 0;
        let mut y = 0;
        for _ in 0..count {
            mesh_helper.draw_mesh(
                ctx,
                stone.as_ref(),
                start.offset(
                    x as f32 * (stone_size + padding),
                    y as f32 * (stone_size + padding),
                ),
            );
            x += 1;
            if x > 3 {
                y += 1;
                x = 0;
            }
        }
    }

    Ok(())
}
