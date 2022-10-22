#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate variantly;

use crate::args::{ARG_GAME, ARG_RULES, ARG_TEST, args_matches};
use crate::constants::games::TEST_MENU;
use crate::menu::print_rules;
use crate::system::game_system::GameSystem;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{event, graphics, ContextBuilder};
use std::env;
use std::path::PathBuf;
use ggez::event::KeyCode::P;

const SCREEN_WIDTH: f32 = 1280.;
const SCREEN_HEIGHT: f32 = 1024.;
#[cfg(debug_assertions)]
const LOGGING_ENABLED: bool = true;
#[cfg(not(debug_assertions))]
const LOGGING_ENABLED: bool = false;

#[cfg(debug_assertions)]
const FPS_ENABLED: bool = true;
#[cfg(not(debug_assertions))]
const FPS_ENABLED: bool = false;

#[macro_use]
mod macros;
mod args;
mod boards;
mod chess;
mod constants;
mod draughts;
mod ext;
mod graphics_testing;
mod mancala;
mod menu;
mod orderchaos;
mod senet;
mod system;
mod tablut;
mod tictactoe;
mod tree;

fn main() {
    debug_log!("Games starting...");
    let matches = args_matches();

    if let Some(true) =  matches.get_one(ARG_RULES) {
        debug_log!("Rules only");
        let game: &String = matches.get_one(ARG_GAME).unwrap();
        print_rules(&game);
    } else {
        let (mut ctx, event_loop) = setup_ggez()
            .build()
            .expect("Could not create ggez context!");

        let mut system = GameSystem::new(&mut ctx);

        debug_log!("Games started");

        if let Some(true) = matches.get_one(ARG_TEST) {
            system.start_game(TEST_MENU);
        } else {
            if let Some(game) = matches.get_one::<&String>(ARG_GAME) {
                debug_log!("Game specified from args: {}", game);
                graphics::set_window_title(&ctx, game);
                system.start_game(game);
            }
        }

        event::run(ctx, event_loop,  system)
    }
}

fn setup_ggez() -> ContextBuilder {
    let mut cb = ContextBuilder::new("games", "Ray Britton")
        .window_mode(WindowMode {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            ..WindowMode::default()
        })
        .window_setup(WindowSetup {
            title: String::from("Games"),
            ..WindowSetup::default()
        });

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        //        println!("Adding path {:?} from manifest", path);
        cb = cb.add_resource_path(path);
    } else {
        cb = cb.add_resource_path(PathBuf::from("./resources"));
    }

    cb
}
