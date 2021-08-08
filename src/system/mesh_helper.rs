use crate::constants::colors::WHITE;
use crate::constants::{Direction, TOLERANCE};
use crate::system::math::{pt, Point};
use ggez::graphics::{
    Color, DrawMode, Drawable, Mesh, MeshBuilder, Rect, Scale, Text, TextFragment,
};
use ggez::{graphics, Context, GameResult};
use std::collections::HashMap;
use std::rc::Rc;

pub struct MeshHelper {
    pub width: f32,
    pub height: f32,
    mesh_cache: HashMap<String, Rc<Mesh>>,
}

impl MeshHelper {
    pub fn new(ctx: &mut Context) -> MeshHelper {
        let (width, height) = MeshHelper::get_screen_size(ctx);
        MeshHelper {
            width,
            height,
            mesh_cache: HashMap::new(),
        }
    }
}

impl MeshHelper {
    pub fn get_screen_size(ctx: &mut Context) -> (f32, f32) {
        return graphics::window(ctx)
            .get_inner_size()
            .map(|physical| (physical.width as f32, physical.height as f32))
            .expect("Failed to get/convert window size");
    }
}

impl MeshHelper {
    pub fn center(&self) -> Point {
        pt(self.width * 0.5, self.height * 0.5)
    }

    pub fn calc_width(&self, percent: f32) -> f32 {
        self.width * percent
    }

    pub fn calc_height(&self, percent: f32) -> f32 {
        self.height * percent
    }

    #[allow(clippy::too_many_arguments)]
    pub fn make_grid(
        &mut self,
        ctx: &mut Context,
        width: f32,
        height: f32,
        cols: usize,
        rows: usize,
        thickness: f32,
        line_color: Color,
        square_colors: Option<[Color; 2]>,
    ) -> GameResult<Rc<Mesh>> {
        self.make_mesh(
            ctx,
            format!(
                "grid_{}_{}_{}_{:?}_{:?}",
                width, height, thickness, line_color, square_colors
            ),
            &(|builder| {
                let xspacing = width / cols as f32;
                let yspacing = height / rows as f32;
                let mut color_idx = 0;

                if let Some(square_colors) = square_colors {
                    for x in 0..cols {
                        for y in 0..rows {
                            let x = x as f32;
                            let y = y as f32;
                            builder.rectangle(
                                DrawMode::fill(),
                                Rect::new(x * xspacing, y * yspacing, xspacing, yspacing),
                                square_colors[color_idx],
                            );
                            color_idx = if color_idx == 0 { 1 } else { 0 }
                        }
                        color_idx = if color_idx == 0 { 1 } else { 0 } //reset for next line
                    }
                }

                for x in 1..cols {
                    let x = x as f32;
                    builder.line(
                        &[pt(x * xspacing, 0.), pt(x * xspacing, height)],
                        thickness,
                        line_color,
                    )?;
                }

                for y in 1..rows {
                    let y = y as f32;
                    builder.line(
                        &[pt(0., y * yspacing), pt(width, y * yspacing)],
                        thickness,
                        line_color,
                    )?;
                }

                Ok(())
            }),
        )
    }

    pub fn make_triangle(
        &mut self,
        ctx: &mut Context,
        width: f32,
        height: f32,
        dir: Direction,
    ) -> GameResult<Rc<Mesh>> {
        self.make_mesh(
            ctx,
            format!("box_{}_{}_{:?}", width, height, dir),
            &(|builder| {
                match dir {
                    Direction::Up => builder.triangles(
                        &[pt(0., height), pt(width, height), pt(width * 0.5, 0.)],
                        WHITE,
                    )?,
                    Direction::Down => builder
                        .triangles(&[pt(0., 0.), pt(width, 0.), pt(width * 0.5, height)], WHITE)?,
                    Direction::Left => builder.triangles(
                        &[pt(width, 0.), pt(width, height), pt(0., height * 0.5)],
                        WHITE,
                    )?,
                    Direction::Right => builder.triangles(
                        &[pt(0., 0.), pt(0., height), pt(width, height * 0.5)],
                        WHITE,
                    )?,
                };

                Ok(())
            }),
        )
    }

