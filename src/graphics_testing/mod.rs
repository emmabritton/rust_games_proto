use crate::constants::colors::WHITE;
use crate::constants::games::{TEST_COLORS, TEST_LETTERS};
use crate::constants::Direction;
use crate::system::math::{Offset, pt, WrappedUsize};
use crate::system::mesh_helper::MeshHelper;
use crate::system::{PlayState, Scene, NEW_TURN_HUMAN};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

pub mod colors;
pub mod letters;

const ITEMS: [(&str, &str); 2] = [(TEST_COLORS, "Colours"), (TEST_LETTERS, "Letters")];

struct State {
    cursor: WrappedUsize,
}

pub struct TestMenu {
    state: State,
    change_scene: Option<&'static str>,
}

impl TestMenu {
    pub fn new() -> Self {
        TestMenu {
            state: State {
                cursor: WrappedUsize::new_zero_based(ITEMS.len()),
            },
            change_scene: None,
        }
    }
}

impl TestMenu {
    fn select(&mut self) {
        self.change_scene = Some(ITEMS[self.state.cursor.value].0);
    }
}

impl Scene for TestMenu {
    fn on_key_down(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => self.state.cursor.dec(),
            KeyCode::Down => self.state.cursor.inc(),
            KeyCode::Return => self.select(),
            _ => {}
        }
    }

    fn on_keyboard_entry(&mut self, _: char) {}

    fn update(&mut self, _: f64) -> GameResult<()> {
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context, mesh_helper: &mut MeshHelper) -> GameResult<()> {
        render(ctx, mesh_helper, &self.state)
    }

    fn is_complete(&self) -> Option<&'static str> {
        self.change_scene
    }

    fn play_state(&self) -> PlayState {
        NEW_TURN_HUMAN
    }
}

fn render(ctx: &mut Context, mesh_helper: &mut MeshHelper, state: &State) -> GameResult<()> {
    let menu_start = pt(34., 100.);
    let cursor_start = pt(16., 100.);
    let cursor = mesh_helper.make_triangle(ctx, 12., 12., Direction::Right)?;

    mesh_helper.draw_mesh(
        ctx,
        cursor.as_ref(),
        cursor_start.offsety(state.cursor.value * 16),
    );

    ITEMS.iter().enumerate().for_each(|(idx, item)| {
        mesh_helper.draw_text(ctx, item.1, menu_start.offsety(idx * 16), WHITE, 14., false);
    });

    Ok(())
}
