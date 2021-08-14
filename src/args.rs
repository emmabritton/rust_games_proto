use crate::constants::games;
use clap::{App, Arg, ArgMatches};

pub(super) const ARG_GAME: &str = "game";
pub(super) const ARG_RULES: &str = "rules";
pub(super) const ARG_TEST: &str = "graphicstest";

const GAMES: [&str; 17] = [
    games::TICTACTOE,
    games::MANCALA,
    games::DRAUGHTS_BRAZILIAN,
    games::DRAUGHTS_CANADIAN,
    games::DRAUGHTS_INTERNATIONAL,
    games::DRAUGHTS_ENGLISH,
    games::CHESS_CAPABLANCA,
    games::CHESS_PROGRESSIVE,
    games::CHESS_MODERN,
    games::CHESS_HOSTAGE,
    games::CHESS_CHECKLESS,
    games::CHESS_ANDERNACH,
    games::CHESS_GRAND,
    games::CHESS_MINI,
    games::CHESS_STANDARD,
    // games::UR,
    // games::SHOGI_STANDARD,
    // games::SHOGI_MINI,
    // games::SHOGI_MEDIUM,
    // games::SHOGI_LARGE,
    // games::SHOGI_HUGE,
    games::ORDERCHAOS,
    // games::BLACKHOLE,
    // games::SENET,
    games::TABLUT,
    // games::GO,
    // games::RITHMOMANCHY,
];

pub(super) fn args_matches<'a>() -> ArgMatches<'a> {
    App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name(ARG_GAME)
                .short("g")
                .long("game")
                .help("Open game directly")
                .required(false)
                .takes_value(true)
                .multiple(true)
                .possible_values(&GAMES),
        )
        .arg(
            Arg::with_name(ARG_RULES)
                .short("r")
                .help("Print rules instead of opening game")
                .required(false)
                .requires("game")
                .takes_value(false)
                .multiple(false),
        )
        .arg(
            Arg::with_name(ARG_TEST)
                .long("graphicstest")
                .required(false)
                .takes_value(false)
                .multiple(false)
                .hidden(true),
        )
        .get_matches()
}
