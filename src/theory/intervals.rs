use super::note::Note;

pub const QUAD: Note = SEMI / 2;
pub const SEMI: Note = 120;
pub const WHOLE: Note = SEMI * 2;

pub const PERFECT_UNISON: Note = 0;
pub const MINOR_SECOND: Note = SEMI;
pub const MAJOR_SECOND: Note = WHOLE;
pub const MINOR_THIRD: Note = SEMI * 3;
pub const MAJOR_THIRD: Note = SEMI * 4;
pub const PERFECT_FOURTH: Note = SEMI * 5;
pub const TRITONE: Note = SEMI * 6;
pub const PERFECT_FIFTH: Note = SEMI * 7;
pub const MINOR_SIXTH: Note =  SEMI * 8;
pub const MAJOR_SIXTH: Note = SEMI * 9;
pub const MINOR_SEVENTH: Note = SEMI * 10;
pub const MAJOR_SEVENTH: Note = SEMI * 11;
pub const PERFECT_OCTAVE: Note = SEMI * 12;

pub const SILENT: Note = -1;
pub const CARRY_ON: Note = -2;

pub fn interval_name(interval: Note) -> String{
    let string = match interval{
        0 => "Perfect Unison",
        1 => "Minor Second",
        2 => "Major Second",
        3 => "Minor Third",
        4 => "Major Third",
        5 => "Perfect Fourth",
        6 => "Tritone",
        7 => "Perfect Fifth",
        8 => "Minor Sixth",
        9 => "Major Sixth",
        10 => "Minor Seventh",
        11 => "Major Seventh",
        12 => "Perfect Octave",
        _ => "",
    }.to_string();
    if string.is_empty(){
        format!("{} Semitones", interval)
    }else{
        string
    }
}

pub fn interval_name_short(interval: Note) -> String{
    let string = match interval{
        0 => "P1",
        1 => "m2",
        2 => "M2",
        3 => "m3",
        4 => "M3",
        5 => "P4",
        6 => "TT",
        7 => "P5",
        8 => "m6",
        9 => "M6",
        10 => "m7",
        11 => "M7",
        12 => "P8",
        _ => "",
    }.to_string();
    if string.is_empty(){
        format!("S{}", interval)
    }else{
        string
    }
}

pub fn interval_name_augdim(interval: Note) -> String{
    let string = match interval{
        0 => "Diminished Second",
        1 => "Augmented Unison",
        2 => "Diminished Third",
        3 => "Augmented Second",
        4 => "Diminished Fourth",
        5 => "Augmented Third",
        6 => "Diminished Fifth/Augmented Fourth",
        7 => "Diminished Sixth",
        8 => "Augmented Fifth",
        9 => "Diminished Seventh",
        10 => "Augmented Sixth",
        11 => "Diminished Octave",
        12 => "Augmented Seventh",
        _ => "",
    }.to_string();
    if string.is_empty(){
        format!("{} Semitones", interval)
    }else{
        string
    }
}

pub fn interval_name_augdim_short(interval: Note) -> String{
    let string = match interval{
        0 => "d2",
        1 => "A1",
        2 => "d3",
        3 => "A2",
        4 => "d4",
        5 => "A3",
        6 => "d5/A4",
        7 => "d6",
        8 => "A5",
        9 => "d7",
        10 => "A6",
        11 => "d8",
        12 => "A7",
        _ => "",
    }.to_string();
    if string.is_empty(){
        format!("S{}", interval)
    }else{
        string
    }
}
