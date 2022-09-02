use crate::theory::{ Steps, Chord, Scale, Note, RootedChord, PC, Interval };
use crate::theory::traits::{
    VecWrapper, ToChord, ToRootedChord, ToPCs, ModeIteratorSpawner, ToScaleTry, ToNote, AsScaleTry,
    AsRelativeIntervals, ScaleIteratorSpawner
};
use crate::libr::{ ModeObj, get_all_scale_objs, ionian };

use fnrs::Sequence;

pub fn find_scale_chords(steps: &Steps, chord_size: usize) -> Vec<Chord>{
    let len = steps.len();
    let mut chords = Vec::new();
    for (i, _) in steps.scale_iter(Note::ZERO).enumerate().take(len){
        let mut chord = Vec::new();
        for note in steps.scale_iter(Note::ZERO).skip(i).step_by(2).take(chord_size){
            chord.push(note);
        }
        chords.push(Scale(chord).to_chord());
    }
    chords
}

pub fn find_rooted_scale_chords(steps: &Steps, tonic: Note, chord_size: usize) -> Vec<RootedChord>{
    let len = steps.len();
    let mut chords = Vec::new();
    for (i, _) in steps.scale_iter(tonic).enumerate().take(len){
        let mut chord = Vec::new();
        for note in steps.scale_iter(tonic).skip(i).step_by(2).take(chord_size){
            chord.push(note);
        }
        chords.push(Scale(chord).to_rooted_chord());
    }
    chords
}

pub fn find_scale(scale: &Scale) -> Option<ModeObj>{
    let steps = scale.as_octave_steps()?;
    let scales = get_all_scale_objs();
    for sc in scales{
        if let Some((mode, msteps)) = sc.steps.clone().mode_nr_of_this(&steps){
            return Option::Some(ModeObj{
                steps: msteps,
                fam_name: sc.family_name(),
                mode_name: sc.get_mode_name(mode),
                mode_nr: mode,
            });
        }
    }
    Option::None
}

