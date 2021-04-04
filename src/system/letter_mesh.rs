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
            'd' => letter_mesh_d(builder, size, size),
            'k' => letter_mesh_k(builder, size, size),
            'o' => letter_mesh_o(builder, size, size),
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
    );

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
            pt(start_x + width * 0.6, height * 0.5),
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
