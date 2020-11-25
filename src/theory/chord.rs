use super::note::*;
use super::interval::*;
use super::scale::*;
use crate::utils::roman_numerals::to_roman_num;

pub const NUM_SUPS: [char; 10] = ['⁰', 'ⁱ', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
pub const NUM_SUBS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

pub const MAJOR: &[Note] = &[MAJOR_THIRD, PERFECT_FIFTH];
pub const MINOR: &[Note] = &[MINOR_THIRD, PERFECT_FIFTH];
pub const MINOR_AUGMENTED: &[Note] = &[MINOR_THIRD, AUGMENTED_FIFTH];
pub const MAJOR_AUGMENTED: &[Note] = &[MAJOR_THIRD, AUGMENTED_FIFTH];
pub const MINOR_DIMINISHED: &[Note] = &[MINOR_THIRD, DIMINISHED_FIFTH];
pub const MAJOR_DIMINISHED: &[Note] = &[MAJOR_THIRD, DIMINISHED_FIFTH];
pub const SUS2: &[Note] = &[MAJOR_SECOND,PERFECT_FIFTH];
pub const SUS4: &[Note] = &[PERFECT_FOURTH,PERFECT_FIFTH];
pub const SUPER_SUS: &[Note] = &[MAJOR_SECOND,PERFECT_FOURTH];
pub const PHRYGIAN: &[Note] = &[MINOR_SECOND,PERFECT_FIFTH];
pub const LYDIAN: &[Note] = &[AUGMENTED_FOURTH,PERFECT_FIFTH];
pub const LOCRIAN2: &[Note] = &[MINOR_SECOND,DIMINISHED_FIFTH];
pub const LOCRIAN4: &[Note] = &[PERFECT_FOURTH,DIMINISHED_FIFTH];
pub const SUPER_LOCRIAN: &[Note] = &[MINOR_SECOND,PERFECT_FOURTH,DIMINISHED_FIFTH];
pub const ITALIAN_SIXTH: &[Note] = &[MAJOR_THIRD,MINOR_SEVENTH];
pub const FRENCH_SIXTH: &[Note] = &[MAJOR_THIRD,TRITONE,MINOR_SEVENTH];
pub const MAJOR_SIXTH_CHORD: &[Note] = &[MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH];
pub const MINOR_SIXTH_CHORD: &[Note] = &[MINOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH];
pub const MAJOR_SEVENTH_CHORD: &[Note] = &[MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];
pub const MINOR_SEVENTH_CHORD: &[Note] = &[MINOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];
pub const DOMINANT_SEVENTH: &[Note] = &[MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];
pub const MINOR_MAJOR_SEVENTH: &[Note] = &[MINOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];
pub const HALF_DIMINISHED_SEVENTH: &[Note] = &[MINOR_THIRD, DIMINISHED_FIFTH, MINOR_SEVENTH];
pub const DIMINISHED_SEVENTH_CHORD: &[Note] = &[MINOR_THIRD, DIMINISHED_FIFTH, DIMINISHED_SEVENTH];
pub const AUGMENTED_SEVENTH_CHORD: &[Note] = &[MAJOR_THIRD, AUGMENTED_FIFTH, MINOR_SEVENTH];
pub const MU_CHORD: &[Note] = &[MAJOR_SECOND,MAJOR_THIRD,PERFECT_FIFTH];
pub const SIX_NINE_CHORD: &[Note] = &[MAJOR_THIRD,PERFECT_FIFTH,MAJOR_SIXTH,NINETH];

// (pattern, name, major base string?, extended collection?)
pub type ChordBook = &'static [(&'static [Note],&'static str,bool,bool)];

pub const STD_CHORD_BOOK: ChordBook = &[
    (MAJOR, "", true, false),
    (MINOR, "", false, false),
    (MINOR_AUGMENTED, "+", false, true),
    (MAJOR_AUGMENTED, "+", true, false),
    (MINOR_DIMINISHED, "°", false, false),
    (MAJOR_DIMINISHED, "°", true, true),
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
    (HALF_DIMINISHED_SEVENTH, "ø", false, false),
    (DIMINISHED_SEVENTH_CHORD, "°7", false, false),
    (AUGMENTED_SEVENTH_CHORD, "+7", true, false),
    (MU_CHORD, "μ", true, true),
    (SIX_NINE_CHORD, "6/9", true, false),
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
        // Just print intervals
        let spelled_out = |basestr: String|{
            let mut spelled_out = basestr;
            spelled_out.push('[');
            for int in &self.0{
                spelled_out.push_str(&interval_chord_extension(*int));
            }
            spelled_out.push(']');
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
        minorcase.push('m');
        let minorstr = if lower{ lowercase }
        else{ minorcase };
        let sname = |major_base| if major_base { basestr.clone() } else { minorstr.clone() };
        // Find exact matches in the book
        for (pattern,postfix,majorstr,ext) in STD_CHORD_BOOK{
            if pattern != &self.0 { continue; }
            if *ext && style == ChordStyling::Std { continue; }
            let mut name = sname(*majorstr);
            name.push_str(postfix);
            return name
        }
        // Extended chords
        let mut name = String::new();
        let mut baselen = 0;
        for (pattern,postfix,majorstr,ext) in STD_CHORD_BOOK{
            if *ext && style == ChordStyling::Std { continue; }
            if self.0.len() <= pattern.len() { continue; }
            if baselen >= pattern.len() { continue; }
            let base = self.0.iter().take(pattern.len()).copied().collect::<Vec<Note>>();
            if &base != pattern { continue; }
            baselen = pattern.len();
            name = sname(*majorstr);
            name.push_str(postfix);
        }
        let ext_name = |bl,mut name: String|{
            if bl >= self.0.len() { return name; }
            name.push('(');
            self.0.iter().skip(bl).for_each(|int|name.push_str(&interval_chord_extension(*int)));
            name.push(')');
            name
        };
        if baselen > 0 { return ext_name(baselen,name); }
        //Sus chords, maybe extended
        baselen = 0;
        for (pattern,postfix,_,ext) in STD_CHORD_BOOK{
            if *ext && style == ChordStyling::Std { continue; }
            if self.0.len() < pattern.len() { continue; }
            if baselen >= pattern.len() { continue; }
            let base = self.0.iter().take(pattern.len()).copied().collect::<Vec<Note>>();
            let res = pattern.iter().zip(base.iter()).fold(10, |res,(ba,se)|{
                if res == 0 { 0 }
                else {
                    if se == ba { 10 }
                    else if se == &MAJOR_SECOND && (ba == &MINOR_THIRD || ba == &MAJOR_THIRD) { 2 }
                    else if se == &PERFECT_FOURTH && (ba == &MINOR_THIRD || ba == &MAJOR_THIRD) { 4 }
                    else { 0 }.min(res)
                }
            });
            if res == 0 || res == 10 { continue; }
            baselen = pattern.len();
            name = sname(true);
            name.push_str(postfix);
            name.push_str(&format!("sus{}", res));
        }
        if baselen > 0 { return ext_name(baselen,name); }
        // Default to spelling out
        spelled_out(basestr)
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
    if len == 0 { return; }
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
}
