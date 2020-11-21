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

pub const DIMINISHED_SECOND: Note = 0;
pub const AUGMENTED_UNISON: Note = SEMI;
pub const DIMINISHED_THIRD: Note = WHOLE;
pub const AUGMENTED_SECOND: Note = MINOR_THIRD;
pub const DIMINISHED_FOURTH: Note = SEMI * 4;
pub const AUGMENTED_THIRD: Note = SEMI * 5;
pub const DIMINISHED_FIFTH: Note = SEMI * 6;
pub const AUGMENTED_FOURTH: Note = SEMI * 6;
pub const DIMINISHED_SIXTH: Note = SEMI * 7;
pub const AUGMENTED_FIFTH: Note = SEMI * 8;
pub const DIMINISHED_SEVENTH: Note = SEMI * 9;
pub const AUGMENTED_SIXTH: Note = SEMI * 10;
pub const DIMINISHED_OCTAVE: Note = SEMI * 11;
pub const AUGMENTED_SEVENTH: Note = SEMI * 12;

pub const S13: Note = SEMI * 13;
pub const S14: Note = SEMI * 14;
pub const S15: Note = SEMI * 15;
pub const S16: Note = SEMI * 16;
pub const S17: Note = SEMI * 17;
pub const S18: Note = SEMI * 18;
pub const S19: Note = SEMI * 19;
pub const S20: Note = SEMI * 20;
pub const S21: Note = SEMI * 21;
pub const S22: Note = SEMI * 22;

pub const SILENT: Note = -1;
pub const CARRY_ON: Note = -2;

pub fn interval_name(interval: Note) -> String{
    let string = match interval{
        PERFECT_UNISON => "Perfect Unison",
        MINOR_SECOND => "Minor Second",
        MAJOR_SECOND => "Major Second",
        MINOR_THIRD => "Minor Third",
        MAJOR_THIRD => "Major Third",
        PERFECT_FOURTH => "Perfect Fourth",
        TRITONE => "Tritone",
        PERFECT_FIFTH => "Perfect Fifth",
        MINOR_SIXTH => "Minor Sixth",
        MAJOR_SIXTH => "Major Sixth",
        MINOR_SEVENTH => "Minor Seventh",
        MAJOR_SEVENTH => "Major Seventh",
        PERFECT_OCTAVE => "Perfect Octave",
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
        PERFECT_UNISON => "P1",
        MINOR_SECOND => "m2",
        MAJOR_SECOND => "M2",
        MINOR_THIRD => "m3",
        MAJOR_THIRD => "M3",
        PERFECT_FOURTH => "P4",
        TRITONE => "TT",
        PERFECT_FIFTH => "P5",
        MINOR_SIXTH => "m6",
        MAJOR_SIXTH => "M6",
        MINOR_SEVENTH => "m7",
        MAJOR_SEVENTH  => "M7",
        PERFECT_OCTAVE => "P8",
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
        PERFECT_UNISON => "Diminished Second",
        MINOR_SECOND => "Augmented Unison",
        MAJOR_SECOND => "Diminished Third",
        MINOR_THIRD => "Augmented Second",
        MAJOR_THIRD => "Diminished Fourth",
        PERFECT_FOURTH => "Augmented Third",
        TRITONE => "Diminished Fifth/Augmented Fourth",
        PERFECT_FIFTH => "Diminished Sixth",
        MINOR_SIXTH => "Augmented Fifth",
        MAJOR_SIXTH => "Diminished Seventh",
        MINOR_SEVENTH => "Augmented Sixth",
        MAJOR_SEVENTH => "Diminished Octave",
        PERFECT_OCTAVE => "Augmented Seventh",
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
        PERFECT_UNISON => "d2",
        MINOR_SECOND => "A1",
        MAJOR_SECOND => "d3",
        MINOR_THIRD => "A2",
        MAJOR_THIRD => "d4",
        PERFECT_FOURTH => "A3",
        TRITONE => "d5/A4",
        PERFECT_FIFTH => "d6",
        MINOR_SIXTH => "A5",
        MAJOR_SIXTH => "d7",
        MINOR_SEVENTH => "A6",
        MAJOR_SEVENTH => "d8",
        PERFECT_OCTAVE => "A7",
        _ => "",
    }.to_string();
    if string.is_empty(){
        format!("S{}", interval)
    }else{
        string
    }
}

pub fn interval_chord_extension(interval: Note) -> String{
    match interval{
        0 => "R",
        MINOR_SECOND => "♭2",
        MAJOR_SECOND => "♮2",
        MINOR_THIRD => "♭3",
        MAJOR_THIRD => "♮3",
        PERFECT_FOURTH => "♮4",
        TRITONE => "♭5",
        PERFECT_FIFTH => "♮5",
        MINOR_SIXTH => "♭6",
        MAJOR_SIXTH => "♮6",
        MINOR_SEVENTH => "♭7",
        MAJOR_SEVENTH => "♮7",
        PERFECT_OCTAVE => "",
        S13 => "♭9",
        S14 => "♮9",
        S15 => "♯9",
        S16 => "♭11",
        S17 => "♮11",
        S18 => "♯11",
        S19 => "♮12",
        S20 => "♭13",
        S21 => "♮13",
        S22 => "♯13",
        _ => "",
    }.to_string()
}

pub fn to_relative_interval_non_nat(interval: Note) -> String{
    let mut res = String::new();
    let i = interval / SEMI;
    if i < 0 { for _ in 0..-i{ res.push_str("♭"); } }
    else if i > 0 { for _ in 0..i{ res.push_str("♯") } }
    res
}
