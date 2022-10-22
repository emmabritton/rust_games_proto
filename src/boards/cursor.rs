use crate::boards::idx_coord::BoardCoord;
use crate::boards::{board_cols, board_rows};
use crate::constants::colors::{BLUE, LIGHT_BLUE};
use crate::constants::Direction;
use crate::system::ggez_ext::keycode_to_direction;
use crate::system::math::{Offset, OffsetTuple, Point};
use crate::system::mesh_helper::MeshHelper;
use ggez::event::KeyCode;
use ggez::graphics::{Color, DrawMode};
use ggez::{Context, GameResult};

#[derive(Debug, Clone)]
pub struct Cursor {
    pub idx: usize,
    pub invalid_squares: Vec<usize>,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            idx: 0,
            invalid_squares: vec![],
        }
    }

    pub fn new_with_invalid(invalid_squares: Vec<usize>) -> Self {
        Cursor {
            idx: 0,
            invalid_squares,
        }
    }
}

impl Cursor {
    fn move_selection(&mut self, dir: Direction) {
        let (mut x, mut y): (isize, isize) = BoardCoord::from(self.idx).into();
        match dir {
            Direction::Up => {
                y -= 1;
                if y < 0 {
                    y = (board_rows() - 1) as isize;
                }
            }
            Direction::Down => {
                y += 1;
                if y >= board_rows() as isize {
                    y = 0;
                }
            }
            Direction::Left => {
                x -= 1;
                if x < 0 {
                    x = (board_cols() - 1) as isize;
                }
            }
            Direction::Right => {
                x += 1;
                if x >= board_cols() as isize {
                    x = 0;
                }
            }
        }
        self.idx = BoardCoord::from((x, y)).idx();
        if self.invalid_squares.contains(&self.idx) {
            self.move_selection(dir);
        }
    }

    pub fn handle_input(&mut self, key: KeyCode) -> bool {
        if let Some(dir) = keycode_to_direction(key) {
            self.move_selection(dir);
            true
        } else {
            false
        }
    }

    pub fn point(&self, board_start: Point, cell_size: f32) -> Point {
        Point::from(BoardCoord::from(self.idx))
            .multiply(cell_size, cell_size)
            .offset_point(board_start)
    }

    fn draw(
        &self,
        ctx: &mut Context,
        mesh_helper: &mut MeshHelper,
        board_start: Point,
        cell_size: f32,
        colour: Color,
    ) -> GameResult<()> {
        let selection_box =
            mesh_helper.make_rect(ctx, cell_size, cell_size, DrawMode::stroke(4.))?;
        mesh_helper.draw_coloured_mesh(
            ctx,
            selection_box.as_ref(),
            self.point(board_start, cell_size),
            colour,
        );
        Ok(())
    }

    pub fn render(
        &self,
        ctx: &mut Context,
        mesh_helper: &mut MeshHelper,
        board_start: Point,
        cell_size: f32,
    ) -> GameResult<()> {
        self.draw(ctx, mesh_helper, board_start, cell_size, LIGHT_BLUE)
    }

    pub fn render_dark(
        &self,
        ctx: &mut Context,
        mesh_helper: &mut MeshHelper,
        board_start: Point,
        cell_size: f32,
    ) -> GameResult<()> {
        self.draw(ctx, mesh_helper, board_start, cell_size, BLUE)
    }
}
