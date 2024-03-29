use crate::constants::colors::WHITE;
use crate::constants::TOLERANCE;
use crate::system::math::pt;
use crate::system::mesh_helper::MeshHelper;
use ggez::graphics::{DrawMode, Mesh, MeshBuilder};
use ggez::{Context, GameResult};
use std::rc::Rc;

pub const PADDING: f32 = 0.2;
const THICKNESS: f32 = 4.;

pub fn make_letter_mesh(
    ctx: &mut Context,
    mesh_helper: &mut MeshHelper,
    size: f32,
    letter: char,
) -> GameResult<Rc<Mesh>> {
    mesh_helper.make_mesh(
        ctx,
        format!("letter_{}_{}", letter, size),
        &(|builder| match letter {
            'a' => letter_mesh_a(builder, size, size),
            'b' => letter_mesh_b(builder, size, size),
            'c' => letter_mesh_c(builder, size, size),
            'd' => letter_mesh_d(builder, size, size),
            'j' => letter_mesh_j(builder, size, size),
            'k' => letter_mesh_k(builder, size, size),
            'm' => letter_mesh_m(builder, size, size),
            'n' => letter_mesh_n(builder, size, size),
            'o' => letter_mesh_o(builder, size, size),
            'p' => letter_mesh_p(builder, size, size),
            'q' => letter_mesh_q(builder, size, size),
            'r' => letter_mesh_r(builder, size, size),
            't' => letter_mesh_t(builder, size, size),
            'w' => letter_mesh_w(builder, size, size),
            'x' => letter_mesh_x(builder, size, size),
            _ => panic!("Not implemented for {}", letter),
        }),
    )
}

fn letter_mesh_x(builder: &mut MeshBuilder, width: f32, height: f32) -> GameResult<()> {
    let start_x = width * PADDING;
    let start_y = height * PADDING;
    let end_x = width - start_x;
    let end_y = height - start_y;
    builder.line(&[pt(start_x, start_y), pt(end_x, end_y)], THICKNESS, WHITE)?;
    builder.line(&[pt(end_x, start_y), pt(start_x, end_y)], THICKNESS, WHITE)?;

    Ok(())
}

//all other methods are (MeshBuilder, f32, f32) -> GameResult so this should be as well for compatibility
#[allow(clippy::unnecessary_wraps)]
fn letter_mesh_o(builder: &mut MeshBuilder, width: f32, height: f32) -> GameResult<()> {
    let radius = (height * 0.5) - (height * PADDING);

    builder.circle(
        DrawMode::stroke(THICKNESS),
        pt(width * 0.5, height * 0.5),
        radius,
        TOLERANCE,
        WHITE,
    )?;

    Ok(())
}

fn letter_mesh_k(builder: &mut MeshBuilder, width: f32, height: f32) -> GameResult<()> {
    let start_x = width * PADDING;
    let start_y = height * PADDING;
    let end_x = width - start_x;
    let end_y = height - start_y;
    let upper_leg_y = (end_y - start_y) * 0.6 + start_y;
    let lower_leg_y = (end_y - start_y) * 0.5 + start_y;
    builder.line(
        &[pt(start_x, start_y), pt(start_x, end_y)],
        THICKNESS,
        WHITE,
    )?;
    builder.line(
        &[pt(start_x, upper_leg_y), pt(end_x, start_y)],
        THICKNESS,
        WHITE,
    )?;
    builder.line(
        &[pt(start_x + (width * 0.1), lower_leg_y), pt(end_x, end_y)],
        THICKNESS,
        WHITE,
    )?;

    Ok(())
}

