extern crate music_gen;
use music_gen::theory::*;
use music_gen::libr::scales::*;
use music_gen::libr::infos::*;
use music_gen::query::*;

fn main(){
    _test2();
}

fn _test2(){
    for named in &ucns_to_named(&[C,CS,E,F,G,GS,AS], 3){
        print!("{}, ", named.to_string());
    }
    println!();
    println!("{}", find_scale(&vec![C,CS,E,F,G,GS,AS].into_scale(0)).unwrap());
    println!("\n");
    for modeobj in find_scale_superseq(&vec![A,B,C,D].into_steps()){
        println!("{}", modeobj);
    }
    print_scales(ChordStyling::Extended);
    let subset = vec![C,E,G,B];
    print!("Scales which are an superset to {{");
    for n in &subset{
        print!("{}, ", n);
    }
    println!("}}");
    for (tonic,modeobj) in find_scale_superset(subset.clone(), true){
        println!("{} {}", tonic, modeobj);
    }
    println!("And the strict chordscales: ");
    for modeobj in find_chordscales(subset.into_steps()){
        println!("{}", modeobj);
    }
    let res = find_scale_from_ionian_relative(Relative(vec![
        RN_NAT, RN_BLANK, RN_B, RN_S, RN_BLANK, RN_BLANK, RN_BLANK,
    ]));
    println!("-------");
    for modeobj in res{
        println!("{}", modeobj);
    }
    println!("-------");
    let subchords = scale_sub_chords(ionian::obj().clone_steps().into_scale(A4));
    for sc in subchords{
        let name = sc.as_string(true, ChordStyling::Std);
        if name.contains('[') { continue; }
        if name.contains('(') { continue; }
        println!("{}: {:?}", name, sc.to_scale().into_ucns());
    }
}

// fn _test1(){
//     let sr = 44100;
//     let mut track = music_gen::tones::Track::new(sr, 2);
//     let tonef = &spread(6, 1.003, 0.0, sine_sample);
//     let volf = &hit_lin_quot_quad(40.0,0.2, 1.0, 2);
//     let hzf = &arg_id;
//     let passf = &smooth_pass(10.0);
//
//     let mut score = Score::new();
//     score.new_staff();
//     score.new_bar(0, Bar::new(Key::std_key(), 120.0, TimeSig::new(1, 1.0)));
//     score.add_note(barnote(NamedNote::A(4).to_note(), 1.0), false, 0);
//     score.add_note(barnote(NamedNote::A(4).to_note(), 1.0), false, 0);
//     score.add_note(barnote(NamedNote::Cs(4).to_note(), 1.0), true, 0);
//     score.add_note(barnote(NamedNote::E(4).to_note(), 1.0), true, 0);
//
//     println!("{}", score.as_string(0));
//
//     score.render_to_track_stereo(0, &mut track, 3.0, 1.0, 0.0, tonef, volf, hzf, passf);
//     track.trim_end(0.001);
//     track.normalize(0.99);
//     track.render("test.wav");
// }
//
// fn _test0(){
//     //let scale = ionian_mode(NamedNote::A(4).to_note(), AEOLIAN);
//     let scale = miscellaneous_scales::satie_scale_steps().as_scale(NamedNote::A(3).to_note());
//     print_notes(&scale.0, "\t");
//     let sr = 44100;
//     let mut track = music_gen::tones::Track::new(sr, 2);
//     let volf = &hit_lin_quot_quad(40.0,0.2, 1.0, 2);
//     let mut time = 0;
//     for note in scale.0{//TODO: make iter for Scale
//         let hz = to_pitch(note);
//         //tone_to_track(&mut track, time, sr * 3, 1.0, 0.0, 0.0, hz, &sine_sample, &arg_id, volf, &arg_id);
//         tone_to_track_stereo(&mut track, time, sr * 3 as usize, 1.0, 0.0, hz, &spread(6, 1.003, 0.0, sine_sample), volf, &arg_id, &smooth_pass(10.0));
//         time += sr/2;
//     }
//     track.trim_end(0.001);
//     track.normalize(0.99);
//     track.render("test.wav");
// }
