extern crate music_gen;
use music_gen::tones::*;
use music_gen::theory::*;

fn main(){
    let phrygian = mode_of_scale(ionian_scale_steps(), PHRYGIAN);
    let g_phr = scale_notes(&phrygian, to_note(NoteName::G, 4));
    for note in g_phr{
        println!("{}", to_note_name(note));
    }

    let sr = 44100;
    let mut track = music_gen::tones::Track::new(sr, 2);
    sine_wave(&mut track, 0, sr * 4, 0.0, 1.0, 100.0);
    let volf = &hit_lin_quad(20.0,2.0,2.0);
    tone_to_track(&mut track, sr, sr, 0.0, 1.0, 400.0, &triangle_sample, &arg_id, volf, &arg_id);
    tone_to_track(&mut track, sr*2, sr, 0.0, 1.0, 400.0, &topflat_sine_sample, &arg_id, volf, &arg_id);
    tone_to_track(&mut track, sr*3, sr, 0.0, 1.0, 400.0, &square_sample, &arg_id, volf, &arg_id);
    track.normalize();
    track.render("test.wav");
}