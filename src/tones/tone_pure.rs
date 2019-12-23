use std::i16;
use std::f32::consts::PI;
use super::track::{Track};
use crate::constants::{LEFT,RIGHT};

pub fn sine_sample(t: f32, pan: f32, vol: f32, hz: f32) -> (f32,f32){
    let vr = 0.5 + (pan / 2.0);
    let vl = 1.0 - vr;
    let sl = (t * hz * 2.0 * PI).sin() * vol * vl;
    let sr = (t * hz * 2.0 * PI).sin() * vol * vr;
    (sl,sr)
}

pub fn arg_id(a: f32, t: f32) -> f32 { a }

pub fn tone_to_track<ToneF,ArgF>(track: &mut Track, start: usize, duration: usize, mut pan: f32, mut vol: f32, mut hz: f32, tonef: ToneF, panf: ArgF, volf: ArgF, hzf: ArgF)
    where
        ToneF: Fn(f32,f32,f32,f32) -> (f32,f32),
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
        let (sl,sr) = tonef(t, pan, vol, hz);
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
