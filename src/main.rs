#![feature(fn_traits)]
extern crate core;

use std::fmt::Debug;

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
    row: i32,
    value: TrackData,
    easing: Easing
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    name: String,
    data_type: TrackDataDiscriminants,
    keyframes: Vec<Keyframe>
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
    tracks: Vec<Track>
}

fn main() {
    let track: Track = Track {
        name: String::from("hi"),
        data_type: TrackDataDiscriminants::F64,
        keyframes: vec![Keyframe { row: 1, value: TrackData::F64(0.3), easing: Easing::Linear }]
    };

    let project = Project {
        name: String::from("Project 1"),
        tracks: vec![track]
    };

    let serialized = save_project(&project);
    println!("serialized = {serialized}");
}

fn save_project(project: &Project) -> String {
    serde_json::to_string(project).expect("Saving should work")
}
