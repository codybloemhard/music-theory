extern crate music_gen;
use music_gen::*;
use music_gen::tones::*;

fn main(){
    let sr = 44100;
    let mut track = music_gen::tones::Track::new(sr, 2);
    sine_wave(&mut track, 0, sr * 4, 0.0, 1.0, 100.0);
    sine_wave(&mut track, 0, sr * 4, 0.5, 1.0, 300.0);
    sine_wave(&mut track, 0, sr * 4, 0.5, 1.0, 600.0);
    tone_to_track(&mut track, sr, sr, -0.5, 1.0, 400.0, triangle_sample, arg_id, arg_id, arg_id);
    tone_to_track(&mut track, sr*2, sr, -0.5, 1.0, 400.0, topflat_sine_sample, arg_id, arg_id, arg_id);
    tone_to_track(&mut track, sr*3, sr, -0.5, 1.0, 400.0, square_sample, arg_id, arg_id, arg_id);
    track.normalize();
    track.render("test.wav")
}