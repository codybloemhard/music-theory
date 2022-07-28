// use crate::theory::note::{ Steps, Scale, Relative, IntoPCs, PCs, PC, NoteSequence, ToScale, IntoScale, ToRelative, ToNote };
// use crate::theory::scale::{ notes_to_octave_scale, StepsTrait, ModeIteratorSpawner };
// use crate::theory::interval::{ _SEMI };
// use crate::libr::scales::{get_all_scale_objs, ModeObj,ionian};

use crate::theory::{ Steps, Chord, scale_iter, Scale, Note, RootedChord };
use crate::theory::traits::{ VecWrapper, ToChord, ToRootedChord };

// use fnrs::Sequence;

pub fn find_scale_chords(steps: &Steps, chord_size: usize) -> Vec<Chord>{
    let len = steps.len();
    let mut chords = Vec::new();
    for (i, _) in scale_iter(Note::ZERO, &steps.0).enumerate().take(len){
        let mut chord = Vec::new();
        for note in scale_iter(Note::ZERO, &steps.0).skip(i).step_by(2).take(chord_size){
            chord.push(note);
        }
        chords.push(Scale(chord).to_chord());
    }
    chords
}

pub fn find_rooted_scale_chords(steps: &Steps, tonic: Note, chord_size: usize) -> Vec<RootedChord>{
    let len = steps.len();
    let mut chords = Vec::new();
    for (i, _) in scale_iter(tonic, &steps.0).enumerate().take(len){
        let mut chord = Vec::new();
        for note in scale_iter(tonic, &steps.0).skip(i).step_by(2).take(chord_size){
            chord.push(note);
        }
        chords.push(Scale(chord).to_rooted_chord());
    }
    chords
}


// pub fn find_scale(scale: &Scale) -> Option<ModeObj>{
//     let steps = Steps(notes_to_octave_scale(scale));
//     let scales = get_all_scale_objs();
//     for sc in scales{
//         if let Some((mode,msteps)) = sc.steps.clone().mode_nr_of_this(&steps){
//             return Option::Some(ModeObj{
//                 steps: msteps,
//                 fam_name: sc.family_name(),
//                 mode_name: sc.get_mode_name(mode as u8),
//                 mode_nr: mode,
//             });
//         }
//     }
//     Option::None
// }
//
// pub fn find_scale_superstring(scale: &Scale) -> Vec<(PC,ModeObj)>{
//     let pcs = scale.clone().into_pcs();
//     let scales = get_all_scale_objs();
//     let mut res = Vec::new();
//     for sc in scales{
//         for (i,mode) in sc.steps.clone().mode_iter().enumerate(){
//             for j in 0..12{
//                 let tonic = j * _SEMI;
//                 let modescale = mode.clone().into_scale(tonic).into_pcs();
//                 if modescale.has_seq(&pcs){
//                     res.push((modescale[0],
//                         ModeObj{
//                             steps: mode.clone(),
//                             fam_name: sc.family_name(),
//                             mode_name: sc.get_mode_name(i as u8),
//                             mode_nr: i,
//                         })
//                     );
//                 }
//             }
//         }
//     }
//     res
// }
//
// // Finds all the scales that are a super set of the set of notes given.
// // When same_tonic == true, it only gives scales that have the same note as the
// // first note in the set(ordered set shortly) as the tonic.
// pub fn find_scale_superset(scale: PCs, same_tonic: bool) -> Vec<(PC,ModeObj)>{
//     let target_tonic = scale[0].to_note();
//     let scales = get_all_scale_objs();
//     let mut res = Vec::new();
//     for sc in scales{
//         for (i,mode) in sc.steps.clone().mode_iter().enumerate(){
//             for tonic in 0..12{
//                 let tonic_note = tonic * _SEMI;
//                 if same_tonic && tonic_note != target_tonic { continue; }
//                 let notes = mode.clone().into_scale(tonic_note).into_pcs();
//                 let mut has = true;
//                 'outer: for a in &scale{
//                     for b in &notes{
//                         if a == b { continue 'outer; }
//                     }
//                     has = false;
//                 }
//                 if has {
//                     res.push(
//                         (notes[0],ModeObj{
//                             steps: mode.clone(),
//                             fam_name: sc.family_name(),
//                             mode_name: sc.get_mode_name(i as u8),
//                             mode_nr: i,
//                         })
//                     );
//                 }
//             }
//         }
//     }
//     res
// }
// // Finds all the scales where the input is the I chord
// pub fn find_chordscales(pcs: &[PC]) -> Vec<ModeObj>{
//     let mut res = Vec::new();
//     if pcs.is_empty() { return res; }
//     let tonic = pcs[0].to_note();
//     let scales = get_all_scale_objs();
//     for sc in scales{
//         'outer: for (i,mode) in sc.steps.clone().mode_iter().enumerate(){
//             let modescale = mode.to_scale(tonic).into_pcs();
//             for j in 0..pcs.len(){
//                 if j * 2 > modescale.len() - 1 {
//                     continue 'outer;
//                 }
//                 if pcs[j] != modescale[j * 2]{
//                     continue 'outer;
//                 }
//             }
//             res.push(
//                 ModeObj{
//                     steps: mode.clone(),
//                     fam_name: sc.family_name(),
//                     mode_name: sc.get_mode_name(i as u8),
//                     mode_nr: i,
//                 }
//             );
//         }
//     }
//     res
// }
// // Finds all the scales with the given relative properties
// pub fn find_scale_from_ionian_relative(rel: Relative) -> Vec<ModeObj>{
//     let scales = get_all_scale_objs();
//     let mut res = Vec::new();
//     for sc in scales{
//         'outer: for (i,mode) in sc.steps.clone().mode_iter().enumerate(){
//             let rl = mode.to_relative(&ionian::steps()).unwrap();
//             if rel.len() != rl.len() { continue; }
//             for (i, rn) in rel.0.iter().enumerate(){
//                 if rn != &rl.0[i] { continue 'outer; }
//             }
//             res.push(
//                 ModeObj{
//                     steps: mode.clone(),
//                     fam_name: sc.family_name(),
//                     mode_name: sc.get_mode_name(i as u8),
//                     mode_nr: i,
//                 }
//             );
//         }
//     }
//     res
// }

#[cfg(test)]
mod tests{
    use super::*;
    use crate::theory::*;

    #[test]
    fn test_find_scale_chords(){
        let chords = find_scale_chords(&crate::libr::ionian::steps(), 3);
        assert_eq!(
            chords,
            vec![
                Chord::new(MAJOR),
                Chord::new(MINOR),
                Chord::new(MINOR),
                Chord::new(MAJOR),
                Chord::new(MAJOR),
                Chord::new(MINOR),
                Chord::new(MINOR_DIMINISHED)
            ]
        );
    }

    #[test]
    fn test_find_rooted_scale_chords(){
        let chords = find_rooted_scale_chords(&crate::libr::ionian::steps(), Note::C1, 3);
        assert_eq!(
            chords,
            vec![
                RootedChord{ root: Note::C1, chord: Chord::new(MAJOR) },
                RootedChord{ root: Note::D1, chord: Chord::new(MINOR) },
                RootedChord{ root: Note::E1, chord: Chord::new(MINOR) },
                RootedChord{ root: Note::F1, chord: Chord::new(MAJOR) },
                RootedChord{ root: Note::G1, chord: Chord::new(MAJOR) },
                RootedChord{ root: Note::A2, chord: Chord::new(MINOR) },
                RootedChord{ root: Note::B2, chord: Chord::new(MINOR_DIMINISHED) }
            ]
        );
    }
}
