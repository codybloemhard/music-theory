use fnrs::MutFunc;
use super::note::*;
use super::interval::*;
use super::scale::*;
use crate::utils::roman_numerals::to_roman_num;
use crate::utils::misc::*;

pub const NUM_SUPS: [char; 10] = ['⁰', 'ⁱ', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
pub const NUM_SUBS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

pub const MAJOR_TRIAD: &'static [Note] = &[MAJOR_THIRD, PERFECT_FIFTH];
pub const MINOR_TRIAD: &'static [Note] = &[MINOR_THIRD, PERFECT_FIFTH];
pub const MINOR_AUGMENTED_TRIAD: &'static [Note] = &[MINOR_THIRD, AUGMENTED_FIFTH];
pub const MAJOR_AUGMENTED_TRIAD: &'static [Note] = &[MAJOR_THIRD, AUGMENTED_FIFTH];
pub const MINOR_DIMINISHED_TRIAD: &'static [Note] = &[MINOR_THIRD, DIMINISHED_FIFTH];
pub const MAJOR_DIMINISHED_TRIAD: &'static [Note] = &[MAJOR_THIRD, DIMINISHED_FIFTH];
pub const MAJOR_SIXTH_TETRAD: &'static [Note] = &[MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH];
pub const MINOR_SIXTH_TETRAD: &'static [Note] = &[MINOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH];
pub const DOMINANT_SEVENTH_TETRAD: &'static [Note] = &[MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];
pub const AUGMENTED_SEVENTH_TETRAD: &'static [Note] = &[MAJOR_THIRD, AUGMENTED_FIFTH, MINOR_SEVENTH];
pub const MAJOR_SEVENTH_TETRAD: &'static [Note] = &[MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];
pub const MINOR_SEVENTH_TETRAD: &'static [Note] = &[MINOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];
pub const MINOR_MAJOR_SEVENTH_TETRAD: &'static [Note] = &[MINOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];
pub const DIMINISHED_SEVENTH_TETRAD: &'static [Note] = &[MINOR_THIRD, DIMINISHED_FIFTH, DIMINISHED_SEVENTH];
pub const HALF_DIMINISHED_SEVENTH_TETRAD: &'static [Note] = &[MINOR_THIRD, DIMINISHED_FIFTH, MINOR_SEVENTH];

// pub type ChordBook = &[(bool,&str,&'static [Note])];
//
// pub const STD_CHORD_BOOK: ChordBook = &[
//     (true, "", MAJOR_TRIAD),
//     (false, "", MINOR_TRIAD),
//     (true, "+", MAJOR_AUGMENTED_TRIAD),
//     (false, "o", MINOR_DIMINISHED_TRIAD),
//     (true, "maj6", MAJOR_SIXTH_TETRAD),
//     (true, "")
// ];
//
//         match self{
//             Self::Power(_) => format!("{}!", basestr),
//             Self::Major(_) => format!("{}", basestr),
//             Self::Minor(_) => format!("{}", minorstr),
//             Self::MinorAugmented(_) => format!("{}+", minorstr),
//             Self::MajorAugmented(_) => format!("{}+", basestr),
//             Self::MinorDiminished(_) => format!("{}o", minorstr),
//             Self::MajorDiminished(_) => format!("{}o", basestr),
//             Self::MajorSixth(_) => format!("{}maj6", basestr),
//             Self::MinorSixth(_) => format!("{}min6", basestr),
//             Self::DominantSeventh(_) => format!("{}7", basestr),
//             Self::AugmentedSeventh(_) => format!("{}+7", basestr),
//             Self::MajorSeventh(_) => format!("{}∆", basestr),
//             Self::MinorSeventh(_) => format!("{}-", basestr),
//             Self::MinorMajorSeventh(_) => format!("{}min(maj7)", basestr),
//             Self::DiminishedSeventh(_) => format!("{}o7", basestr),
//             Self::HalfDiminishedSeventh(_) => format!("{}ø7", basestr),
//             Self::Arbitrary(_) => String::new(),
//         }
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

    pub fn quality(&self, basestr: String, lower: bool) -> String{
        let mut lowercase = String::new();
        for c in basestr.chars(){
            for l in c.to_lowercase(){
                lowercase.push(l);
            }
        }
        let mut minorcase = String::new();
        minorcase.push_str(&basestr);
        minorcase.push_str("m");
        let minorstr = if lower{
            lowercase
        }else{
            minorcase
        };
        basestr
    }

    pub fn as_string(&self) -> String{
        self.quality("X".to_string(), true)
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

    pub fn as_string(&self, lower: bool) -> String{
        let root = NamedNote::from_note(self.root).to_string_name_sharp();
        self.chord.quality(root, lower)
    }

}

pub fn intervals_from_chord(chord: &Notes) -> Notes{
    if chord.is_empty() { return Vec::new(); }
    let root = chord[0];
    let mut intervals = vec![0];
    for note in chord.iter().skip(1){
        let diff = note - root;
        intervals.push(diff);
    }
    intervals
}

pub fn print_chords(chords: &[Chord], sep: &str){
    let len = chords.len();
    if len <= 0 { return; }
    for chord in chords.iter().take(len - 1){
        print!("{}{}", chord.as_string(), sep);
    }
    println!("{}", &chords[len - 1].as_string());
}

pub fn scale_chords(steps: &Steps, chord_size: usize) -> Vec<Chord>{
    let len = steps.len();
    let mut chords = Vec::new();
    for (i, _) in note_iter(0, &steps.0).enumerate().take(len){
        let mut chord = Vec::new();
        for note in note_iter(0, &steps.0).skip(i).step_by(2).take(chord_size){
            chord.push(note);
        }
        chords.push(Chord(chord));
    }
    chords
}

pub fn strs_scale_chords_roman(steps: &Steps, size: usize) -> Vec<String>{
    let chords = scale_chords(steps, size);
    let mut res = Vec::new();
    for (i, chord) in chords.iter().enumerate(){
        res.push(chord.quality(to_roman_num(i + 1), true));
    }
    res
}

// pub fn scale_chords_intervals(steps: &Steps, size: usize) -> Vec<Notes>{
//     let chords_notes = scale_chords(steps, size);
//     fnrs::map(&chords_notes, &intervals_from_chord)
// }

#[cfg(test)]
mod tests{
    use super::*;
}
