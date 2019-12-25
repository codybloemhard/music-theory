extern crate music_gen;
use music_gen::tones::*;
use music_gen::theory::*;

fn main(){
    let phrygian = mode_of_scale(ionian_scale_steps(), PHRYGIAN);
    let g_phr = scale_notes(&phrygian, to_note(NoteName::G, 3));
    for note in &g_phr{
        println!("{}", to_note_name(*note));
    }

    let sr = 44100;
    let mut track = music_gen::tones::Track::new(sr, 2);
    let volf = &hit_lin_quad(10.0,2.0,2.0);
    let mut time = 0;
    for note in g_phr{
        let hz = to_pitch(note);
        tone_to_track(&mut track, time, sr, 1.0, 0.0, 0.0, hz, &triangle_sample, &arg_id, volf, &arg_id);
        time += sr/2;
    }
    track.normalize(0.95);
    track.render("test.wav");
}