fn letter_mesh_d(builder: &mut MeshBuilder, width: f32, height: f32) -> GameResult<()> {
    let start_x = width * PADDING;
    let start_y = height * PADDING;
    let bottom_x = width * PADDING;
    let bottom_y = height - height * PADDING;
    builder.line(
        &[pt(start_x, start_y), pt(bottom_x, bottom_y)],
        THICKNESS,
        WHITE,
    )?;
    builder.polyline(
        DrawMode::stroke(THICKNESS),
        &[
            pt(start_x, start_y),
            pt(start_x + width * 0.3, start_y),
            pt(start_x + width * 0.5, start_y + height * 0.1),
            pt(start_x + width * 0.55, start_y + height * 0.2),
            pt(bottom_x + width * 0.55, bottom_y - height * 0.2),
            pt(bottom_x + width * 0.5, bottom_y - height * 0.1),
            pt(bottom_x + width * 0.3, bottom_y),
            pt(bottom_x, bottom_y),
        ],
        WHITE,
    )?;

    Ok(())
}

fn letter_mesh_a(builder: &mut MeshBuilder, width: f32, height: f32) -> GameResult<()> {
    let start_x = width * PADDING;
    let start_y = height - height * PADDING;
    let top_x = width * 0.5;
    let top_y = height * PADDING;
    let end_x = width - width * PADDING;
    let end_y = height - height * PADDING;
    let bar_start_x = width * 0.3;
    let bar_start_y = height * 0.6;
    let bar_end_x = width * 0.7;
    let bar_end_y = height * 0.6;
    builder.line(&[pt(start_x, start_y), pt(top_x, top_y)], THICKNESS, WHITE)?;
    builder.line(&[pt(end_x, end_y), pt(top_x, top_y)], THICKNESS, WHITE)?;
    builder.line(
        &[pt(bar_start_x, bar_start_y), pt(bar_end_x, bar_end_y)],
        THICKNESS,
        WHITE,
    )?;

    Ok(())
}

fn letter_mesh_q(builder: &mut MeshBuilder, width: f32, height: f32) -> GameResult<()> {
    let radius = (height * 0.5) - (height * PADDING);

    builder.circle(
        DrawMode::stroke(THICKNESS),
        pt(width * 0.5, height * 0.5),
        radius,
        TOLERANCE,
        WHITE,
    )?;

    builder.line(
        &[
            pt(width * 0.5, height * 0.6),
            pt(width - width * PADDING, height - height * PADDING),
        ],
        THICKNESS,
        WHITE,
    )?;

    Ok(())
}

fn letter_mesh_r(builder: &mut MeshBuilder, width: f32, height: f32) -> GameResult<()> {
    let start_x = width * PADDING;
    let start_y = height * PADDING;
    let bottom_x = width * PADDING;
    let bottom_y = height * 0.5;
    builder.line(
        &[
            pt(start_x, start_y),
            pt(bottom_x, height - height * PADDING),
        ],
        THICKNESS,
        WHITE,
    )?;
    builder.polyline(
        DrawMode::stroke(THICKNESS),
        &[
            pt(start_x, start_y),
            pt(start_x + width * 0.3, start_y),
            pt(start_x + width * 0.45, start_y + (bottom_y - start_y) * 0.2),
            pt(start_x + width * 0.5, start_y + (bottom_y - start_y) * 0.4),
            pt(bottom_x + width * 0.5, start_y + (bottom_y - start_y) * 0.6),
            pt(
                bottom_x + width * 0.45,
                start_y + (bottom_y - start_y) * 0.8,
            ),
            pt(bottom_x + width * 0.3, bottom_y),
            pt(bottom_x, bottom_y),
        ],
        WHITE,
    )?;
    builder.line(
        &[
            pt(width * 0.4, height * 0.5),
            pt(width - width * PADDING, height - height * PADDING),
        ],
        THICKNESS,
        WHITE,
    )?;

    Ok(())
}

