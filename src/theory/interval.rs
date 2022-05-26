use super::note::{ Note, ToNote };
use std::cmp::Ordering;

pub const SEMI: Note = 1;
pub const WHOLE: Note = 2;

pub const ROOT: Note = 0;
pub const UNISON: Note = 0;
pub const MINOR_SECOND: Note = 1;
pub const MAJOR_SECOND: Note = 2;
pub const MINOR_THIRD: Note = 3;
pub const MAJOR_THIRD: Note = 4;
pub const PERFECT_FOURTH: Note = 5;
pub const TRITONE: Note = 6;
pub const PERFECT_FIFTH: Note = 7;
pub const MINOR_SIXTH: Note =  8;
pub const MAJOR_SIXTH: Note = 9;
pub const MINOR_SEVENTH: Note = 10;
pub const MAJOR_SEVENTH: Note = 11;
pub const OCTAVE: Note = 12;

pub const FLAT_NINETH: Note = 13;
pub const NINETH: Note = 14;
pub const SHARP_NINETH: Note = 15;
pub const FLAT_ELEVENTH: Note = 16;
pub const ELEVENTH: Note = 17;
pub const SHARP_ELEVENTH: Note = 18;
pub const TWELVETH: Note = 19;
pub const FLAT_THIRTEENTH: Note = 20;
pub const THIRTEENTH: Note = 21;
pub const SHARP_THIRTEENTH: Note = 22;

pub const MIN2: Note = 1;
pub const MAJ2: Note = 2;
pub const MIN3: Note = 3;
pub const MAJ3: Note = 4;
pub const PER4: Note = 5;
pub const TRIT: Note = 6;
pub const PER5: Note = 7;
pub const MIN6: Note = 8;
pub const MAJ6: Note = 9;
pub const MIN7: Note = 10;
pub const MAJ7: Note = 11;

pub const DIMINISHED_SECOND: Note = 0;
pub const AUGMENTED_UNISON: Note = 1;
pub const DIMINISHED_THIRD: Note = 2;
pub const AUGMENTED_SECOND: Note = 3;
pub const DIMINISHED_FOURTH: Note = 4;
pub const AUGMENTED_THIRD: Note = 5;
pub const DIMINISHED_FIFTH: Note = 6;
pub const AUGMENTED_FOURTH: Note = 6;
pub const DIMINISHED_SIXTH: Note = 7;
pub const AUGMENTED_FIFTH: Note = 8;
pub const DIMINISHED_SEVENTH: Note = 9;
pub const AUGMENTED_SIXTH: Note = 10;
pub const DIMINISHED_OCTAVE: Note = 11;
pub const AUGMENTED_SEVENTH: Note = 12;

#[derive(Clone, Copy)]
pub enum Interval{
    Root = 0,
    Min2 = 1,
    Maj2 = 2,
    Min3 = 3,
    Maj3 = 4,
    Per4 = 5,
    Trit = 6,
    Per5 = 7,
    Min6 = 8,
    Maj6 = 9,
    Min7 = 10,
    Maj7 = 11,
    Per8 = 12,
    Min9 = 13,
    Maj9 = 14,
    Aug9 = 15,
    Min11 = 16,
    Maj11 = 17,
    Aug11 = 18,
    Min13 = 20,
    Maj13 = 21,
    Aug13 = 22,
}

#[derive(Clone, Copy)]
pub enum OctaveInterval{
    Root = 0,
    Min2 = 1,
    Maj2 = 2,
    Min3 = 3,
    Maj3 = 4,
    Per4 = 5,
    Trit = 6,
    Per5 = 7,
    Min6 = 8,
    Maj6 = 9,
    Min7 = 10,
    Maj7 = 11,
}

pub trait ToInterval{
    fn to_interval_try(self) -> Option<Interval>;
    fn to_interval_mod(self) -> Interval;
}

