extern crate music_gen;
use music_gen::*;
use music_gen::tones::sine_wave;

fn main(){
    let sr = 44100;
    let mut track = music_gen::tones::Track::new(sr, 2);
    sine_wave(&mut track, 0, sr * 2, 0.0, 1.0, 100.0);
    sine_wave(&mut track, 0, sr * 2, 0.5, 1.0, 300.0);
    sine_wave(&mut track, 0, sr * 2, 0.5, 1.0, 600.0);
    sine_wave(&mut track, sr, sr, -0.5, 1.0, 200.0);
    track.normalize();
    track.render("test.wav")
}