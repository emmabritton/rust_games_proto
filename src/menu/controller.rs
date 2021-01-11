use crate::menu::menu_items::ITEMS;
use crate::menu::renderer::render;
use crate::menu::State;
use crate::system::math::WrappedUsize;
use crate::system::mesh_helper::MeshHelper;
use crate::system::{PlayState, Scene, NEW_TURN_HUMAN};
use ggez::event::KeyCode;
use ggez::{Context, GameResult};

const OPEN: [KeyCode; 3] = [KeyCode::Return, KeyCode::Space, KeyCode::Right];
const CLOSE: [KeyCode; 3] = [KeyCode::Escape, KeyCode::Back, KeyCode::Left];
const DOWN: KeyCode = KeyCode::Down;
const UP: KeyCode = KeyCode::Up;

pub struct Controller {
    state: State,
    change_scene: Option<&'static str>,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            state: State {
                cursor: WrappedUsize::new_zero_based(ITEMS.len()),
                subcursor: None,
            },
            change_scene: None,
        }
    }
}

impl Controller {
    fn cursor_up(&mut self) {
        match &mut self.state.subcursor {
            None => self.state.cursor.dec(),
            Some(subcursor) => subcursor.dec(),
        }
    }

    fn cursor_down(&mut self) {
        match &mut self.state.subcursor {
            None => self.state.cursor.inc(),
            Some(subcursor) => subcursor.inc(),
        }
    }

    fn open_submenu(&mut self) {
        self.state.subcursor = Some(WrappedUsize::new_zero_based(
            ITEMS[self.state.cursor.value].1.as_ref().unwrap().len(),
        ));
    }

    fn close_submenu(&mut self) {
        self.state.subcursor = None;
    }

    fn selected_has_submenu(&mut self) -> bool {
        self.state.subcursor.is_none() && ITEMS[self.state.cursor.value].1.is_some()
    }

    fn open_selected(&mut self) {
        let name = match &self.state.subcursor {
            None => ITEMS[self.state.cursor.value].0.code,
            Some(subcursor) => {
                ITEMS[self.state.cursor.value].1.as_ref().unwrap()[subcursor.value].code
            }
        };
        debug_log_start!("Starting game {} from menu", name);
        self.change_scene = Some(name)
    }
}

impl Scene for Controller {
    fn on_key_down(&mut self, key: KeyCode) {
        if key == UP {
            self.cursor_up();
        } else if key == DOWN {
            self.cursor_down();
        }
    }

    fn on_key_up(&mut self, key: KeyCode) -> bool {
        if OPEN.contains(&key) {
            if self.selected_has_submenu() {
                self.open_submenu();
            } else {
                self.open_selected();
            }
            return true;
        } else if CLOSE.contains(&key) && self.state.subcursor.is_some() {
            self.close_submenu();
            return true;
        }
        false
    }

    fn on_keyboard_entry(&mut self, _: char) {
        //not used
    }

    fn update(&mut self, _: f64) -> GameResult<()> {
        //not used
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
