extern crate music_gen;
use music_gen::tones::*;
use music_gen::theory::*;

fn main(){
    let scale = ionian_mode(NamedNote::A(4).to_note(), AEOLIAN);
    //let scale = notes_of_mode(NamedNote::A(4).to_note(), greek_dorian_enharmonic(), 0);
    print_notes(&scale);
    let sr = 44100;
    let mut track = music_gen::tones::Track::new(sr, 2);
    let volf = &hit_lin_quot(40.0,0.9);   //todo: remove popcorn on hit
    //let volf = &hit_lin_quad(40.0,2.0,2.0);
    let mut time = 0;
    for note in scale{
        let hz = to_pitch(note);
        //tone_to_track(&mut track, time, sr, 1.0, 0.0, 0.0, hz, &triangle_sample, &arg_id, volf, &arg_id);
        tone_to_track_stereo(&mut track, time, sr, 1.0, 0.0, hz, &spread(2,0.99,0.0, triangle_sample), volf, &arg_id);
        time += sr/2;
    }
    track.normalize(0.99);
    track.render("test.wav");
}
