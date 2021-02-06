pub const TOLERANCE: f32 = 0.6;
pub const AI_MOVE_DELAY: f64 = 0.2;
pub const ANIMATION_DURATION: f64 = 0.5;

pub mod games {
    //These are the code names used in the code for swapping scenes, etc
    pub const TICTACTOE: &str = "tictactoe";
    pub const MANCALA: &str = "mancala";
    pub const DRAUGHTS_ENGLISH: &str = "draughts_english";
    pub const DRAUGHTS_INTERNATIONAL: &str = "draughts_international";
    pub const DRAUGHTS_BRAZILIAN: &str = "draughts_brazilian";
    pub const DRAUGHTS_CANADIAN: &str = "draughts_canadian";
    pub const TABLUT: &str = "tablut";
    pub const GO: &str = "go";
    pub const UR: &str = "ur";
    pub const SENET: &str = "senet";
    pub const BLACKHOLE: &str = "blackhole";
    pub const ORDERCHAOS: &str = "orderchaos";
    pub const RITHMOMANCHY: &str = "rithmomanchy";
    pub const SHOGI_MINI: &str = "shogi_mini";
    pub const SHOGI_STANDARD: &str = "shogi_standard";
    pub const SHOGI_MEDIUM: &str = "shogi_medium";
    pub const SHOGI_LARGE: &str = "shogi_large";
    pub const SHOGI_HUGE: &str = "shogi_huge";
    pub const CHESS_STANDARD: &str = "chess_standard";
    pub const CHESS_CHECKLESS: &str = "chess_checkless";
    pub const CHESS_ANDERNACH: &str = "chess_andernach";
    pub const CHESS_GRAND: &str = "chess_grand";
    pub const CHESS_MODERN: &str = "chess_modern";
    pub const CHESS_FOURBOARD: &str = "chess_fourboard";
    pub const CHESS_MINI: &str = "chess_mini";
    pub const CHESS_CAPABLANCA: &str = "chess_capablanca";
    pub const CHESS_HOSTAGE: &str = "chess_hostage";
    pub const CHESS_PROGRESSIVE: &str = "chess_progressive";
    pub const SUBMENU: &str = "submenu-invalid";

    pub const TEST_MENU: &str = "test-menu";
    pub const TEST_COLORS: &str = "test-colours";
    pub const TEST_LETTERS: &str = "test-letters";
}

#[allow(dead_code)]
pub mod colors {
    use ggez::graphics::Color;

    const fn clr(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b, a: 1. }
    }

    pub const WHITE: Color = clr(1., 1., 1.);
    pub const BLACK: Color = clr(0., 0., 0.);
    pub const RED: Color = clr(1., 0., 0.);
    pub const GREEN: Color = clr(0., 1., 0.);
    pub const BLUE: Color = clr(0., 0., 1.);
    pub const DARK_RED: Color = clr(0.4, 0., 0.);
    pub const DARK_GREEN: Color = clr(0., 0.4, 0.);
    pub const DARK_BLUE: Color = clr(0., 0., 0.4);
    pub const LIGHT_RED: Color = clr(1., 0.4, 0.4);
    pub const LIGHT_GREEN: Color = clr(0.4, 1., 0.4);
    pub const LIGHT_BLUE: Color = clr(0.2, 0.6, 1.);
    pub const LIGHT_GRAY: Color = clr(0.7, 0.7, 0.7);
    pub const GRAY: Color = clr(0.5, 0.5, 0.5);
    pub const DARK_GRAY: Color = clr(0.3, 0.3, 0.3);
    pub const CREAM: Color = clr(0.9, 0.85, 0.65);
    pub const DARK_CREAM: Color = clr(0.8, 0.75, 0.55);
    pub const BROWN: Color = clr(0.3, 0.25, 0.2);
    pub const DARK_BROWN: Color = clr(0.2, 0.15, 0.1);
    pub const PIECE_PLAYER: Color = clr(0.9, 0.85, 0.85);
    pub const PIECE_COMPUTER: Color = clr(0.35, 0.25, 0.2);
    pub const APRICOT: Color = clr(1.0, 0.81, 0.62);
    pub const COPPER: Color = clr(0.82, 0.55, 0.28);

    pub const FAINT_RED: Color = clr(1., 0.75, 0.75);
    pub const FAINT_BLUE: Color = clr(0.75, 0.75, 1.);

    pub const FILTER_BLACK: Color = Color {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.4,
    };

    pub const TRANSPARENT: Color = Color {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };

    pub fn alpha(color: Color, amount: f32) -> Color {
        Color { a: amount, ..color }
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
