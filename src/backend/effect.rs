use image::Rgb;
use imgfx::{Direction, SortBy};

#[derive(Clone, PartialEq)]
pub enum Effect {
    Or {
        color: Rgb<u8>,
        negate: bool,
        lhs: Option<Vec<String>>,
        rhs: Option<Vec<String>>,
    },
    And {
        color: Rgb<u8>,
        negate: bool,
        lhs: Option<Vec<String>>,
        rhs: Option<Vec<String>>,
    },
    Xor {
        color: Rgb<u8>,
        negate: bool,
        lhs: Option<Vec<String>>,
        rhs: Option<Vec<String>>,
    },
    Left {
        bits: u8,
        negate: bool,
        lhs: Option<Vec<String>>,
    },
    Right {
        bits: u8,
        negate: bool,
        lhs: Option<Vec<String>>,
    },
    Add {
        color: Rgb<u8>,
        lhs: Option<Vec<String>>,
        rhs: Option<Vec<String>>,
    },
    Sub {
        color: Rgb<u8>,
        negate: bool,
        lhs: Option<Vec<String>>,
        rhs: Option<Vec<String>>,
    },
    Mult {
        color: Rgb<u8>,
        lhs: Option<Vec<String>>,
        rhs: Option<Vec<String>>,
    },
    Pow {
        color: Rgb<u8>,
        lhs: Option<Vec<String>>,
        rhs: Option<Vec<String>>,
    },
    Div {
        color: Rgb<u8>,
        lhs: Option<Vec<String>>,
        rhs: Option<Vec<String>>,
    },
    Average {
        color: Rgb<u8>,
        lhs: Option<Vec<String>>,
        rhs: Option<Vec<String>>,
    },
    Screen {
        color: Rgb<u8>,
        lhs: Option<Vec<String>>,
        rhs: Option<Vec<String>>,
    },
    Overlay {
        color: Rgb<u8>,
        lhs: Option<Vec<String>>,
        rhs: Option<Vec<String>>,
    },
    Bloom {
        intensity: f64,
        radius: f64,
        min_threshold: u8,
        max_threshold: Option<u8>,
    },
    Sort {
        direction: Direction,
        sort_by: SortBy,
        min_threshold: f64,
        max_threshold: f64,
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
