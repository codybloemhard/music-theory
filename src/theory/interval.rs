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

pub fn to_chord_interval(interval: Note) -> String{
    let s13 = SEMI * 13; let s14 = SEMI * 14;
    let s15 = SEMI * 15; let s16 = SEMI * 16;
    let s17 = SEMI * 17; let s18 = SEMI * 18;
    let s19 = SEMI * 19; let s20 = SEMI * 20;
    let s21 = SEMI * 21; let s22 = SEMI * 22;
    match interval{
        0 => "R",
        MINOR_SECOND => "♭2",
        MAJOR_SECOND => "♮2",
        MINOR_THIRD => "♭3",
        MAJOR_THIRD => "♮3",
        PERFECT_FOURTH => "♮4",
        TRITONE => "o",
        PERFECT_FIFTH => "♮5",
        MINOR_SIXTH => "+",
        MAJOR_SIXTH => "♮6",
        MINOR_SEVENTH => "♭7",
        MAJOR_SEVENTH => "♮7",
        PERFECT_OCTAVE => "♮8",
        s13 => "♭9",
        s14 => "♮9",
        s15 => "♯9",
        s16 => "♭11",
        s17 => "♮11",
        s18 => "♯11",
        s19 => "^5",
        s20 => "♭13",
        s21 => "♮13",
        s22 => "♯13",
        _ => "",
    }.to_string()
}
