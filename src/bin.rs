extern crate music_gen;
use music_gen::tones::*;
use music_gen::theory::*;

fn main(){
    let phrygian = ionian_mode(to_note(NoteName::D, 4), PHRYGIAN);
    print_notes(&phrygian);

    let sr = 44100;
    let mut track = music_gen::tones::Track::new(sr, 2);
    let volf = &hit_lin_quad(10.0,2.0,2.0);
    let mut time = 0;
    for note in phrygian{
        let hz = to_pitch(note);
        tone_to_track(&mut track, time, sr, 1.0, 0.0, 0.0, hz, &triangle_sample, &arg_id, volf, &arg_id);
        time += sr/2;
    }
    track.normalize(0.99);
    track.render("test.wav");
}