impl ToInterval for Note{
    fn to_interval_try(self) -> Option<Interval>{
        match self{
            0 => Some(Interval::Root),
            1 => Some(Interval::Min2),
            2 => Some(Interval::Maj2),
            3 => Some(Interval::Min3),
            4 => Some(Interval::Maj3),
            5 => Some(Interval::Per4),
            6 => Some(Interval::Trit),
            7 => Some(Interval::Per5),
            8 => Some(Interval::Min6),
            9 => Some(Interval::Maj6),
            10 => Some(Interval::Min7),
            11 => Some(Interval::Maj7),
            12 => Some(Interval::Per8),
            13 => Some(Interval::Min9),
            14 => Some(Interval::Maj9),
            15 => Some(Interval::Aug9),
            16 => Some(Interval::Min11),
            17 => Some(Interval::Maj11),
            18 => Some(Interval::Aug11),
            20 => Some(Interval::Min13),
            21 => Some(Interval::Maj13),
            22 => Some(Interval::Aug13),
            _ => None,
        }
    }

    fn to_interval_mod(self) -> Interval{
        let int = (self % 24).to_interval_try();
        match int{
            Some(i) => i,
            None => (self % 12).to_interval_try().unwrap()
        }
    }
}

impl ToInterval for OctaveInterval{
    fn to_interval_try(self) -> Option<Interval>{
        (self as Note).to_interval_try()
    }

    fn to_interval_mod(self) -> Interval{
        (self as Note).to_interval_mod()
    }
}

impl ToOctaveInterval for Note{
    fn to_octave_interval_try(self) -> Option<OctaveInterval>{
        match self{
            0 => Some(OctaveInterval::Root),
            1 => Some(OctaveInterval::Min2),
            2 => Some(OctaveInterval::Maj2),
            3 => Some(OctaveInterval::Min3),
            4 => Some(OctaveInterval::Maj3),
            5 => Some(OctaveInterval::Per4),
            6 => Some(OctaveInterval::Trit),
            7 => Some(OctaveInterval::Per5),
            8 => Some(OctaveInterval::Min6),
            9 => Some(OctaveInterval::Maj6),
            10 => Some(OctaveInterval::Min7),
            11 => Some(OctaveInterval::Maj7),
            _ => None,
        }
    }

    fn to_octave_interval_mod(self) -> OctaveInterval{
        (self % 12).to_octave_interval_try().unwrap()
    }
}

impl ToOctaveInterval for Interval{
    fn to_octave_interval_try(self) -> Option<OctaveInterval>{
        (self as Note).to_octave_interval_try()
    }

    fn to_octave_interval_mod(self) -> OctaveInterval{
        (self as Note).to_octave_interval_mod()
    }
}

pub trait ToOctaveInterval{
    fn to_octave_interval_try(self) -> Option<OctaveInterval>;
    fn to_octave_interval_mod(self) -> OctaveInterval;
}

pub trait ToIntervalChordExtension{
    fn to_interval_chord_extension(self) -> String;
}

impl ToIntervalChordExtension for Interval{
    fn to_interval_chord_extension(self) -> String{
        let names = [
            "R", "♭2", "♮2", "♭3", "♮3", "♮4", "♭5", "♮5", "♭6", "♮6", "♭7", "♮7", "♮8",
            "♭9", "♮9", "♯9", "♭11", "♮11", "♯11", "", "♭13", "♮13", "♯13",
        ];
        names[self as usize].to_string()
    }
}

impl ToIntervalChordExtension for Option<Interval>{
    fn to_interval_chord_extension(self) -> String{
        match self{
            None => "x".to_string(),
            Some(int) => int.to_interval_chord_extension(),
        }
    }
}

pub fn to_relative_interval_non_nat(interval: Note) -> String{
    let mut res = String::new();
    let i = interval;
    match i.cmp(&0){
        Ordering::Less => { for _ in 0..-i{ res.push('♭'); } },
        Ordering::Greater => { for _ in 0..i{ res.push('♯'); } },
        Ordering::Equal => { res.push('♮'); },
    }
    res
}

pub fn to_degree(interval: Note) -> String{
    match interval{
        0 => "I",
        1 => "bII",
        2 => "II",
        3 => "bIII",
        4 => "III",
        5 => "IV",
        6 => "bV",
        7 => "V",
        8 => "bVI",
        9 => "VI",
        10 => "bVII",
        11 => "VII",
        _ => "[outofrange]",
    }.to_string()
}