fn letter_mesh_p(builder: &mut MeshBuilder, width: f32, height: f32) -> GameResult<()> {
    let start_x = width * PADDING;
    let start_y = height * PADDING;
    let bottom_x = width * PADDING;
    let bottom_y = height * 0.5;
    builder.line(
        &[
            pt(start_x, start_y),
            pt(bottom_x, height - height * PADDING),
        ],
        THICKNESS,
        WHITE,
    )?;
    builder.polyline(
        DrawMode::stroke(THICKNESS),
        &[
            pt(start_x, start_y),
            pt(start_x + width * 0.3, start_y),
            pt(start_x + width * 0.45, start_y + (bottom_y - start_y) * 0.2),
            pt(start_x + width * 0.5, start_y + (bottom_y - start_y) * 0.4),
            pt(bottom_x + width * 0.5, start_y + (bottom_y - start_y) * 0.6),
            pt(
                bottom_x + width * 0.45,
                start_y + (bottom_y - start_y) * 0.8,
            ),
            pt(bottom_x + width * 0.3, bottom_y),
            pt(bottom_x, bottom_y),
        ],
        WHITE,
    )?;

    Ok(())
}

fn letter_mesh_b(builder: &mut MeshBuilder, width: f32, height: f32) -> GameResult<()> {
    let start_x = width * PADDING;
    let start_y = height * PADDING;
    let bottom_x = width * PADDING;
    let mid_y = height * 0.5;
    let bottom_y = height - height * PADDING;
    builder.line(
        &[pt(start_x, start_y), pt(bottom_x, bottom_y)],
        THICKNESS,
        WHITE,
    )?;
    builder.polyline(
        DrawMode::stroke(THICKNESS),
        &[
            pt(start_x, start_y),
            pt(start_x + width * 0.3, start_y),
            pt(start_x + width * 0.45, start_y + (mid_y - start_y) * 0.2),
            pt(start_x + width * 0.5, start_y + (mid_y - start_y) * 0.4),
            pt(bottom_x + width * 0.5, start_y + (mid_y - start_y) * 0.6),
            pt(bottom_x + width * 0.45, start_y + (mid_y - start_y) * 0.8),
            pt(bottom_x + width * 0.3, mid_y),
            pt(bottom_x, mid_y),
        ],
        WHITE,
    )?;
    builder.polyline(
        DrawMode::stroke(THICKNESS),
        &[
            pt(start_x, mid_y),
            pt(start_x + width * 0.3, mid_y),
            pt(start_x + width * 0.45, mid_y + (bottom_y - mid_y) * 0.2),
            pt(start_x + width * 0.5, mid_y + (bottom_y - mid_y) * 0.4),
            pt(bottom_x + width * 0.5, mid_y + (bottom_y - mid_y) * 0.6),
            pt(bottom_x + width * 0.45, mid_y + (bottom_y - mid_y) * 0.8),
            pt(bottom_x + width * 0.3, bottom_y),
            pt(bottom_x, bottom_y),
        ],
        WHITE,
    )?;
    Ok(())
}

fn letter_mesh_m(builder: &mut MeshBuilder, width: f32, height: f32) -> GameResult<()> {
    let left = width * PADDING;
    let right = width - width * PADDING;
    let top = height * PADDING;
    let bottom = height - height * PADDING;
    let mid_x = width * 0.5;
    let mid_y = height * 0.5;

    builder.polyline(
        DrawMode::stroke(THICKNESS),
        &[
            pt(left, bottom),
            pt(left, top),
            pt(mid_x, mid_y),
            pt(right, top),
            pt(right, bottom),
        ],
        WHITE,
    )?;

    Ok(())
}

fn letter_mesh_w(builder: &mut MeshBuilder, width: f32, height: f32) -> GameResult<()> {
    let left = width * PADDING;
    let left_inner = width * PADDING * 1.5;
    let right = width - width * PADDING;
    let right_inner = width - (width * PADDING * 1.5);
    let top = height * PADDING;
    let bottom = height - (height * PADDING * 1.5);
    let mid_x = width * 0.5;
    let mid_y = height * 0.4;

    builder.polyline(
        DrawMode::stroke(THICKNESS),
        &[
            pt(left, top),
            pt(left_inner, bottom),
            pt(mid_x, mid_y),
            pt(right_inner, bottom),
            pt(right, top),
        ],
        WHITE,
    )?;

    Ok(())
}

