use crate::theory::note::{Steps,Scale,Relative,RelativeNote,IntoPCs,PCs,PC,NoteSequence,ToScale,IntoScale,ToRelative,ToSteps};
use crate::theory::scale::{notes_to_octave_scale,StepsTrait,ModeIteratorSpawner};
use crate::theory::interval::{SEMI};
use fnrs::Sequence;
use crate::libr::scales::{get_all_scale_objs, ModeObj,ionian};

pub fn find_scale(scale: &Scale) -> Option<ModeObj>{
    let steps = Steps(notes_to_octave_scale(scale));
    let scales = get_all_scale_objs();
    for sc in scales{
        if let Some((mode,msteps)) = sc.steps.clone().mode_nr_of_this(&steps){
            return Option::Some(ModeObj{
                steps: msteps,
                fam_name: sc.family_name(),
                mode_name: sc.get_mode_name(mode as u8),
                mode_nr: mode,
            });
        }
    }
    Option::None
}

pub fn find_steps_superseq(scale: &Steps) -> Vec<ModeObj>{
    let scales = get_all_scale_objs();
    let mut res = Vec::new();
    for sc in scales{
        for (i,mode) in sc.steps.clone().mode_iter().enumerate(){
            if mode.0.has_seq(&scale.0){
                res.push(
                    ModeObj{
                        steps: mode,
                        fam_name: sc.family_name(),
                        mode_name: sc.get_mode_name(i as u8),
                        mode_nr: i,
                    }
                );
            }
        }
    }
    res
}

pub fn find_scale_superseq(scale: &Scale) -> Vec<(PC,ModeObj)>{
    let steps = scale.to_steps();
    let pcs = scale.clone().into_pcs();
    let scales = get_all_scale_objs();
    let mut res = Vec::new();
    for sc in scales{
        for (i,mode) in sc.steps.clone().mode_iter().enumerate(){
            if !mode.0.has_seq(&steps.0) { continue; }
            for j in 0..12{
                let tonic = j * SEMI;
                let modescale = mode.clone().into_scale(tonic).into_pcs();
                if modescale.has_seq(&pcs){
                    res.push((modescale[0],
                        ModeObj{
                            steps: mode.clone(),
                            fam_name: sc.family_name(),
                            mode_name: sc.get_mode_name(i as u8),
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
pub fn find_scale_superset(scale: PCs, same_tonic: bool) -> Vec<(PC,ModeObj)>{
    let target_tonic = scale[0].to_note(0);
    let scales = get_all_scale_objs();
    let mut res = Vec::new();
    for sc in scales{
        for (i,mode) in sc.steps.clone().mode_iter().enumerate(){
            for tonic in 0..12{
                let tonic_note = tonic * SEMI;
                if same_tonic && tonic_note != target_tonic { continue; }
                let notes = mode.clone().into_scale(tonic_note).into_pcs();
                let mut has = true;
                'outer: for a in &scale{
                    for b in &notes{
                        if a == b { continue 'outer; }
                    }
                    has = false;
                }
                if has {
                    res.push(
                        (notes[0],ModeObj{
                            steps: mode.clone(),
                            fam_name: sc.family_name(),
                            mode_name: sc.get_mode_name(i as u8),
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
pub fn find_chordscales(scale: Steps) -> Vec<ModeObj>{
    let scale = scale.into_scale(0);
    let scales = get_all_scale_objs();
    let mut res = Vec::new();
    for sc in scales{
        'outer: for (i,mode) in sc.steps.clone().mode_iter().enumerate(){
            let modescale = mode.to_scale(0);
            let l = scale.len().min(modescale.len() / 2);
            for j in 0..l{
                if scale.0[j] != modescale.0[j * 2]{
                    continue 'outer;
                }
            }
            res.push(
                ModeObj{
                    steps: mode.clone(),
                    fam_name: sc.family_name(),
                    mode_name: sc.get_mode_name(i as u8),
                    mode_nr: i,
                }
            );
        }
    }
    res
}
// Finds all the scales with the given relative properties
pub fn find_scale_from_ionian_relative(rel: Relative) -> Vec<ModeObj>{
    let scales = get_all_scale_objs();
    let mut res = Vec::new();
    for sc in scales{
        'outer: for (i,mode) in sc.steps.clone().mode_iter().enumerate(){
            let rl = mode.to_relative(&ionian::steps()).unwrap();
            if rel.len() != rl.len() { continue; }
            for (i,rn) in rel.0.iter().enumerate(){
                match rn{
                    RelativeNote::Blank => { continue; },
                    _ => { if rn != &rl.0[i] { continue 'outer; } }
                }
            }
            res.push(
                ModeObj{
                    steps: mode.clone(),
                    fam_name: sc.family_name(),
                    mode_name: sc.get_mode_name(i as u8),
                    mode_nr: i,
                }
            );
        }
    }
    res
}
