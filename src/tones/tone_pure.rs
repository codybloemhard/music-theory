use std::i16;
use std::f32::consts::PI;
use super::track::{Track};
use crate::constants::{LEFT,RIGHT};

pub fn sine_wave(track: &mut Track, start: usize, duration: usize, pan: f32, vol: f32, hz: f32){
    let end = start + duration;
    if end >= track.len(){
        let diff = end - track.len() + 1;
        track.enlongate(diff);
    }
    let sr = track.sample_rate();
    let vr = 0.5 + (pan / 2.0);
    let vl = 1.0 - vr;
    for i in 0..duration{
        let t = i as f32 / sr as f32;
        let sl = (t * hz * 2.0 * PI).sin() * vol * vl;
        let sr = (t * hz * 2.0 * PI).sin() * vol * vr;
        track.add_sample(sl, start + i, LEFT);
        track.add_sample(sr, start + i, RIGHT);
    }
}
