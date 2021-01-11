use crate::boards::idx_coord::BoardCoord;
use crate::boards::set_board_size;
use crate::constants::colors::*;
use crate::system::math::{Offset, Point, WrappedUsize};
use crate::system::mesh_helper::MeshHelper;
use crate::system::{PlayState, Scene, NEW_TURN_HUMAN};
use ggez::graphics::{Color, DrawMode};
use ggez::input::keyboard::KeyCode;
use ggez::{graphics, Context, GameResult};

const COLOURS: [Color; 24] = [
    WHITE,
    BLACK,
    TRANSPARENT,
    RED,
    LIGHT_RED,
    DARK_RED,
    GREEN,
    LIGHT_GREEN,
    DARK_GREEN,
    BLUE,
    LIGHT_BLUE,
    DARK_BLUE,
    GRAY,
    LIGHT_GRAY,
    DARK_GRAY,
    CREAM,
    DARK_CREAM,
    BROWN,
    DARK_BROWN,
    PIECE_COMPUTER,
    PIECE_PLAYER,
    FAINT_BLUE,
    FAINT_RED,
    FILTER_BLACK,
];
const COLOUR_NAMES: [&str; 24] = [
    "White",
    "Black",
    "Transparent",
    "Red",
    "Light Red",
    "Dark Red",
    "Green",
    "Light Green",
    "Dark Green",
    "Blue",
    "Light Blue",
    "Dark Blue",
    "Gray",
    "Light Gray",
    "Dark Gray",
    "Cream",
    "Dark Cream",
    "Brown",
    "Dark Brown",
    "Piece - Computer",
    "Piece - Player",
    "Faint Blue",
    "Faint Red",
    "Filter Black",
];

const BACKGROUND: [Color; 2] = [BLACK, WHITE];
const TEXT: [Color; 2] = [WHITE, BLACK];

pub struct TestColours {
    background: WrappedUsize,
    text: WrappedUsize,
}

impl TestColours {
    pub fn new() -> Self {
        set_board_size((10, 10));
        TestColours {
            background: WrappedUsize::new_zero_based(2),
            text: WrappedUsize::new_zero_based(2),
        }
    }
}

impl Scene for TestColours {
    fn on_key_down(&mut self, key: KeyCode) {
        if let KeyCode::Space = key {
            self.background.inc();
            self.text.inc();
        }
    }

    fn on_keyboard_entry(&mut self, _: char) {}

    fn update(&mut self, _: f64) -> GameResult<()> {
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context, mesh_helper: &mut MeshHelper) -> GameResult<()> {
        graphics::clear(ctx, BACKGROUND[self.background.value]);

        let offset = mesh_helper.calc_width(0.02);
        let square_size = mesh_helper.calc_width(0.07);
        let square = mesh_helper.make_rect(ctx, square_size, square_size, DrawMode::fill())?;
        for i in 0..COLOURS.len() {
            let coord = BoardCoord::from(i);
            let mut pt: Point = coord.into();
            pt = pt
                .multiply(square_size, square_size)
                .offset(offset, offset)
                .offset(coord.0 as f32 * offset, coord.1 as f32 * offset);
            mesh_helper.draw_coloured_mesh(ctx, square.as_ref(), pt, COLOURS[i]);
            mesh_helper.draw_text(
                ctx,
                COLOUR_NAMES[i],
                pt.offset(square_size * 0.5, square_size),
                TEXT[self.text.value],
                12.,
                true,
            );
        }
        Ok(())
    }

    fn play_state(&self) -> PlayState {
        NEW_TURN_HUMAN
    }
}
