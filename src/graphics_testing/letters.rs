use crate::boards::idx_coord::BoardCoord;
use crate::boards::set_board_size;
use crate::constants::colors::LIGHT_GRAY;
use crate::system::letter_mesh::*;
use crate::system::math::{pt, Offset, Point};
use crate::system::mesh_helper::MeshHelper;
use crate::system::{PlayState, Scene, NEW_TURN_HUMAN};
use ggez::event::KeyCode;
use ggez::{Context, GameResult};

#[derive(Default)]
pub(crate) struct TestLetters {
    grid: bool,
    debug: bool,
}

impl TestLetters {
    pub fn new() -> Self {
        set_board_size((10, 10));
        TestLetters::default()
    }
}

impl Scene for TestLetters {
    fn on_key_down(&mut self, key: KeyCode) {
        match key {
            KeyCode::Space => self.grid = !self.grid,
            KeyCode::L => self.debug = !self.debug,
            _ => {}
        }
    }

    fn update(&mut self, _: f64) -> GameResult<()> {
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context, mesh_helper: &mut MeshHelper) -> GameResult<()> {
        let size = mesh_helper.calc_height(0.1);
        let grid =
            mesh_helper.make_grid(ctx, size * 10., size * 10., 10, 10, 2., LIGHT_GRAY, None)?;

        if self.grid {
            mesh_helper.draw_mesh(ctx, grid.as_ref(), pt(0., 0.));
        }

        [
            'a', 'd', 'k', 'o', 'x', 'q', 'r', 'b', 'p', 'm', 'w', 'j', 'c', 'n',
        ]
        .iter()
        .enumerate()
        .for_each(|(idx, letter)| {
            let coord = BoardCoord::from(idx);
            let mesh = make_letter_mesh(ctx, mesh_helper, size, *letter).unwrap();
            let pos = Point::from(coord).multiply(size, size);
            mesh_helper.draw_mesh(ctx, mesh.as_ref(), pos);
            if self.debug {
                mesh_helper.draw_white_text(
                    ctx,
                    &letter.to_string(),
                    pos.offset(8., 2.),
                    14.,
                    false,
                );
            }
        });

        Ok(())
    }

    fn play_state(&self) -> PlayState {
        NEW_TURN_HUMAN
    }
}
