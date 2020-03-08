use crate::theory::note::{Notes,Steps};
use crate::theory::scale::{notes_to_octave_scale,StepsTrait,ModeIteratorSpawner};
use fnrs::Sequence;
use crate::libr::scales::{get_all_scale_objs, ModeObj};

pub fn find_scale(scale: &Notes) -> Option<ModeObj>{
    let steps = Steps(notes_to_octave_scale(scale));
    let scales = get_all_scale_objs();
    for sc in scales{
        if let Some((mode,msteps)) = sc.steps.clone().mode_nr_of_this(&steps){
            return Option::Some(ModeObj{
                steps: msteps,
                fam_name: sc.family_name(),
                mode_name: sc.get_mode_name(mode as u8),
            });
        }
    }
    Option::None
}

pub fn find_scale_subseq(scale: &Notes) -> Vec<ModeObj>{
    let scales = get_all_scale_objs();
    let mut res = Vec::new();
    for sc in scales{
        for (i,mode) in sc.steps.clone().mode_iter().enumerate(){
            if mode.0.has_seq(scale){
                res.push(
                    ModeObj{
                        steps: mode,
                        fam_name: sc.family_name(),
                        mode_name: sc.get_mode_name(i as u8),
                    }
                );
            }
        }
    }
    res
}
