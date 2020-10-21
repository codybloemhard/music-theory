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

pub mod ionian{
    use crate::theory::note::{Steps};
    use crate::theory::scale::Mode;
    use crate::theory::interval::*;
    use super::ScaleObj;

    pub const IONIAN: Mode = 0;
    pub const DORIAN: Mode = 1;
    pub const PHRYGIAN: Mode = 2;
    pub const LYDIAN: Mode = 3;
    pub const MIXOLYDIAN: Mode = 4;
    pub const AEOLIAN: Mode = 5;
    pub const LOCRIAN: Mode = 6;

    pub fn steps() -> Steps{
        Steps(vec![WHOLE,WHOLE,SEMI,WHOLE,WHOLE,WHOLE,SEMI])
    }

    pub fn obj() -> ScaleObj{
        ScaleObj{
            steps: steps(),
            fam_name: String::from("Ionian"),
            modes: vec![String::from("Ionian"),
                        String::from("Dorian"),
                        String::from("Phrygian"),
                        String::from("Lydian"),
                        String::from("Mixolidian"),
                        String::from("Aeolian"),
                        String::from("Locrian")]
        }
    }
}

pub mod miscellaneous_scales{
    use crate::theory::interval::*;
    use crate::theory::note::{Steps};

    /*
    Old Greek Dorian mode.
    A 7 note scale in a octave of 2 four-note segments separated by a whole tone.
    quarter,quarter,major third,whole,quarter,quarter,major third.
    1/4 + 1/4 + 2 + 1 + 1/4 + 1/4 + 2 = 6 whole tones = 12 semitones = 1 octave
    https://en.wikipedia.org/wiki/Dorian_mode
    */
    pub fn greek_dorian_steps() -> Steps{
        Steps(vec![QUAD,QUAD,MAJOR_THIRD,WHOLE,QUAD,QUAD,MAJOR_THIRD])
    }

    pub fn greek_dorian_chromatic_steps() -> Steps{
        Steps(vec![SEMI,SEMI,MINOR_THIRD,WHOLE,SEMI,SEMI,MINOR_THIRD])
    }
    /*
    A,B,C,D#,E,F#,A
    2 + 1 + 3 + 1 + 2 + 3 = 12
    */
    pub fn satie_scale_steps() -> Steps{
        Steps(vec![WHOLE,SEMI,MINOR_THIRD,SEMI,WHOLE,MINOR_THIRD])
    }

    pub fn chromatic_scale_steps() -> Steps{
        Steps(vec![SEMI,SEMI,SEMI,SEMI,SEMI,SEMI,SEMI,SEMI,SEMI,SEMI,SEMI,SEMI])
    }
}

pub mod harmonic_minor{
    use crate::theory::note::{Steps};
    use crate::theory::interval::*;
    use super::ScaleObj;

    pub fn steps() -> Steps{
        Steps(vec![WHOLE,SEMI,WHOLE,WHOLE,SEMI,MINOR_THIRD,SEMI])
    }

    pub fn obj() -> ScaleObj{
        ScaleObj{
            steps: steps(),
            fam_name: String::from("Harmonic Minor"),
            modes: vec![String::from("Harmonic Minor"),
                        String::from("Locrian ♯6"),
                        String::from("Ionian ♯5"),
                        String::from("Dorian ♯4"),
                        String::from("Phrygian Dominant"),
                        String::from("Lydian ♯2"),
                        String::from("Superlocrian")]
        }
    }
}

pub mod harmonic_major{
    use crate::theory::note::{Steps};
    use crate::theory::interval::*;
    use super::ScaleObj;

    pub fn steps() -> Steps{
        Steps(vec![WHOLE,WHOLE,SEMI,WHOLE,SEMI,MINOR_THIRD,SEMI])
    }

    pub fn obj() -> ScaleObj{
        ScaleObj{
            steps: steps(),
            fam_name: String::from("Harmonic Major"),
            modes: vec![String::from("Harmonic Major"),
                        String::from("Dorian ♭5"),
                        String::from("Super Phrygian"),
                        String::from("Lydian Diminished"),
                        String::from("Mixolydian ♭9"),
                        String::from("Lydian Augmented ♯2"),
                        String::from("Locrian ♭♭7")]
        }
    }
}

pub mod byzantine{
    use crate::theory::note::{Steps};
    use crate::theory::interval::*;
    use super::ScaleObj;

    pub fn steps() -> Steps{
        Steps(vec![SEMI,MINOR_THIRD,SEMI,WHOLE,SEMI,MINOR_THIRD,SEMI])
    }

    pub fn obj() -> ScaleObj{
        ScaleObj{
            steps: steps(),
            fam_name: String::from("Byzantine"),
            modes: vec![String::from("Byzantine"),
                        String::from("Lydian ♯2 ♯6"),
                        String::from("Ultra Phrygian"),
                        String::from("Hungarian Minor"),
                        String::from("Oriental"),
                        String::from("Ionian ♯2 ♯5"),
                        String::from("Locrian ♭♭3 ♭♭7")]
        }
    }
}

