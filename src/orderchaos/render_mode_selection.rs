use crate::constants::colors::{LIGHT_BLUE, LIGHT_GRAY, WHITE};
use crate::orderchaos::renderer::square_to_color;
use crate::orderchaos::{Mode, State};
use crate::system::math::{Offset, pt};
use crate::system::mesh_helper::MeshHelper;
use ggez::graphics::DrawMode;
use ggez::{Context, GameResult};

pub(super) fn render_mode_selection(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    state: &State,
) -> GameResult<()> {
    let cell_size = mesh_helper.calc_width(0.05);
    let order_pos = pt(mesh_helper.calc_width(0.42), mesh_helper.calc_height(0.5));
    let chaos_pos = pt(mesh_helper.calc_width(0.58), mesh_helper.calc_height(0.5));
    let msg_pos = pt(mesh_helper.width * 0.5, mesh_helper.height * 0.7);

    let disc = mesh_helper.make_circle(ctx, 0., cell_size, DrawMode::fill())?;
    let highlight = mesh_helper.make_circle(ctx, 0., cell_size, DrawMode::stroke(4.))?;

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

    let (order, chaos, msg, highlight_pos) = match state.player_mode {
        Mode::Order => (
            WHITE,
            LIGHT_GRAY,
            "Create a line of 5 of either colour",
            order_pos,
        ),
        Mode::Chaos => (
            LIGHT_GRAY,
            WHITE,
            "Block the line from being created",
            chaos_pos,
        ),
    };

    mesh_helper.draw_coloured_mesh(ctx, highlight.as_ref(), highlight_pos, LIGHT_BLUE);
    mesh_helper.draw_text(
        ctx,
        "Order",
        order_pos.offset(0., cell_size * 1.2),
        order,
        20.,
        true,
    );
    mesh_helper.draw_text(
        ctx,
        "Chaos",
        chaos_pos.offset(0., cell_size * 1.2),
        chaos,
        20.,
        true,
    );
    mesh_helper.draw_white_text(ctx, msg, msg_pos, 18., true);

    Ok(())
}
