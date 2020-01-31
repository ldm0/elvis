use elvis::Colors as ElvisColors;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Colors {
    Amber,
    AmberAccent,
    Black,
    Blue,
    BlueAccent,
    BlueGrey,
    Brown,
    Cyan,
    CyanAccent,
    DeepOrange,
    DeepOrangeAccent,
    DeepPurple,
    DeepPurpleAccent,
    Green,
    GreenAccent,
    Grey,
    Indigo,
    IndigoAccent,
    LightBlue,
    LightBlueAccent,
    LightGreen,
    LightGreenAccent,
    Lime,
    LimeAccent,
    Orange,
    OrangeAccent,
    Pink,
    PinkAccent,
    Purple,
    PurpleAccent,
    Red,
    RedAccent,
    Teal,
    TealAccent,
    Transparent,
    White,
    Yellow,
    YellowAccent,
}

impl Colors {
    pub fn elvis(&self) -> ElvisColors {
        match self {
            Self::Amber => ElvisColors::Amber,
            Self::AmberAccent => ElvisColors::AmberAccent,
            Self::Black => ElvisColors::Black,
            Self::Blue => ElvisColors::Blue,
            Self::BlueAccent => ElvisColors::BlueAccent,
            Self::BlueGrey => ElvisColors::BlueGrey,
            Self::Brown => ElvisColors::Brown,
            Self::Cyan => ElvisColors::Cyan,
            Self::CyanAccent => ElvisColors::CyanAccent,
            Self::DeepOrange => ElvisColors::DeepOrange,
            Self::DeepOrangeAccent => ElvisColors::DeepOrangeAccent,
            Self::DeepPurple => ElvisColors::DeepPurple,
            Self::DeepPurpleAccent => ElvisColors::DeepPurpleAccent,
            Self::Green => ElvisColors::Green,
            Self::GreenAccent => ElvisColors::GreenAccent,
            Self::Grey => ElvisColors::Grey,
            Self::Indigo => ElvisColors::Indigo,
            Self::IndigoAccent => ElvisColors::IndigoAccent,
            Self::LightBlue => ElvisColors::LightBlue,
            Self::LightBlueAccent => ElvisColors::LightBlueAccent,
            Self::LightGreen => ElvisColors::LightGreen,
            Self::LightGreenAccent => ElvisColors::LightGreenAccent,
            Self::Lime => ElvisColors::Lime,
            Self::LimeAccent => ElvisColors::LimeAccent,
            Self::Orange => ElvisColors::Orange,
            Self::OrangeAccent => ElvisColors::OrangeAccent,
            Self::Pink => ElvisColors::Pink,
            Self::PinkAccent => ElvisColors::PinkAccent,
            Self::Purple => ElvisColors::Purple,
            Self::PurpleAccent => ElvisColors::PurpleAccent,
            Self::Red => ElvisColors::Red,
            Self::RedAccent => ElvisColors::RedAccent,
            Self::Teal => ElvisColors::Teal,
            Self::TealAccent => ElvisColors::TealAccent,
            Self::Transparent => ElvisColors::Transparent,
            Self::White => ElvisColors::White,
            Self::Yellow => ElvisColors::Yellow,
            Self::YellowAccent => ElvisColors::YellowAccent,
        }
    }
}