pub mod hungarian_major{
    use crate::theory::note::{Steps};
    use crate::theory::interval::*;
    use super::ScaleObj;

    pub fn steps() -> Steps{
        Steps(vec![MINOR_THIRD,SEMI,WHOLE,SEMI,WHOLE,SEMI,WHOLE])
    }

    pub fn obj() -> ScaleObj{
        ScaleObj{
            steps: steps(),
            fam_name: String::from("Hungarian Major"),
            modes: vec![String::from("Hungarian Major"),
                        String::from("Super Locrian Diminished ♭6"),
                        String::from("Harmonic Minor ♭5"),
                        String::from("Super Locarian ♮6"),
                        String::from("Melodic Minor ♯5"),
                        String::from("Dorian ♭2 ♯4"),
                        String::from("Lydian Augmented ♯3")]
        }
    }
}

pub mod neapolitan_minor{
    use crate::theory::note::{Steps};
    use crate::theory::interval::*;
    use super::ScaleObj;

    pub fn steps() -> Steps{
        Steps(vec![SEMI,WHOLE,WHOLE,WHOLE,SEMI,MINOR_THIRD,SEMI])
    }

    pub fn obj() -> ScaleObj{
        ScaleObj{
            steps: steps(),
            fam_name: String::from("Neapolitan Minor"),
            modes: vec![String::from("Neapolitan Minor"),
                        String::from("Lydian ♯6"),
                        String::from("Mixolydian Augmented"),
                        String::from("Lydian Minor"),
                        String::from("Locrian ♮3"),
                        String::from("Ionian ♯2"),
                        String::from("Super Locrian Diminished ♭3")]
        }
    }
}

pub mod neapolitan_major{
    use crate::theory::note::{Steps};
    use crate::theory::interval::*;
    use super::ScaleObj;

    pub fn steps() -> Steps{
        Steps(vec![SEMI,WHOLE,WHOLE,WHOLE,WHOLE,WHOLE,SEMI])
    }

    pub fn obj() -> ScaleObj{
        ScaleObj{
            steps: steps(),
            fam_name: String::from("Neapolitan Major"),
            modes: vec![String::from("Neapolitan Major"),
                        String::from("Lydian Augmented ♯6"),
                        String::from("Lydian Augmented ♭7"),
                        String::from("Lydian Dominant ♭6"),
                        String::from("Mixolydian ♭5 ♭6"),
                        String::from("locrian ♮2 ♭4"),
                        String::from("Super Locrian ♭♭3")]
        }
    }
}

pub mod melodic_minor{
    use crate::theory::note::{Steps};
    use crate::theory::interval::*;
    use super::ScaleObj;

    pub fn steps() -> Steps{
        Steps(vec![WHOLE,SEMI,WHOLE,WHOLE,WHOLE,WHOLE,SEMI])
    }

    pub fn obj() -> ScaleObj{
        ScaleObj{
            steps: steps(),
            fam_name: String::from("Melodic Minor"),
            modes: vec![String::from("Melodic Minor"),
                        String::from("Dorian ♭2"),
                        String::from("Lydian Augmented"),
                        String::from("Lydian Dominant"),
                        String::from("Melodic Major"),
                        String::from("Aeolian ♭5"),
                        String::from("Altered Scale")]
        }
    }
}

pub mod enigmatic_major{
    use crate::theory::note::{Steps};
    use crate::theory::interval::*;
    use super::ScaleObj;

    pub fn steps() -> Steps{
        Steps(vec![SEMI,MINOR_THIRD,WHOLE,WHOLE,WHOLE,SEMI,SEMI])
    }

    pub fn obj() -> ScaleObj{
        ScaleObj{
            steps: steps(),
            fam_name: String::from("Enigmatic Major"),
            modes: vec![String::from("Enigmatic Major"),
                        String::from("Araboth*"),
                        String::from("Lydian ♭6 ♭♭7*"),
                        String::from("Greed*"),
                        String::from("Heresy*"),
                        String::from("Fraud*"),
                        String::from("Gluttony*")]
        }
    }
}

pub mod enigmatic_minor{
    use crate::theory::note::{Steps};
    use crate::theory::interval::*;
    use super::ScaleObj;

    pub fn steps() -> Steps{
        Steps(vec![SEMI,WHOLE,MINOR_THIRD,SEMI,MINOR_THIRD,SEMI,SEMI])
    }

    pub fn obj() -> ScaleObj{
        ScaleObj{
            steps: steps(),
            fam_name: String::from("Enigmatic Minor"),
            modes: vec![String::from("Enigmatic Minor"),
                        String::from("Ma'on*"),
                        String::from("Mixolidian Augmented ♯2 ♯♯4*"),
                        String::from("Tamaha prabha*"),
                        String::from("Ionian ♯2 ♭5 ♭6*"),
                        String::from("Dutch Scale*"),
                        String::from("Dhuma prabha*")]
        }
    }
}
