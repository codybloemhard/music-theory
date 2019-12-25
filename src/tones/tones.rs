use super::track::{Track};
use crate::constants::{LEFT,RIGHT};
use super::samples::{sine_sample, square_sample};

pub fn to_stereo(s: f32, pan: f32) -> (f32,f32){
    let vr = 0.5 + (pan / 2.0);
    let vl = 1.0 - vr;
    (s * vl, s * vr)
}

pub fn arg_id(a: f32, _: f32) -> f32 { a }

pub fn tone_to_track<F0,F1,F2,F3>(track: &mut Track, start: usize, duration: usize, 
    vol: f32, mut pan: f32, mut start_vol: f32, mut hz: f32,
    tonef: &F0, panf: &F1, volf: &F2, hzf: &F3)
    where
        F0: Fn(f32,f32) -> f32,
        F1: Fn(f32,f32) -> f32,
        F2: Fn(f32,f32) -> f32,
        F3: Fn(f32,f32) -> f32,
{
    let end = start + duration;
    if end >= track.len(){
        let diff = end - track.len() + 1;
        track.enlongate(diff);
    }
    let sr = track.sample_rate();

    for i in 0..duration{
        let t = i as f32 / sr as f32;
        let (sl,sr) = to_stereo(tonef(t, hz) * vol * start_vol, pan);
        track.add_sample(sl, start + i, LEFT);
        track.add_sample(sr, start + i, RIGHT);
        pan = panf(pan, t);
        start_vol = volf(start_vol, t);
        hz = hzf(hz, t);
    }
}

pub fn sine_wave(track: &mut Track, start: usize, duration: usize, pan: f32, vol: f32, hz: f32){
    tone_to_track(track, start, duration, vol, pan, 0.0, hz, &sine_sample, &arg_id, &arg_id, &arg_id);
}

pub fn square_wave(track: &mut Track, start: usize, duration: usize, pan: f32, vol: f32, hz: f32){
    tone_to_track(track, start, duration, vol, pan, 0.0, hz, &square_sample, &arg_id, &arg_id, &arg_id);
}