fn letter_mesh_t(builder: &mut MeshBuilder, width: f32, height: f32) -> GameResult<()> {
    let top = height * PADDING;
    let left = width * PADDING;
    let right = width - width * PADDING;
    let mid_x = width * 0.5;
    let bottom = height - height * PADDING;

    builder.line(&[pt(left, top), pt(right, top)], THICKNESS, WHITE)?;
    builder.line(&[pt(mid_x, top), pt(mid_x, bottom)], THICKNESS, WHITE)?;

    Ok(())
}

fn letter_mesh_j(builder: &mut MeshBuilder, width: f32, height: f32) -> GameResult<()> {
    let top = height * PADDING;
    let left = width * PADDING;
    let right = width - width * PADDING;
    let mid_x = width * 0.5;
    let bar_bottom = height * 0.6;
    let step_w = mid_x - left;
    let step_h = height * 0.08;

    builder.line(&[pt(left, top), pt(right, top)], THICKNESS, WHITE)?;
    builder.line(&[pt(mid_x, top), pt(mid_x, bar_bottom)], THICKNESS, WHITE)?;
    builder.polyline(
        DrawMode::stroke(THICKNESS),
        &[
            pt(left, bar_bottom),
            pt(left + step_w * 0.1, bar_bottom + step_h),
            pt(left + step_w * 0.32, bar_bottom + step_h * 2.0),
            pt(left + step_w * 0.68, bar_bottom + step_h * 2.0),
            pt(left + step_w * 0.9, bar_bottom + step_h),
            pt(left + step_w * 1.0, bar_bottom),
        ],
        WHITE,
    )?;

    Ok(())
}

fn letter_mesh_c(builder: &mut MeshBuilder, width: f32, height: f32) -> GameResult<()> {
    let left = width * PADDING;
    let bar_top = height * 0.35;
    let bar_bottom = height * 0.65;
    let step_w = width - left * 2.0;
    let step_h = height - (height * PADDING) - bar_bottom;

    builder.line(&[pt(left, bar_top), pt(left, bar_bottom)], THICKNESS, WHITE)?;
    builder.polyline(
        DrawMode::stroke(THICKNESS),
        &[
            pt(left + step_w * 0.0, bar_bottom + step_h * 0.0),
            pt(left + step_w * 0.2, bar_bottom + step_h * 0.6),
            pt(left + step_w * 0.5, bar_bottom + step_h * 1.0),
            pt(left + step_w * 0.8, bar_bottom + step_h * 0.6),
            pt(left + step_w * 1.0, bar_bottom + step_h * 0.0),
        ],
        WHITE,
    )?;
    builder.polyline(
        DrawMode::stroke(THICKNESS),
        &[
            pt(left + step_w * 0.0, bar_top - step_h * 0.0),
            pt(left + step_w * 0.2, bar_top - step_h * 0.6),
            pt(left + step_w * 0.5, bar_top - step_h * 1.0),
            pt(left + step_w * 0.8, bar_top - step_h * 0.6),
            pt(left + step_w * 1.0, bar_top - step_h * 0.0),
        ],
        WHITE,
    )?;

    Ok(())
}

fn letter_mesh_n(builder: &mut MeshBuilder, width: f32, height: f32) -> GameResult<()> {
    let left = width * PADDING;
    let right = width - width * PADDING;
    let top = height * PADDING;
    let bottom = height * PADDING;

    builder.polyline(
        DrawMode::stroke(THICKNESS),
        &[
            pt(left, height - bottom),
            pt(left, top * 1.3),
            pt(right, height - bottom * 1.3),
            pt(right, top),
        ],
        WHITE,
    )?;

    Ok(())
}
