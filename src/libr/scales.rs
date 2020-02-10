use crate::theory::scale::{Scale};
use crate::theory::interval::*;

pub mod ionian{
    use crate::theory::scale::{Scale,Mode};
    use crate::theory::interval::*;

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

    pub fn mode_name(mode: Mode) -> String{
        match mode{
            0 => "Ionian",
            1 => "Dorian",
            2 => "Phrygian",
            3 => "Lydian",
            4 => "Mixolydian",
            5 => "Aeolian",
            6 => "Locrian",
            _ => "Error",
        }.to_string()
    }
}

pub mod miscellaneous_scales{
    use crate::theory::scale::{Scale,Mode};
    use crate::theory::interval::*;

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

    pub fn mode_name(mode: Mode) -> String{
        String::from("unnamed")
    }
}

pub mod harmonic_minor{
    use crate::theory::scale::{Scale,Mode};
    use crate::theory::interval::*;

    pub fn steps() -> Scale{
        vec![WHOLE,SEMI,WHOLE,WHOLE,SEMI,MINOR_THIRD,SEMI]
    }

    pub fn mode_name(mode: Mode) -> String{
        String::from("unnamed")
    }
}
