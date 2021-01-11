use crate::system::mesh_helper::MeshHelper;
use crate::system::PlayState::{ModeSelection, Playing};
use crate::system::Turn::{Computer, Human};
use crate::system::TurnState::SelectingPiece;
use ggez::event::KeyCode;
use ggez::{Context, GameResult};

pub mod find_nearest;
pub mod game_system;
pub mod ggez_ext;
pub mod letter_mesh;
pub mod math;
pub mod mesh_helper;
pub mod neighbours;

pub const NEW_TURN_HUMAN: PlayState = Playing(Human(SelectingPiece));
pub const NEW_TURN_COMPUTER: PlayState = Playing(Computer(SelectingPiece));

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Player {
    Human,
    Computer,
}

impl Player {
    pub fn opposite(&self) -> Player {
        match self {
            Player::Human => Player::Computer,
            Player::Computer => Player::Human,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum PlayState {
    ModeSelection,
    Init,
    Playing(Turn),
    Draw,
    HumanWin,
    ComputerWin,
}

impl PlayState {
    pub fn is_human(&self, turn_state: TurnState) -> bool {
        if let Playing(Human(state)) = self {
            return state == &turn_state;
        }
        false
    }

    pub fn is_computer(&self, turn_state: TurnState) -> bool {
        if let Playing(Computer(state)) = self {
            return state == &turn_state;
        }
        false
    }

    pub fn is_either(&self, turn_state: TurnState) -> bool {
        self.is_human(turn_state) || self.is_computer(turn_state)
    }

    pub fn is_playing(&self) -> bool {
        matches!(self, Playing(_))
    }

    pub fn player(&self) -> Option<Player> {
        match self {
            ModeSelection => None,
            PlayState::Init => None,
            Playing(turn) => match turn {
                Human(_) => Some(Player::Human),
                Computer(_) => Some(Player::Computer),
            },
            PlayState::Draw => None,
            PlayState::HumanWin => None,
            PlayState::ComputerWin => None,
        }
    }

    pub fn supports_input(&self) -> bool {
        matches!(self, Playing(Human(_))) || self == &ModeSelection
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Turn {
    Human(TurnState),
    Computer(TurnState),
}

impl Turn {
    pub fn new(player: Player, turn_state: TurnState) -> Turn {
        match player {
            Player::Human => Turn::Human(turn_state),
            Player::Computer => Turn::Computer(turn_state),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum TurnState {
    SelectingPiece,
    SelectingMove,
    Animating,
}

pub trait Scene {
    fn on_key_down(&mut self, key: KeyCode);
    fn on_key_up(&mut self, _key: KeyCode) -> bool {
        false
    }
    // fn on_mouse_down(&mut self, button: MouseButton, x: f32, y: f32);
    // fn on_mouse_up(&mut self, button: MouseButton, x: f32, y: f32);
    // fn on_mouse_move(&mut self, x: f32, y: f32);
    fn on_keyboard_entry(&mut self, _input: char) {}
    // fn save(&mut self, file: File) -> GameResult<()>;
    // fn load(&mut self, file: File) -> GameResult<()>;
    fn update(&mut self, delta: f64) -> GameResult<()>;
    fn render(&mut self, ctx: &mut Context, mesh_helper: &mut MeshHelper) -> GameResult<()>;
    fn is_complete(&self) -> Option<&'static str> {
        None
    }
    fn play_state(&self) -> PlayState;
}

pub trait Renderer {
    fn render(&mut self, ctx: &mut Context, mesh_helper: &mut MeshHelper) -> GameResult<()>;
}
