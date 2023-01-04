use serde::{Serialize, Deserialize};
use strum_macros::{EnumDiscriminants, EnumString};

#[derive(Serialize, Deserialize, Debug)]
pub enum Easing {
    Linear
}

#[derive(Serialize, Deserialize, Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString, Serialize, Deserialize))]
pub enum TrackData {
    F32(f32),
    F64(f64),
    F64x2([f64;2]),
    F64x3([f64;3])
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keyframe {
    pub row: i32,
    pub value: TrackData,
    pub easing: Easing
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    pub name: String,
    pub data_type: TrackDataDiscriminants,
    pub keyframes: Vec<Keyframe>
}
