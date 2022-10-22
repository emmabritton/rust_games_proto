use clap::{Arg, ArgAction, ArgMatches, command};
use clap::builder::PossibleValuesParser;
use crate::constants::games;

pub(super) const ARG_GAME: &str = "game";
pub(super) const ARG_RULES: &str = "rules";
pub(super) const ARG_TEST: &str = "graphicstest";

const GAMES: [&str; 18] = [
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
    games::SENET,
    games::TABLUT,
    // games::GO,
    // games::RITHMOMANCHY,
];

pub(super) fn args_matches() -> ArgMatches {
    command!()
        .arg(
            Arg::new(ARG_GAME)
                .short('g')
                .long("game")
                .help("Open game directly")
                .num_args(1)
                .value_parser(PossibleValuesParser::new(&GAMES))
        )
        .arg(
            Arg::new(ARG_RULES)
                .short('r')
                .help("Print rules instead of opening game")
                .requires("game")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new(ARG_TEST)
                .long("graphicstest")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(vec![ARG_RULES, ARG_GAME])
                .hide(true)
        )
        .get_matches()
}
