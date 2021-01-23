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
            String::from("unnamed")
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
    "Harmonic Minor", "Locrian ♯6", "Ionian ♯5", "Dorian ♯4", "Phrygian Dominant", "Lydian ♯2", "Superlocrian");

DefScale!(harmonic_major, vec![WHOLE,WHOLE,SEMI,WHOLE,SEMI,MINOR_THIRD,SEMI], "Harmonic Major",
            "Harmonic Major" ,"Dorian ♭5", "Super Phrygian", "Lydian Diminished", "Mixolydian ♭9", "Lydian Augmented ♯2" ,"Locrian ♭♭7");

DefScale!(byzantine, vec![SEMI,MINOR_THIRD,SEMI,WHOLE,SEMI,MINOR_THIRD,SEMI], "Byzantine",
    "Byzantine", "Lydian ♯2 ♯6", "Ultra Phrygian", "Hungarian Minor", "Oriental", "Ionian ♯2 ♯5", "Locrian ♭♭3 ♭♭7");

DefScale!(hungarian_major, vec![MINOR_THIRD,SEMI,WHOLE,SEMI,WHOLE,SEMI,WHOLE], "Hungarian Major",
    "Hungarian Major", "Super Locrian Diminished ♭6", "Harmonic Minor ♭5", "Super Locarian ♮6", "Melodic Minor ♯5", "Dorian ♭2 ♯4", "Lydian Augmented ♯3");

DefScale!(neapolitan_minor, vec![SEMI,WHOLE,WHOLE,WHOLE,SEMI,MINOR_THIRD,SEMI], "Neapolitan Minor",
    "Neapolitan Minor", "Lydian ♯6", "Mixolydian Augmented", "Lydian Minor", "Locrian ♮3", "Ionian ♯2", "Super Locrian Diminished ♭3");

DefScale!(neapolitan_major, vec![SEMI,WHOLE,WHOLE,WHOLE,WHOLE,WHOLE,SEMI],"Neapolitan Major",
    "Neapolitan Major", "Lydian Augmented ♯6", "Lydian Augmented ♭7", "Lydian Dominant ♭", "Mixolydian ♭5 ♭", "locrian ♮2 ♭4", "Super Locrian ♭♭3");

DefScale!(melodic_minor, vec![WHOLE,SEMI,WHOLE,WHOLE,WHOLE,WHOLE,SEMI], "Melodic Minor",
    "Melodic Minor", "Dorian ♭2", "Lydian Augmented", "Lydian Dominant", "Melodic Major", "Aeolian ♭5", "Altered Scale");

DefScale!(enigmatic_major, vec![SEMI,MINOR_THIRD,WHOLE,WHOLE,WHOLE,SEMI,SEMI], "Enigmatic Major",
    "Enigmatic Major", "", "Lydian ♭6 ♭♭7*", "", "", "", "");

DefScale!(enigmatic_minor, vec![SEMI,WHOLE,MINOR_THIRD,SEMI,MINOR_THIRD,SEMI,SEMI], "Enigmatic Minor",
            "Enigmatic Minor", "", "Mixolidian Augmented ♯2 ♯♯4*", "", "Ionian ♯2 ♭5 ♭6*", "", "");
