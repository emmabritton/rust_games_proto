use crate::constants::colors::{LIGHT_GRAY, WHITE};
use crate::constants::Direction;
use crate::ext::NewLines;
use crate::menu::menu_items::ITEMS;
use crate::menu::State;
use crate::system::math::{pt, Offset, Point};
use crate::system::mesh_helper::MeshHelper;
use ggez::graphics::Color;
use ggez::{Context, GameResult};

pub(super) fn render(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    state: &State,
) -> GameResult<()> {
    let title_start = pt(32., 32.);
    let menu_start = pt(34., 100.);
    let submenu_start = pt(240., 100.);
    let cursor_start = pt(16., 105.);
    let subcursor_start = pt(225., 105.);
    let desc_start = pt(34., 164. + (ITEMS.len() * 24) as f32);
    let subdesc_start = pt(34., 164. + (ITEMS.len() * 24) as f32);
    let cursor = mesh_helper.make_triangle(ctx, 12., 12., Direction::Right)?;

    mesh_helper.draw_white_text(ctx, "Games", title_start, 48., false);

    match &state.subcursor {
        None => {
            mesh_helper.draw_mesh(
                ctx,
                cursor.as_ref(),
                cursor_start.offsety(state.cursor.value * 24),
            );

            mesh_helper.draw_white_text(
                ctx,
                &ITEMS[state.cursor.value].0.desc,
                desc_start,
                20.,
                false,
            );

            draw_menu_text(ctx, mesh_helper, menu_start, WHITE);

            if ITEMS[state.cursor.value].1.is_some() {
                draw_submenu_text(ctx, mesh_helper, state, submenu_start, LIGHT_GRAY);
            }
        }
        Some(subcursor) => {
            draw_menu_text(ctx, mesh_helper, menu_start, LIGHT_GRAY);
            draw_submenu_text(ctx, mesh_helper, state, submenu_start, WHITE);

            mesh_helper.draw_mesh(
                ctx,
                cursor.as_ref(),
                subcursor_start.offsety(subcursor.value * 22),
            );
            mesh_helper.draw_coloured_mesh(
                ctx,
                cursor.as_ref(),
                cursor_start.offsety(state.cursor.value * 24),
                LIGHT_GRAY,
            );

            mesh_helper.draw_white_text(
                ctx,
                ITEMS[state.cursor.value].0.desc,
                desc_start,
                20.,
                false,
            );
            mesh_helper.draw_white_text(
                ctx,
                ITEMS[state.cursor.value].1.as_ref().unwrap()[subcursor.value].desc,
                subdesc_start.offsety((&ITEMS[state.cursor.value].0.desc.new_lines() + 2) * 20),
                20.,
                false,
            );
        }
    }

    Ok(())
}

fn draw_menu_text(ctx: &mut Context, mesh_helper: &mut MeshHelper, start: Point, color: Color) {
    ITEMS.iter().enumerate().for_each(|(idx, item)| {
        mesh_helper.draw_text(
            ctx,
            &item.0.name,
            start.offsety(idx * 24),
            color,
            24.,
            false,
        );
    });
}

fn draw_submenu_text(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    state: &State,
    start: Point,
    color: Color,
) {
    ITEMS[state.cursor.value]
        .1
        .as_ref()
        .unwrap()
        .iter()
        .enumerate()
        .for_each(|(idx, item)| {
            mesh_helper.draw_text(ctx, &item.name, start.offsety(idx * 22), color, 22., false);
        });
}
