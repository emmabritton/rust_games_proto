use crate::draughts::board_index_to_pdn_num;
use crate::tree::LeafPrint;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct Capture {
    pub(super) dest: usize,
    pub(super) capturing: usize,
}

#[derive(Debug, Clone, Variantly, Eq, PartialEq)]
pub(super) enum Move {
    Step {
        origin: usize,
        dest: usize,
        value: usize,
    },
    Jump {
        origin: usize,
        capture: Capture,
        value: usize,
    },
    MultiJump {
        origin: usize,
        captures: Vec<Capture>,
        value: usize,
    },
}

impl Move {
    pub(super) fn origin(&self) -> usize {
        *match self {
            Move::Step {
                origin,
                dest: _,
                value: _,
            } => origin,
            Move::Jump {
                origin,
                capture: _,
                value: _,
            } => origin,
            Move::MultiJump {
                origin,
                captures: _,
                value: _,
            } => origin,
        }
    }

    pub(super) fn value(&self) -> usize {
        *match self {
            Move::Step {
                origin: _,
                dest: _,
                value,
            } => value,
            Move::Jump {
                origin: _,
                capture: _,
                value,
            } => value,
            Move::MultiJump {
                origin: _,
                captures: _,
                value,
            } => value,
        }
    }

    pub(super) fn dest(&self) -> usize {
        match self {
            Move::Step {
                origin: _,
                dest,
                value: _,
            } => *dest,
            Move::Jump {
                origin: _,
                capture,
                value: _,
            } => capture.dest,
            Move::MultiJump {
                origin: _,
                captures,
                value: _,
            } => captures.last().unwrap().dest,
        }
    }

    pub(super) fn desc(&self) -> String {
        match self {
            Move::Step {
                origin: _,
                dest,
                value: _,
            } => format!("Step({})", dest),
            Move::Jump {
                origin: _,
                capture,
                value: _,
            } => format!("Jump({})", capture.dest),
            Move::MultiJump {
                origin: _,
                captures,
                value: _,
            } => format!(
                "MultiJump({},{})",
                captures.len(),
                captures.last().unwrap().dest
            ),
        }
    }

    pub(super) fn len(&self) -> usize {
        match self {
            Move::Step { .. } => 1,
            Move::Jump { .. } => 1,
            Move::MultiJump {
                origin: _,
                captures,
                value: _,
            } => captures.len(),
        }
    }
}

impl LeafPrint for Move {
    fn id(&self) -> String {
        format!("{}", self.origin())
    }

    fn desc(&self) -> String {
        self.desc()
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Move::Step {
                origin,
                dest,
                value: _,
            } => write!(
                f,
                "{}-{}",
                board_index_to_pdn_num(*origin),
                board_index_to_pdn_num(*dest)
            ),
            Move::Jump {
                origin,
                capture,
                value: _,
            } => write!(
                f,
                "{}x{}",
                board_index_to_pdn_num(*origin),
                board_index_to_pdn_num(capture.dest)
            ),
            Move::MultiJump {
                origin,
                captures,
                value: _,
            } => {
                let steps: Vec<String> = captures
                    .iter()
                    .map(|cap| format!("x{}", board_index_to_pdn_num(cap.dest)))
                    .collect();
                write!(f, "{}{}", board_index_to_pdn_num(*origin), steps.join(""))
            }
        }
    }
}
