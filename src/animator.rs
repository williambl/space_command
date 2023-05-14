use crate::project::Project;
use crate::track::{Easing, Keyframe, Track, TrackData, TrackDataDiscriminants};

pub trait Animator {
    fn get_value(&self, track: Track, row: f64) -> TrackData;
}

impl Animator for Project {
    fn get_value(&self, track: Track, row: f64) -> TrackData {
        let frame_before = track.keyframes.iter()
            .filter(|k| k.row <= row as i32)
            .last()
            .unwrap_or_else(|| &Keyframe {
                row: row as i32,
                value: default_value(track.data_type),
                easing: Easing::Linear(Box::new([0.0, 1.0])),
            });

        let frame_after = track.keyframes.iter()
            .filter(|k| k.row > row as i32)
            .next()
            .unwrap_or_else(|| &Keyframe {
                row: row as i32 + 1,
                value: default_value(track.data_type),
                easing: Easing::Linear(Box::new([0.0, 1.0])),
            });

        let factor = lerp_factor(row, &frame_before.easing);
        lerp(&frame_before.value, &frame_after.value, factor)
    }
}

fn default_value(data_type: TrackDataDiscriminants) -> TrackData {
    match data_type {
        TrackDataDiscriminants::F32 => TrackData::F32(0.0),
        TrackDataDiscriminants::F64 => TrackData::F64(0.0),
        TrackDataDiscriminants::F64x2 => TrackData::F64x2([0.0, 0.0]),
        TrackDataDiscriminants::F64x3 => TrackData::F64x3([0.0, 0.0, 0.0]),
    }
}

fn lerp_factor(row: f64, easing: &Easing) -> f64 {
    match easing {
        Easing::Linear(points) => {todo!()}
        Easing::CubicBezier { x1, x2, y1, y2 } => {todo!()}
        Easing::Steps { num_steps, direction } => {todo!()}
    }
}

fn lerp(a: &TrackData, b: &TrackData, factor: f64) -> TrackData {
    match a {
        TrackData::F32(x) => {
            if let TrackData::F32(y) = b {
                TrackData::F32(lerp_numbers(*x, *y, factor))
            } else {
                panic!("Mismatched track data types")
            }
        }
        TrackData::F64(x) => {
            if let TrackData::F64(y) = b {
                TrackData::F64(lerp_numbers(*x, *y, factor))
            } else {
                panic!("Mismatched track data types")
            }
        }
        TrackData::F64x2(xs) => {
            if let TrackData::F64x2(ys) = b {
                TrackData::F64x2([
                    lerp_numbers(xs[0], ys[0], factor),
                    lerp_numbers(xs[1], ys[1], factor)
                ])
            } else {
                panic!("Mismatched track data types")
            }
        }
        TrackData::F64x3(xs) => {
            if let TrackData::F64x3(ys) = b {
                TrackData::F64x3([
                    lerp_numbers(xs[0], ys[0], factor),
                    lerp_numbers(xs[1], ys[1], factor),
                    lerp_numbers(xs[2], ys[2], factor)
                ])
            } else {
                panic!("Mismatched track data types")
            }
        }
    }
}

fn lerp_numbers<T>(a: T, b: T, factor: f64) -> T {
    a * (1.0 - factor) + b * factor
}