use image::Rgb;
use imgfx::{Direction, SortBy};

#[derive(Clone, Copy, PartialEq)]
pub enum Effect {
    Or {
        color: Rgb<u8>,
        negate: bool,
    },
    And {
        color: Rgb<u8>,
        negate: bool,
    },
    Xor {
        color: Rgb<u8>,
        negate: bool,
    },
    Left {
        bits: u8,
        negate: bool,
    },
    Right {
        bits: u8,
        negate: bool,
    },
    Add {
        color: Rgb<u8>,
    },
    Sub {
        color: Rgb<u8>,
        negate: bool,
    },
    Mult {
        color: Rgb<u8>,
    },
    Pow {
        color: Rgb<u8>,
    },
    Div {
        color: Rgb<u8>,
    },
    Average {
        color: Rgb<u8>,
    },
    Screen {
        color: Rgb<u8>,
    },
    Overlay {
        color: Rgb<u8>,
    },
    Bloom {
        intensity: f32,
        radius: f32,
        min_threshold: u8,
        max_threshold: Option<u8>,
    },
    Sort {
        direction: Direction,
        sort_by: SortBy,
        min_threshold: f32,
        max_threshold: f32,
        reversed: bool,
    },
}

impl Effect {
    pub fn name(&self) -> &'static str {
        match self {
            Effect::Or { .. } => "OR",
            Effect::And { .. } => "AND",
            Effect::Xor { .. } => "XOR",
            Effect::Left { .. } => "Left",
            Effect::Right { .. } => "Right",
            Effect::Add { .. } => "Add",
            Effect::Sub { .. } => "Subtract",
            Effect::Mult { .. } => "Multiply",
            Effect::Pow { .. } => "Power",
            Effect::Div { .. } => "Divide",
            Effect::Average { .. } => "Average",
            Effect::Screen { .. } => "Screen",
            Effect::Overlay { .. } => "Overlay",
            Effect::Bloom { .. } => "Bloom",
            Effect::Sort { .. } => "Sort",
        }
    }
}
