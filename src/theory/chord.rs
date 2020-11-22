use super::note::*;
use super::interval::*;
use super::scale::*;
use crate::utils::roman_numerals::to_roman_num;

pub const NUM_SUPS: [char; 10] = ['⁰', 'ⁱ', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
pub const NUM_SUBS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

pub const MAJOR: &'static [Note] = &[MAJOR_THIRD, PERFECT_FIFTH];
pub const MINOR: &'static [Note] = &[MINOR_THIRD, PERFECT_FIFTH];
pub const MINOR_AUGMENTED: &'static [Note] = &[MINOR_THIRD, AUGMENTED_FIFTH];
pub const MAJOR_AUGMENTED: &'static [Note] = &[MAJOR_THIRD, AUGMENTED_FIFTH];
pub const MINOR_DIMINISHED: &'static [Note] = &[MINOR_THIRD, DIMINISHED_FIFTH];
pub const MAJOR_DIMINISHED: &'static [Note] = &[MAJOR_THIRD, DIMINISHED_FIFTH];
pub const SUS2: &'static [Note] = &[MAJOR_SECOND,PERFECT_FIFTH];
pub const SUS4: &'static [Note] = &[PERFECT_FOURTH,PERFECT_FIFTH];
pub const SUPER_SUS: &'static [Note] = &[MAJOR_SECOND,PERFECT_FOURTH];
pub const PHRYGIAN: &'static [Note] = &[MINOR_SECOND,PERFECT_FIFTH];
pub const LYDIAN: &'static [Note] = &[AUGMENTED_FOURTH,PERFECT_FIFTH];
pub const LOCRIAN2: &'static [Note] = &[MINOR_SECOND,DIMINISHED_FIFTH];
pub const LOCRIAN4: &'static [Note] = &[PERFECT_FOURTH,DIMINISHED_FIFTH];
pub const SUPER_LOCRIAN: &'static [Note] = &[MINOR_SECOND,PERFECT_FOURTH,DIMINISHED_FIFTH];
pub const ITALIAN_SIXTH: &'static [Note] = &[MAJOR_THIRD,MINOR_SEVENTH];
pub const FRENCH_SIXTH: &'static [Note] = &[MAJOR_THIRD,TRITONE,MINOR_SEVENTH];
pub const MAJOR_SIXTH_CHORD: &'static [Note] = &[MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH];
pub const MINOR_SIXTH_CHORD: &'static [Note] = &[MINOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH];
pub const MAJOR_SEVENTH_CHORD: &'static [Note] = &[MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];
pub const MINOR_SEVENTH_CHORD: &'static [Note] = &[MINOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];
pub const DOMINANT_SEVENTH: &'static [Note] = &[MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];
pub const MINOR_MAJOR_SEVENTH: &'static [Note] = &[MINOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];
pub const HALF_DIMINISHED_SEVENTH: &'static [Note] = &[MINOR_THIRD, DIMINISHED_FIFTH, MINOR_SEVENTH];
pub const DIMINISHED_SEVENTH_CHORD: &'static [Note] = &[MINOR_THIRD, DIMINISHED_FIFTH, DIMINISHED_SEVENTH];
pub const AUGMENTED_SEVENTH_CHORD: &'static [Note] = &[MAJOR_THIRD, AUGMENTED_FIFTH, MINOR_SEVENTH];

// (pattern, name, major base string?, extended collection?)
pub type ChordBook = &'static [(&'static [Note],&'static str,bool,bool)];

pub const STD_CHORD_BOOK: ChordBook = &[
    (MAJOR, "", true, false),
    (MINOR, "", false, false),
    (MINOR_AUGMENTED, "+", false, true),
    (MAJOR_AUGMENTED, "+", true, false),
    (MINOR_DIMINISHED, "°", false, false),
    (MAJOR_DIMINISHED, "°", true, true),
    (SUS2, "sus2", true, false),
    (SUS4, "sus4", true, false),
    (SUPER_SUS, "ssus", true, true),
    (PHRYGIAN, "phry", true, false),
    (LYDIAN, "lyd", true, false),
    (LOCRIAN2, "loc2", true, false),
    (LOCRIAN4, "loc4", true, false),
    (SUPER_LOCRIAN, "o", true, true),
    (ITALIAN_SIXTH, "it+6", true, false),
    (FRENCH_SIXTH, "fr+6", true, false),
    (MAJOR_SIXTH_CHORD, "6", true, false),
    (MINOR_SIXTH_CHORD, "6", false, false),
    (MAJOR_SEVENTH_CHORD, "∆", true, false),
    (MINOR_SEVENTH_CHORD, "-", false, false),
    (DOMINANT_SEVENTH, "7", true, false),
    (MINOR_MAJOR_SEVENTH, "-(maj7)", true, false),
    (HALF_DIMINISHED_SEVENTH, "°7", false, false),
    (DIMINISHED_SEVENTH_CHORD, "ø", false, false),
    (AUGMENTED_SEVENTH_CHORD, "+7", true, false),
];

#[derive(PartialEq,Eq,Clone,Copy)]
pub enum ChordStyling{ Std, Extended, SpelledOut }

impl Chord{
    pub fn new(intervals: &[Note]) -> Self{
        Chord(intervals.to_owned())
    }

    pub fn same_intervals(&self, blueprint: &[Note]) -> bool{
        self.0 == blueprint
    }

    pub fn has_intervals(&self, blueprint: &[Note]) -> bool{
        for note in blueprint{
            if !self.0.contains(note){
                return false;
            }
        }
        true
    }

    pub fn quality(&self, basestr: String, lower: bool, style: ChordStyling) -> String{
        let spelled_out = |basestr: String|{
            let mut spelled_out = basestr;
            spelled_out.push_str("[");
            for int in &self.0{
                spelled_out.push_str(&interval_chord_extension(*int));
            }
            spelled_out.push_str("]");
            spelled_out
        };
        if style == ChordStyling::SpelledOut{
            return spelled_out(basestr);
        }
        let mut lowercase = String::new();
        for c in basestr.chars(){
            for l in c.to_lowercase(){
                lowercase.push(l);
            }
        }
        let mut minorcase = String::new();
        minorcase.push_str(&basestr);
        minorcase.push_str("m");
        let minorstr = if lower{ lowercase }
        else{ minorcase };
        let sname = |major_base| if major_base { basestr.clone() } else { minorstr };
        for (pattern,postfix,majorstr,ext) in STD_CHORD_BOOK{
            if pattern != &self.0 { continue; }
            if *ext && style == ChordStyling::Std { continue; }
            let mut name = sname(*majorstr);
            name.push_str(postfix);
            return name
        }
        return spelled_out(basestr);
    }

    pub fn as_string(&self, styling: ChordStyling) -> String{
        self.quality("X".to_string(), true, styling)
    }
}

pub struct RootedChord{
    root: Note,
    chord: Chord,
}

impl RootedChord{
    pub fn from_chord(root: Note, chord: Chord) -> Self{
        Self{ root, chord }
    }

    pub fn from_intervals(root: Note, intervals: &[Note]) -> Self{
        Self{ root, chord: Chord::new(intervals) }
    }

    pub fn as_string(&self, lower: bool, styling: ChordStyling) -> String{
        let root = NamedNote::from_note(self.root).to_string_name_sharp();
        self.chord.quality(root, lower, styling)
    }

}

pub fn print_chords(chords: &[Chord], sep: &str, styling: ChordStyling){
    let len = chords.len();
    if len <= 0 { return; }
    for chord in chords.iter().take(len - 1){
        print!("{}{}", chord.as_string(styling), sep);
    }
    println!("{}", &chords[len - 1].as_string(styling));
}

pub fn scale_chords(steps: &Steps, chord_size: usize) -> Vec<Chord>{
    let len = steps.len();
    let mut chords = Vec::new();
    for (i, _) in note_iter(0, &steps.0).enumerate().take(len){
        let mut chord = Vec::new();
        for note in note_iter(0, &steps.0).skip(i).step_by(2).take(chord_size){
            chord.push(note);
        }
        chords.push(Scale(chord).as_chord());
    }
    chords
}

pub fn strs_scale_chords_roman(steps: &Steps, size: usize, styling: ChordStyling) -> Vec<String>{
    let chords = scale_chords(steps, size);
    let mut res = Vec::new();
    for (i, chord) in chords.iter().enumerate(){
        res.push(chord.quality(to_roman_num(i + 1), true, styling));
    }
    res
}

#[cfg(test)]
mod tests{
    use super::*;
}
