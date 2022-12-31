use std::collections::HashMap;
use std::iter::Map;
use splines::{Interpolation, Key, Spline};

pub trait Keyframeable<T> {
    fn keyframe(&self, time: i32) -> Option<&Key<f64, T>>;
    fn set_keyframe(&mut self, time: i32, value: T, interpolation: Interpolation<f64, T>) -> ();
    fn remove_keyframe(&mut self, time: i32) -> bool;
}

#[derive(Debug)]
pub struct Track<T> {
    name: String,
    keyframes: Spline<f64, T>
}

impl<T> Keyframeable<T> for Track<T> {
    fn keyframe(&self, time: i32) -> Option<&Key<f64, T>> {
        self.keyframes.keys().iter().find(|key| key.t == f64::from(time))
    }

    fn set_keyframe(&mut self, time: i32, value: T, interpolation: Interpolation<f64, T>) -> () {
        self.keyframes.add(Key::new(f64::from(time), value, interpolation));
    }

    fn remove_keyframe(&mut self, time: i32) -> bool {
        match self.keyframes.keys().iter().position(|key| key.t == f64::from(time)) {
            Some(x) => self.keyframes.remove(x).is_some(),
            None => false
        }
    }
}

fn main() {
    let mut track: Track<f64> = Track {
        name: String::from("hi"),
        keyframes: Spline::from_vec(vec![])
    };

    track.set_keyframe(0, 0.3, Interpolation::Linear);
    track.set_keyframe(2, 0.6, Interpolation::Linear);

    println!("{:#?}", track.keyframe(0).expect("Should work"));
    println!("{:#?}", track.keyframes.sample(1.0).expect("Should work"))
}
