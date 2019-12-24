use std::i16;
use std::f32::consts::PI;
use super::track::{Track};
use crate::constants::{LEFT,RIGHT};
use crate::mathh::{Clampable};

pub fn to_stereo(s: f32, pan: f32) -> (f32,f32){
    let vr = 0.5 + (pan / 2.0);
    let vl = 1.0 - vr;
    (s * vl, s * vr)
}

pub fn sine_sample(t: f32, hz: f32) -> f32{
    (t * hz * 2.0 * PI).sin()
}

pub fn square_sample(t: f32, hz: f32) -> f32{
    ((t * hz * 2.0 * PI).sin() * 10000.0).cclamp(-1.0, 1.0)
}

pub fn topflat_sine_sample(t: f32, hz: f32) -> f32{
    (t * hz * 2.0 * PI).sin().min(0.0) * 2.0 + 1.0
}

pub fn botflat_sine_sample(t: f32, hz: f32) -> f32{
    (t * hz * 2.0 * PI).sin().max(0.0) * 2.0 - 1.0
}

pub fn triangle_sample(t: f32, hz: f32) -> f32{
    4.0 * ((t * hz) - ((t * hz) + 0.5).floor()).abs() - 1.0
}

pub fn arg_id(a: f32, t: f32) -> f32 { a }

pub fn tone_to_track<ToneF,ArgF>(track: &mut Track, start: usize, duration: usize, mut pan: f32, mut vol: f32, mut hz: f32, tonef: ToneF, panf: ArgF, volf: ArgF, hzf: ArgF)
    where
        ToneF: Fn(f32,f32) -> f32,
        ArgF: Fn(f32,f32) -> f32,
{
    let end = start + duration;
    if end >= track.len(){
        let diff = end - track.len() + 1;
        track.enlongate(diff);
    }
    let sr = track.sample_rate();

    for i in 0..duration{
        let t = i as f32 / sr as f32;
        let (sl,sr) = to_stereo(tonef(t, hz) * vol, pan);
        track.add_sample(sl, start + i, LEFT);
        track.add_sample(sr, start + i, RIGHT);
        pan = panf(pan, t);
        vol = volf(vol, t);
        hz = hzf(hz, t);
    }
}

pub fn sine_wave(track: &mut Track, start: usize, duration: usize, pan: f32, vol: f32, hz: f32){
    tone_to_track(track, start, duration, pan, vol, hz, sine_sample, arg_id, arg_id, arg_id);
}

pub fn square_wave(track: &mut Track, start: usize, duration: usize, pan: f32, vol: f32, hz: f32){
    tone_to_track(track, start, duration, pan, vol, hz, square_sample, arg_id, arg_id, arg_id);
}


