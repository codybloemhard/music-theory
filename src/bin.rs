extern crate music_gen;
use music_gen::tones::*;
use music_gen::theory::*;
 
fn main(){
    //let scale = ionian_mode(NamedNote::A(4).to_note(), AEOLIAN);
    let scale = notes_of_mode(NamedNote::A(4).to_note(), satie_scale(), 0);
    print_notes(&scale);
    let sr = 44100;
    let mut track = music_gen::tones::Track::new(sr, 2);
    let volf = &hit_lin_quot_quad(40.0,0.2, 1.0, 2);
    let mut time = 0;
    for note in scale{
        let hz = to_pitch(note);
        //tone_to_track(&mut track, time, sr * 3, 1.0, 0.0, 0.0, hz, &triangle_sample, &arg_id, volf, &arg_id);
        tone_to_track_stereo(&mut track, time, (sr as f32 * 1.5) as usize, 1.0, 0.0, hz, &spread(6,1.003,0.0, triangle_sample), volf, &arg_id);
        time += sr/2;
    }
    track.normalize(0.99);
    track.render("test.wav");
}
