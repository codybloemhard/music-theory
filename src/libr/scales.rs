use crate::theory::scale::{Scale,Mode};
use crate::theory::interval::*;

pub struct ScaleObj{
    pub steps: Scale,
    pub fam_name: String,
    pub modes: Vec<String>,
}

impl ScaleObj{
    pub fn clone_steps(&self) -> Scale{
        self.steps.clone()
    }

    pub fn family_name(&self) -> String{
        self.fam_name.clone()
    }

    pub fn get_mode_name(&self, mode: Mode) -> String{
        let m = mode as usize % self.steps.len();
        let name = self.modes[m].clone();
        if name.is_empty(){
            String::from("unnamed")
        }else{
            name
        }
    }
}

pub fn get_all_scale_objs() -> Vec<ScaleObj>{
    vec![ionian::obj(), harmonic_minor::obj(), harmonic_major::obj()]
}

pub mod ionian{
    use crate::theory::scale::{Scale,Mode};
    use crate::theory::interval::*;
    use super::ScaleObj;

    pub const IONIAN: Mode = 0;
    pub const DORIAN: Mode = 1;
    pub const PHRYGIAN: Mode = 2;
    pub const LYDIAN: Mode = 3;
    pub const MIXOLYDIAN: Mode = 4;
    pub const AEOLIAN: Mode = 5;
    pub const LOCRIAN: Mode = 6;

    pub fn steps() -> Scale{
        vec![WHOLE,WHOLE,SEMI,WHOLE,WHOLE,WHOLE,SEMI]
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
    use crate::theory::scale::{Scale};

    /*
    Old Greek Dorian mode.
    A 7 note scale in a octave of 2 four-note segments separated by a whole tone.
    quarter,quarter,major third,whole,quarter,quarter,major third.
    1/4 + 1/4 + 2 + 1 + 1/4 + 1/4 + 2 = 6 whole tones = 12 semitones = 1 octave
    https://en.wikipedia.org/wiki/Dorian_mode
    */
    pub fn greek_dorian_steps() -> Scale{
        vec![QUAD,QUAD,MAJOR_THIRD,WHOLE,QUAD,QUAD,MAJOR_THIRD]
    }

    pub fn greek_dorian_chromatic_steps() -> Scale{
        vec![SEMI,SEMI,MINOR_THIRD,WHOLE,SEMI,SEMI,MINOR_THIRD]
    }
    /*
    A,B,C,D#,E,F#,A
    2 + 1 + 3 + 1 + 2 + 3 = 12
    */
    pub fn satie_scale_steps() -> Scale{
        vec![WHOLE,SEMI,MINOR_THIRD,SEMI,WHOLE,MINOR_THIRD]
    }

    pub fn chromatic_scale_steps() -> Scale{
        vec![SEMI,SEMI,SEMI,SEMI,SEMI,SEMI,SEMI,SEMI,SEMI,SEMI,SEMI,SEMI]
    }
}

pub mod harmonic_minor{
    use crate::theory::scale::{Scale};
    use crate::theory::interval::*;
    use super::ScaleObj;

    pub fn steps() -> Scale{
        vec![WHOLE,SEMI,WHOLE,WHOLE,SEMI,MINOR_THIRD,SEMI]
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
    use crate::theory::scale::{Scale};
    use crate::theory::interval::*;
    use super::ScaleObj;

    pub fn steps() -> Scale{
        vec![WHOLE,WHOLE,SEMI,WHOLE,SEMI,MINOR_THIRD,SEMI]
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
                        String::from("Lydian Augmented #2"),
                        String::from("Locrian ♭♭7")]
        }
    }
}
