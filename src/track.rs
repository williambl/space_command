use serde::{Serialize, Deserialize};
use strum_macros::{EnumDiscriminants, EnumString};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum StepsDir {
    Start,
    End,
    Both,
    None
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString, Serialize, Deserialize))]
pub enum Easing {
    Linear(Box<[f64]>),
    CubicBezier{
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64
    },
    Steps {
        num_steps: usize,
        direction: StepsDir
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString, Serialize, Deserialize))]
pub enum TrackData {
    F32(f32),
    F64(f64),
    F64x2([f64;2]),
    F64x3([f64;3])
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keyframe {
    pub row: i32,
    pub value: TrackData,
    pub easing: Easing
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Track {
    pub name: String,
    pub data_type: TrackDataDiscriminants,
    pub keyframes: Vec<Keyframe>
}
