use crate::constants::colors::{BLACK, BLUE, DARK_RED, GREEN, LIGHT_BLUE, RED, WHITE};
use crate::constants::games;
use crate::system::math::Offset;
use crate::system::mesh_helper::MeshHelper;
use crate::system::PlayState::*;
use crate::system::{PlayState, Scene};
use crate::{chess, draughts, graphics_testing, mancala, menu, orderchaos, tablut, tictactoe};
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{Color, DrawMode};
use ggez::input::keyboard::KeyMods;
use ggez::{graphics, timer, Context, GameResult};
use std::collections::HashMap;

pub struct GameSystem {
    mesh_helper: MeshHelper,
    active: Box<dyn Scene>,
    active_name: Option<String>,
    dialog_anim_idx: usize,
    dialog_anim_reset: f64,
}

lazy_static! {
    static ref DIALOG_CONTENTS: HashMap<PlayState, (&'static str, Vec<Color>, f64)> = {
        let mut map = HashMap::new();

        map.insert(Draw, ("Draw", vec![WHITE], 0.));
        map.insert(
            HumanWin,
            ("Player Wins", vec![RED, GREEN, BLUE, LIGHT_BLUE], 0.3),
        );
        map.insert(ComputerWin, ("Computer Wins", vec![RED, DARK_RED], 1.));

        map
    };
}

impl GameSystem {
    pub fn new(ctx: &mut Context) -> Self {
        GameSystem {
            mesh_helper: MeshHelper::new(ctx),
            active: Box::new(menu::controller::Controller::new()),
            active_name: None,
            dialog_anim_idx: 0,
            dialog_anim_reset: 0.,
        }
    }
}

impl GameSystem {
    pub fn start_game(&mut self, game: &str) {
        self.active_name = Some(game.to_string());
        self.active = match game {
            games::TEST_MENU => Box::new(graphics_testing::TestMenu::new()),
            games::TEST_COLORS => Box::new(graphics_testing::colors::TestColours::new()),
            games::TEST_LETTERS => Box::new(graphics_testing::letters::TestLetters::new()),
            games::TICTACTOE => Box::new(tictactoe::controller::Controller::new()),
            games::TABLUT => Box::new(tablut::controller::Controller::new()),
            games::DRAUGHTS_CANADIAN
            | games::DRAUGHTS_BRAZILIAN
            | games::DRAUGHTS_INTERNATIONAL
            | games::DRAUGHTS_ENGLISH => Box::new(draughts::controller::Controller::new(game)),
            games::MANCALA => Box::new(mancala::controller::Controller::new()),
            games::ORDERCHAOS => Box::new(orderchaos::controller::Controller::new()),
            games::CHESS_STANDARD
            | games::CHESS_MINI
            | games::CHESS_GRAND
            | games::CHESS_ANDERNACH
            | games::CHESS_CHECKLESS
            | games::CHESS_HOSTAGE
            | games::CHESS_MODERN
            | games::CHESS_PROGRESSIVE
            | games::CHESS_CAPABLANCA => Box::new(chess::controller::Controller::new(game)),
            _ => panic!("Invalid game: {}", game),
        }
    }

    fn handle_game_over(&mut self, ctx: &mut Context) -> GameResult {
        match self.active.play_state() {
            ModeSelection | Init | Playing(_) => {}
            Draw | HumanWin | ComputerWin => self.draw_dialog(ctx)?,
        }
        Ok(())
    }

    fn draw_dialog(&mut self, ctx: &mut Context) -> GameResult<()> {
        let back = self
            .mesh_helper
            .make_rect(ctx, 300., 100., DrawMode::fill())?;
        let border = self
            .mesh_helper
            .make_rect(ctx, 300., 100., DrawMode::stroke(3.))?;

        let box_start = self.mesh_helper.center().offset(-150, -50);

        let contents = DIALOG_CONTENTS
            .get(&self.active.play_state())
            .expect("Invalid play state");

        self.mesh_helper
            .draw_coloured_mesh(ctx, back.as_ref(), box_start, BLACK);
        self.mesh_helper.draw_coloured_mesh(
            ctx,
            border.as_ref(),
            box_start,
            contents.1[self.dialog_anim_idx],
        );
        self.mesh_helper.draw_white_text(
            ctx,
            contents.0,
            self.mesh_helper.center().offsety(-20),
            40.,
            true,
        );

        Ok(())
    }
}

impl EventHandler for GameSystem {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let delta = timer::duration_to_f64(timer::delta(ctx));

        self.active.update(delta)?;
        if let Some(new_scene) = self.active.is_complete() {
            graphics::set_window_title(ctx, new_scene);
            self.start_game(new_scene);
        }

        match self.active.play_state() {
            ModeSelection | Init | Playing(_) => {}
            Draw | HumanWin | ComputerWin => {
                self.dialog_anim_reset -= delta;
                if self.dialog_anim_reset < 0. {
                    let contents = DIALOG_CONTENTS.get(&self.active.play_state()).unwrap();
                    self.dialog_anim_reset = contents.2;
                    self.dialog_anim_idx += 1;
                    if self.dialog_anim_idx >= contents.1.len() {
                        self.dialog_anim_idx = 0;
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BLACK);

        self.active.render(ctx, &mut self.mesh_helper)?;

        self.handle_game_over(ctx)?;

        // self.mesh_helper.draw_text(
        //     ctx,
        //     &format!("{:.0}", timer::fps(ctx)),
        //     pt(SCREEN_WIDTH - 60., 0.),
        //     RED,
        //     24.,
        //     false,
        // );

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    // fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
    // if let Some(scene) = &mut self.active {
    //     scene.on_mouse_down(button, x, y);
    // }
    // }

    // fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
    //     if let Some(scene) = &mut self.active {
    //         scene.on_mouse_up(button, x, y);
    //     }
    // }

    // fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
    //     if let Some(scene) = &mut self.active {
    //         scene.on_mouse_move(x, y)
    //     }
    // }

    fn key_down_event(
        &mut self,
        _: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if self.active.play_state().supports_input() {
            self.active.on_key_down(keycode)
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        if !self.active.play_state().supports_input() || !self.active.on_key_up(keycode) {
            match (keycode, keymods) {
                (KeyCode::W, KeyMods::LOGO)
                | (KeyCode::F4, KeyMods::ALT)
                | (KeyCode::C, KeyMods::CTRL)
                | (KeyCode::Q, KeyMods::LOGO)
                | (KeyCode::Escape, KeyMods::NONE) => ggez::event::quit(ctx),
                (KeyCode::R, KeyMods::LOGO) | (KeyCode::R, KeyMods::CTRL) => {
                    if let Some(active_game) = self.active_name.clone() {
                        self.start_game(&active_game);
                    }
                }
                (_, _) => {}
            }
        }
    }

    fn text_input_event(&mut self, _ctx: &mut Context, chr: char) {
        if self.active.play_state().supports_input() {
            self.active.on_keyboard_entry(chr)
        }
    }
}
