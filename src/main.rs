#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate variantly;

use crate::args::args_matches;
use crate::constants::games::TEST_MENU;
use crate::menu::print_rules;
use crate::system::game_system::GameSystem;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{event, graphics, ContextBuilder};
use std::env;
use std::path::PathBuf;

const SCREEN_WIDTH: f32 = 1280.;
const SCREEN_HEIGHT: f32 = 1024.;
#[cfg(debug_assertions)]
const LOGGING_ENABLED: bool = true;
#[cfg(not(debug_assertions))]
const LOGGING_ENABLED: bool = false;

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
mod system;
mod tablut;
mod tictactoe;
mod tree;

fn main() {
    debug_log!("Games starting...");
    let matches = args_matches();

    if matches.is_present("rules") {
        debug_log!("Rules only");
        let game = matches.value_of("game").unwrap();
        print_rules(game);
    } else {
        let (ctx, event_loop) = &mut setup_ggez()
            .build()
            .expect("Could not create ggez context!");

        let mut system = GameSystem::new(ctx);

        debug_log!("Games started");

        if matches.is_present("graphicstest") {
            system.start_game(TEST_MENU);
        } else if matches.value_of("game").is_some() {
            let game = matches.value_of("game").unwrap();
            debug_log!("Game specified from args: {}", game);
            graphics::set_window_title(ctx, game);
            system.start_game(game);
        }

        match event::run(ctx, event_loop, &mut system) {
            Ok(_) => println!("Exited cleanly"),
            Err(e) => println!("Error occurred: {}", e),
        }
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
        PathBuf::from("./resources");
    }

    cb
}
