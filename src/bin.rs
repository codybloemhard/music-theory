extern crate music_theory;
use music_theory::theory::*;
// use music_theory::libr::scales::*;
// use music_theory::libr::infos::*;
// use music_theory::query::*;
// use music_theory::utils::*;
// use fnrs::Sequence;

// jazzbøt
#[cfg(not(tarpaulin))]
fn main(){
    dotest();
    // let args = lapp::parse_args("
    //     -c, --chord (default '') comma seperated vector of notes, interpreted as chord
    //     -t, --test testing output
    //     --chordstyling (default 'std') can be std, extended, spelled
    // ");
    // let chord = args.get_string("chord");
    // let test = args.get_bool("test");
    // let style = match args.get_string("chordstyling").as_ref(){
    //     "extended" => ChordStyling::Extended,
    //     "spelled" => ChordStyling::SpelledOut,
    //     _ => ChordStyling::Std,
    // };
    // if test { dotest(); }
    // if !chord.is_empty() {
    //     let res = music_theory::notes_analysis(chord, style);
    //     for (header, content) in res{
    //         println!("\t{}", header);
    //         println!("{}", content);
    //     }
    // }
}

#[cfg(not(tarpaulin))]
fn dotest(){
    let style = ChordStyle::Spelled;
    println!("{}", Scale::wrap(vec![Note::C1,Note::E1,Note::G1,Note::AS2,Note::D2,Note::F2])
             .unwrap().to_chord().quality(String::from("C"), style));
//     for named in vec![C,CS,E,F,G,GS,AS].to_scale(3).0 {
//         print!("{}, ", named.to_pc());
//     }
//     println!();
//     println!("{}", find_scale(&vec![C,CS,E,F,G,GS,AS].into_scale(0)).unwrap());
//     println!("{:?}", find_scale_superstring(&vec![A,B,C,D,E].into_scale(1)));
//     print_scales(ChordStyling::Extended);
//     let subset = vec![C,E,G,B];
//     print!("Scales which are an superset to {{");
//     for n in &subset{
//         print!("{}, ", n);
//     }
//     println!("}}");
//     for (tonic,modeobj) in find_scale_superset(subset.clone(), true){
//         println!("{} {}", tonic, modeobj);
//     }
//     println!("And the strict chordscales: ");
//     for modeobj in find_chordscales(&subset){
//         println!("{}", modeobj);
//     }
//     let res = find_scale_from_ionian_relative(Relative(vec![
//         RN_NAT, RN_S, RN_NAT, RN_S, RN_NAT, RN_S, RN_NAT,
//     ]));
//     println!("-------");
//     for modeobj in res{
//         println!("{}", modeobj);
//     }
//     println!("-------");
//     let subchords = scale_subseq_chords(ionian::obj().clone_steps().mode(0).into_scale(A4 + 3))
//         .into_iter().map(|c| (c.as_string(true, ChordStyling::Std),c))
//         .filter(|(s,_)| !s.contains('[') && !s.contains('(') && !s.is_empty())
//         .map(|(mut s,c)| { s.push_str(&format!(": {:?}", c.to_scale().into_pcs())); s })
//         .collect::<Vec<_>>();
//     print_to_grid_auto(&subchords, 80, 3);
//     println!("-------");
//     let subchords = steps_subseq_chords(harmonic_minor::obj().clone_steps().mode(4));
//     let mut chordstrings = Vec::new();
//     for (i,cell) in subchords.into_iter().enumerate(){
//         let temp = cell.into_iter().map(|c| c.quality(to_roman_num(i + 1), true, ChordStyling::Std))
//         .filter(|s| !s.contains('[') && !s.contains('(') && !s.is_empty())
//         .collect::<Vec<_>>();
//         for s in temp{
//             chordstrings.push(s);
//         }
//     }
//     print_to_grid_auto(&chordstrings, 80, 3);
//     println!("{}", vec![0, 1, 2, 3, 4].has_seq(&vec![0,3]));
}