    pub fn make_one_time_polyline(
        &mut self,
        ctx: &mut Context,
        pts: Vec<Point>,
        thickness: f32,
    ) -> GameResult<Rc<Mesh>> {
        let mut mesh_builder = MeshBuilder::new();

        mesh_builder.polyline(DrawMode::stroke(thickness), &pts, WHITE)?;

        let mesh = Rc::new(mesh_builder.build(ctx)?);
        Ok(mesh)
    }

    pub fn make_rect(
        &mut self,
        ctx: &mut Context,
        width: f32,
        height: f32,
        mode: DrawMode,
    ) -> GameResult<Rc<Mesh>> {
        self.make_mesh(
            ctx,
            format!("rect_{}_{}_{:?}", width, height, mode),
            &(|builder| {
                builder.rectangle(mode, Rect::new(0., 0., width, height), WHITE);
                Ok(())
            }),
        )
    }

    pub fn make_circle(
        &mut self,
        ctx: &mut Context,
        offset: f32,
        radius: f32,
        mode: DrawMode,
    ) -> GameResult<Rc<Mesh>> {
        self.make_mesh(
            ctx,
            format!("circle_{}_{:?}", radius, mode),
            &(|builder| {
                builder.circle(
                    mode,
                    pt(offset * 0.5, offset * 0.5),
                    radius,
                    TOLERANCE,
                    WHITE,
                );
                Ok(())
            }),
        )
    }

    pub fn draw_white_text(
        &mut self,
        ctx: &mut Context,
        text: &str,
        position: Point,
        font_size: f32,
        centered: bool,
    ) {
        self.draw_text(ctx, text, position, WHITE, font_size, centered);
    }

    pub fn draw_text(
        &mut self,
        ctx: &mut Context,
        text: &str,
        position: Point,
        color: Color,
        font_size: f32,
        centered: bool,
    ) {
        let text = Text::new(TextFragment {
            text: text.to_string(),
            color: Some(color),
            scale: Some(Scale::uniform(font_size)),
            ..TextFragment::default()
        });
        let mut xy = position;
        if centered {
            xy = pt(position.x - (text.width(ctx) as f32 / 2.), position.y);
        }
        self.draw_mesh(ctx, &text, xy);
    }

    #[allow(clippy::map_entry)] //I don't think this fix is practical
    pub fn make_mesh(
        &mut self,
        ctx: &mut Context,
        key: String,
        builder: &dyn Fn(&mut MeshBuilder) -> GameResult<()>,
    ) -> GameResult<Rc<Mesh>> {
        if self.mesh_cache.contains_key(&key) {
            Ok(self.mesh_cache[&key].clone())
        } else {
            let mut mesh_builder = MeshBuilder::new();

            builder(&mut mesh_builder)?;

            let mesh = Rc::new(mesh_builder.build(ctx)?);
            self.mesh_cache.insert(key, mesh.clone());
            Ok(mesh)
        }
    }

    pub fn draw_mesh<D: Drawable>(&mut self, ctx: &mut Context, mesh: &D, xy: Point) {
        graphics::draw(ctx, mesh, (xy,)).expect("couldn't draw");
    }

    pub fn draw_coloured_mesh<D: Drawable>(
        &mut self,
        ctx: &mut Context,
        mesh: &D,
        xy: Point,
        new_colour: Color,
    ) {
        graphics::draw(ctx, mesh, (xy, new_colour)).expect("couldn't draw");
    }

    // pub fn make_shogi_piece(&mut self, ctx: &mut Context, width: f32, height: f32) -> GameResult<Rc<Mesh>> {}
}