pub fn find_scale_superstring(scale: &Scale) -> Vec<(PC, ModeObj)>{
    let pcs = scale.clone().to_pcs();
    let scales = get_all_scale_objs();
    let mut res = Vec::new();
    for sc in scales{
        for (i, mode) in sc.steps.clone().mode_iter().enumerate(){
            for j in 0..12{
                let tonic = Note(j);
                // shouldn't be able to fail as it does not depend on input
                let modescale = mode.clone().to_scale_try(tonic).unwrap().to_pcs();
                if modescale.has_seq(&pcs){
                    res.push((modescale[0],
                        ModeObj{
                            steps: mode.clone(),
                            fam_name: sc.family_name(),
                            mode_name: sc.get_mode_name(i),
                            mode_nr: i,
                        })
                    );
                }
            }
        }
    }
    res
}
// Finds all the scales that are a super set of the set of notes given.
// When same_tonic == true, it only gives scales that have the same note as the
// first note in the set(ordered set shortly) as the tonic.
pub fn find_scale_superset(scale: &[PC], same_tonic: bool) -> Vec<(PC, ModeObj)>{
    let target_tonic = scale[0].to_note().0;
    let scales = get_all_scale_objs();
    let mut res = Vec::new();
    for sc in scales{
        for (i, mode) in sc.steps.clone().mode_iter().enumerate(){
            for tonic in 0..12{
                if same_tonic && tonic != target_tonic { continue; }
                let notes = mode.clone().to_scale_try(Note(tonic)).unwrap().to_pcs();
                let mut has = true;
                'outer: for a in scale{
                    for b in &notes{
                        if a == b { continue 'outer; }
                    }
                    has = false;
                }
                if has {
                    res.push(
                        (notes[0], ModeObj{
                            steps: mode.clone(),
                            fam_name: sc.family_name(),
                            mode_name: sc.get_mode_name(i),
                            mode_nr: i,
                        })
                    );
                }
            }
        }
    }
    res
}
// Finds all the scales where the input is the I chord
pub fn find_chordscales(pcs: &[PC]) -> Vec<ModeObj>{
    let mut res = Vec::new();
    if pcs.is_empty() { return res; }
    let tonic = pcs[0].to_note();
    let scales = get_all_scale_objs();
    for sc in scales{
        'outer: for (i, mode) in sc.steps.clone().mode_iter().enumerate(){
            let modescale = mode.as_scale_try(tonic).unwrap().to_pcs();
            for j in 0..pcs.len(){
                if j * 2 > modescale.len() - 1 {
                    continue 'outer;
                }
                if pcs[j] != modescale[j * 2]{
                    continue 'outer;
                }
            }
            res.push(
                ModeObj{
                    steps: mode.clone(),
                    fam_name: sc.family_name(),
                    mode_name: sc.get_mode_name(i),
                    mode_nr: i,
                }
            );
        }
    }
    res
}
// Finds all the scales with the given relative properties
pub fn find_scale_from_ionian_relative(rel: &[Interval]) -> Option<ModeObj>{
    let scales = get_all_scale_objs();
    for sc in scales{
        'outer: for (i, mode) in sc.steps.clone().mode_iter().enumerate(){
            let rl = mode.as_relative_intervals(&ionian::steps()).unwrap();
            if rel.len() != rl.len() { continue; }
            for (i, rn) in rel.iter().enumerate(){
                if rn != &rl[i] { continue 'outer; }
            }
            return Some(ModeObj{
                steps: mode,
                fam_name: sc.family_name(),
                mode_name: sc.get_mode_name(i),
                mode_nr: i,
            });
        }
    }
    None
}

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

    #[test]
    fn test_find_scale(){
        let hs = Interval(1);
        let ws = Interval(2);
        let ts = Interval(3);
        assert_eq!(
            find_scale(&Scale(vec![
                Note::C1, Note::D1, Note::E1, Note::F1, Note::G1, Note::A2, Note::B2
            ])),
            Some(ModeObj {
                steps: Steps(vec![ws, ws, hs, ws, ws, ws, hs]),
                fam_name: "Ionian".to_string(),
                mode_name: "Ionian".to_string(),
                mode_nr: 0
            })
        );
        assert_eq!(
            find_scale(&Scale(vec![
                Note::A1, Note::B1, Note::C1, Note::D1, Note::E1, Note::F1, Note::G1
            ])),
            Some(ModeObj {
                steps: Steps(vec![ws, hs, ws, ws, hs, ws, ws]),
                fam_name: "Ionian".to_string(),
                mode_name: "Aeolian".to_string(),
                mode_nr: 5
            })
        );
        assert_eq!(
            find_scale(&Scale(vec![
                Note::A1, Note::B1, Note::C1, Note::D1, Note::E1, Note::F1, Note::GS1
            ])),
            Some(ModeObj {
                steps: Steps(vec![ws, hs, ws, ws, hs, ts, hs]),
                fam_name: "Harmonic Minor".to_string(),
                mode_name: "Harmonic Minor".to_string(),
                mode_nr: 0
            })
        );
        assert_eq!(
            find_scale(&Scale(vec![
                Note::A1, Note::B1, Note::C1, Note::D1, Note::E1, Note::F1
            ])),
            None
        );
    }

    #[test]
    fn test_find_scale_superstring(){
        let scale = Scale(vec![Note::C1, Note::D1, Note::E1, Note::F1, Note::G1]);
        let mut res = find_scale_superstring(&scale)
            .into_iter().map(|(pc, mo)| (pc, mo.fam_name, mo.mode_nr));
        assert_eq!(res.next(), Some((PC::C, "Ionian".to_string(), 0)));
        assert_eq!(res.next(), Some((PC::A, "Ionian".to_string(), 2)));
        assert_eq!(res.next(), Some((PC::As, "Ionian".to_string(), 3)));
        assert_eq!(res.next(), Some((PC::C, "Ionian".to_string(), 4)));
        assert_eq!(res.next(), Some((PC::A, "Ionian".to_string(), 5)));
        assert_eq!(res.next(), Some((PC::B, "Ionian".to_string(), 6)));
        assert_eq!(res.next(), Some((PC::C, "Harmonic Major".to_string(), 0)));
        assert_eq!(res.next(), Some((PC::Gs, "Harmonic Major".to_string(), 5)));
        assert_eq!(res.next(), Some((PC::B, "Harmonic Major".to_string(), 6)));
        assert_eq!(res.next(), Some((PC::Gs, "Melodic Minor".to_string(), 2)));
        assert_eq!(res.next(), Some((PC::As, "Melodic Minor".to_string(), 3)));
        assert_eq!(res.next(), Some((PC::C, "Melodic Minor".to_string(), 4)));
        assert_eq!(res.next(), None);
    }

    #[test]
    fn test_find_scale_superset(){
        let scale = Scale(vec![Note::C1, Note::D1, Note::E1, Note::F1, Note::G1]).to_pcs();
        let res = find_scale_superset(&scale, false);
        assert_eq!(res.len(), 35);
        let mut res = find_scale_superset(&scale, true)
            .into_iter().map(|(pc, mo)| (pc, mo.fam_name, mo.mode_nr));
        assert_eq!(res.next(), Some((PC::C, "Ionian".to_string(), 0)));
        assert_eq!(res.next(), Some((PC::C, "Ionian".to_string(), 4)));
        assert_eq!(res.next(), Some((PC::C, "Harmonic Major".to_string(), 0)));
        assert_eq!(res.next(), Some((PC::C, "Melodic Minor".to_string(), 4)));
        assert_eq!(res.next(), Some((PC::C, "Enigmatic Major".to_string(), 3)));
        assert_eq!(res.next(), None);
    }

    #[test]
    fn test_find_chord_scales(){
        let mut res = find_chordscales(&[PC::F, PC::A, PC::C, PC::E])
            .into_iter().map(|mo| (mo.fam_name, mo.mode_nr));
        assert_eq!(res.next(), Some(("Ionian".to_string(), 0)));
        assert_eq!(res.next(), Some(("Ionian".to_string(), 3)));
        assert_eq!(res.next(), Some(("Harmonic Minor".to_string(), 5)));
        assert_eq!(res.next(), Some(("Harmonic Major".to_string(), 0)));
        assert_eq!(res.next(), Some(("Double Harmonic Major".to_string(), 0)));
        assert_eq!(res.next(), Some(("Double Harmonic Major".to_string(), 1)));
        assert_eq!(res.next(), Some(("Neapolitan Minor".to_string(), 1)));
        assert_eq!(res.next(), Some(("Neapolitan Minor".to_string(), 5)));
        assert_eq!(res.next(), None);
    }

    #[test]
    fn test_find_scale_from_ionian_relative(){
        let res = find_scale_from_ionian_relative(
            &[Interval::NAT, Interval::NAT, Interval::NAT, Interval::NAT,
            Interval::NAT, Interval::NAT, Interval::NAT]
        ).unwrap();
        assert_eq!(&res.fam_name, "Ionian");
        assert_eq!(res.mode_nr, 0);
        assert_eq!(
            find_scale_from_ionian_relative(&[
                Interval::NAT, Interval::FLAT, Interval::NAT, Interval::NAT,
                Interval::NAT, Interval::NAT, Interval::NAT
            ]),
            None
        );
    }
}
