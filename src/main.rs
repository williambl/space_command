#![feature(fn_traits)]

mod track;
mod server;

extern crate core;

use std::arch::x86_64::_mm_stream_ps;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use prost::Message;

use serde::{Serialize, Deserialize};
use strum_macros::{EnumDiscriminants, EnumString};
use crate::track::{Easing, Keyframe, Track, TrackData, TrackDataDiscriminants};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    name: String,
    tracks: Vec<Track>
}

fn main() -> Result<(), std::io::Error> {
    let track: Track = Track {
        name: String::from("hi"),
        data_type: TrackDataDiscriminants::F64,
        keyframes: vec![Keyframe { row: 1, value: TrackData::F64(0.3), easing: Easing::Linear(Box::new([0.0, 0.5, 0.6, 1.0])) }]
    };

    let project = Project {
        name: String::from("Project 1"),
        tracks: vec![track]
    };

    let serialized = save_project(&project);
    println!("serialized = {serialized}");

    Ok(())
}

fn save_project(project: &Project) -> String {
    serde_json::to_string(project).expect("Saving should work")
}
