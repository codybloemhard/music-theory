use crate::theory::scale::{Mode};
use crate::theory::note::{Steps};
use crate::theory::scale::ModeIteratorSpawner;

pub struct ScaleObj{
    pub steps: Steps,
    pub fam_name: String,
    pub modes: Vec<String>,
}

impl ScaleObj{
    pub fn clone_steps(&self) -> Steps{
        self.steps.clone()
    }

    pub fn family_name(&self) -> String{
        self.fam_name.clone()
    }

    pub fn get_mode_name(&self, mode: Mode) -> String{
        let m = mode as usize % self.steps.0.len();
        let name = self.modes[m].clone();
        if name.is_empty(){
            String::from("")
        }else{
            name
        }
    }

    pub fn get_modes(self) -> Vec<ModeObj>{
        let fname = self.family_name();
        let mut res = Vec::new();
        for (i,mode) in self.clone_steps().mode_iter().enumerate(){
            res.push(
                ModeObj{
                    steps: mode,
                    fam_name: fname.clone(),
                    mode_name: self.get_mode_name(i as u8),
                    mode_nr: i,
                }
            );
        }
        res
    }
}

pub fn get_all_scale_objs() -> Vec<ScaleObj>{
    vec![ionian::obj(),
    harmonic_minor::obj(), harmonic_major::obj(),
    melodic_minor::obj(),
    byzantine::obj(), hungarian_major::obj(),
    neapolitan_minor::obj(), neapolitan_major::obj(),
    enigmatic_major::obj(), enigmatic_minor::obj()]
}

pub struct ModeObj{
    pub steps: Steps,
    pub fam_name: String,
    pub mode_name: String,
    pub mode_nr: usize,
}

impl std::fmt::Display for ModeObj{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}, {}ᵉ mode of {}", self.mode_name, self.mode_nr + 1, self.fam_name)
    }
}

macro_rules! DefScale{
    ($modname:ident, $steps:expr, $name:expr, $( $mode:expr ),* ) => {
        pub mod $modname{
            use crate::theory::note::{Steps};
            use crate::theory::interval::*;
            use super::ScaleObj;

            pub fn steps() -> Steps{
                Steps($steps)
            }

            pub fn obj() -> ScaleObj{
                let mut modes = Vec::new();
                $(
                    modes.push(String::from($mode));
                )*
                ScaleObj{
                    steps: steps(),
                    fam_name: String::from($name),
                    modes,
                }
            }
        }
    }
}

DefScale!(ionian, vec![WHOLE,WHOLE,SEMI,WHOLE,WHOLE,WHOLE,SEMI], "Ionian",
    "Ionian", "Dorian", "Phrygian", "Lydian", "Mixolidian", "Aeolian", "Locrian");

DefScale!(harmonic_minor, vec![WHOLE,SEMI,WHOLE,WHOLE,SEMI,MINOR_THIRD,SEMI], "Harmonic Minor",
    "Harmonic Minor", "", "", "", "Phrygian Dominant", "", "Superlocrian");

DefScale!(harmonic_major, vec![WHOLE,WHOLE,SEMI,WHOLE,SEMI,MINOR_THIRD,SEMI], "Harmonic Major",
    "Harmonic Major" ,"", "Super Phrygian", "Lydian Diminished", "", "" ,"");

DefScale!(byzantine, vec![SEMI,MINOR_THIRD,SEMI,WHOLE,SEMI,MINOR_THIRD,SEMI], "Double Harmonic Major",
    "Byzantine", "", "Ultra Phrygian", "Hungarian Minor", "Oriental", "", "");

DefScale!(hungarian_major, vec![MINOR_THIRD,SEMI,WHOLE,SEMI,WHOLE,SEMI,WHOLE], "Hungarian Major",
    "Hungarian Major", "", "", "", "", "", "");

DefScale!(neapolitan_minor, vec![SEMI,WHOLE,WHOLE,WHOLE,SEMI,MINOR_THIRD,SEMI], "Neapolitan Minor",
    "Neapolitan Minor", "", "Mixolydian Augmented", "Lydian Minor", "", "", "");

DefScale!(neapolitan_major, vec![SEMI,WHOLE,WHOLE,WHOLE,WHOLE,WHOLE,SEMI],"Neapolitan Major",
    "Neapolitan Major", "", "", "", "", "", "");

DefScale!(melodic_minor, vec![WHOLE,SEMI,WHOLE,WHOLE,WHOLE,WHOLE,SEMI], "Melodic Minor",
    "Melodic Minor", "", "Lydian Augmented", "Lydian Dominant", "Melodic Major", "", "Altered Scale");

DefScale!(enigmatic_major, vec![SEMI,MINOR_THIRD,WHOLE,WHOLE,WHOLE,SEMI,SEMI], "Enigmatic Major",
    "Enigmatic Major", "", "", "", "", "", "");

DefScale!(enigmatic_minor, vec![SEMI,WHOLE,MINOR_THIRD,SEMI,MINOR_THIRD,SEMI,SEMI], "Enigmatic Minor",
    "Enigmatic Minor", "", "", "", "", "", "");

use crate::Scale;
use crate::theory::note::ToScale;
use crate::theory::note::NoteSequence;
use crate::theory::note::IntoScale;
use crate::to_relative_interval_non_nat;

pub struct HeptatonicScaleNamer{
    basis: Vec<(Scale, String)>,
}

impl HeptatonicScaleNamer{
    pub fn new() -> Self{
        let scales = get_all_scale_objs();
        let mut basis = Vec::new();
        for sc in scales{
            for (i,mode) in sc.steps.clone().mode_iter().enumerate(){
                let mode_name = sc.get_mode_name(i as u8);
                if mode_name.is_empty() {
                    continue;
                }
                basis.push((mode.to_scale(0), mode_name));
            }
        }
        Self{
            basis,
        }
    }

    pub fn name(&self, steps: &Steps) -> String{
        let nameless = steps.to_scale(0);
        if nameless.len() != 7 { return String::new(); }
        let mut dif_positions = 8;
        let mut dif_units = 9999;
        let ionian = ionian::obj().steps.into_scale(0);
        let mut base_scale = ionian.clone();
        let mut base_name = "Ionian".to_string();
        for (scale, name) in &self.basis{
            let mut dp = 0;
            let mut du = 0;
            for i in 0..7{
                let d = nameless.0[i] - scale.0[i];
                if d == 0 { continue; }
                dp += 1;
                du += d.abs();
            }
            if dp > dif_positions { continue; }
            if du >= dif_units { continue; }
            dif_positions = dp;
            dif_units = du;
            base_scale = scale.clone();
            base_name = name.to_string();
        }
        for i in 0..7{
            let d = nameless.0[i] - base_scale.0[i];
            if d == 0 { continue; }
            let d = nameless.0[i] - ionian.0[i];
            base_name.push_str(&format!(" {}{}", to_relative_interval_non_nat(d), i + 1));
        }
        base_name
    }
}
