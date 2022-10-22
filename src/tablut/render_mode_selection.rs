use crate::constants::colors::{LIGHT_BLUE, LIGHT_GRAY};
use crate::constants::Direction;
use crate::system::math::{Offset, pt};
use crate::system::mesh_helper::MeshHelper;
use crate::tablut::{Mode, State};
use ggez::graphics::DrawMode;
use ggez::{Context, GameResult};

pub(super) fn render_mode_selection(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    state: &State,
) -> GameResult<()> {
    let symbol_size = mesh_helper.calc_height(0.2);
    let attack = mesh_helper.make_rect(ctx, symbol_size, symbol_size, DrawMode::fill())?;
    let defend = mesh_helper.make_rect(ctx, symbol_size, symbol_size, DrawMode::fill())?;
    let cursor = mesh_helper.make_triangle(ctx, 12., 12., Direction::Right)?;

    let attack_pos = pt(
        mesh_helper.width * 0.5 - symbol_size * 1.2,
        mesh_helper.calc_height(0.3),
    );
    let defend_pos = pt(
        mesh_helper.width * 0.5 + symbol_size * 0.2,
        mesh_helper.calc_height(0.3),
    );

    let (attack_color, defend_color, cursor_pos, msg) = match state.player_mode {
        Mode::Attacker => (
            LIGHT_BLUE,
            LIGHT_GRAY,
            attack_pos.offset(-18., symbol_size * 1.1 + 4.),
            "Capture the king!",
        ),
        Mode::Defender => (
            LIGHT_GRAY,
            LIGHT_BLUE,
            defend_pos.offset(-18., symbol_size * 1.1 + 4.),
            "Get the king to a corner!",
        ),
    };

    mesh_helper.draw_coloured_mesh(ctx, attack.as_ref(), attack_pos, attack_color);
    mesh_helper.draw_coloured_mesh(ctx, defend.as_ref(), defend_pos, defend_color);
    mesh_helper.draw_white_text(
        ctx,
        "Attacker",
        attack_pos.offset(0., symbol_size * 1.1),
        20.,
        false,
    );
    mesh_helper.draw_white_text(
        ctx,
        "Defender",
        defend_pos.offset(0., symbol_size * 1.1),
        20.,
        false,
    );
    mesh_helper.draw_white_text(
        ctx,
        msg,
        mesh_helper
            .center()
            .offset(0., mesh_helper.calc_height(0.12)),
        16.,
        true,
    );
    mesh_helper.draw_mesh(ctx, cursor.as_ref(), cursor_pos);

    Ok(())
}
