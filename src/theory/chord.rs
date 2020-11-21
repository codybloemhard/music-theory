use super::note::*;
use super::interval::*;
use super::scale::*;
use crate::utils::roman_numerals::to_roman_num;

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
//     (true, "6", MAJOR_SIXTH_TETRAD),
//     (true, "6")
// ];

        // match self{
        //     Self::Major(_) => format!("{}", basestr),
        //     Self::Minor(_) => format!("{}", minorstr),
        //     Self::MinorAugmented(_) => format!("{}+", minorstr),
        //     Self::MajorAugmented(_) => format!("{}+", basestr),
        //     Self::MinorDiminished(_) => format!("{}o", minorstr),
        //     Self::MajorDiminished(_) => format!("{}o", basestr),
        //     Self::MajorSixth(_) => format!("{}maj6", basestr),
        //     Self::MinorSixth(_) => format!("{}min6", basestr),
        //     Self::DominantSeventh(_) => format!("{}7", basestr),
        //     Self::AugmentedSeventh(_) => format!("{}+7", basestr),
        //     Self::MajorSeventh(_) => format!("{}∆", basestr),
        //     Self::MinorSeventh(_) => format!("{}-", basestr),
        //     Self::MinorMajorSeventh(_) => format!("{}min(maj7)", basestr),
        //     Self::DiminishedSeventh(_) => format!("{}o7", basestr),
        //     Self::HalfDiminishedSeventh(_) => format!("{}ø", basestr),
        //     Self::Arbitrary(_) => String::new(),
        // }

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
        let mut presence = [false; 11];
        for i in 0..11{
            presence[i] = self.0.contains(&((i + 1) as Note * SEMI));
        }
        let mut used = Vec::new();
        let mut has = |inter: Note, add: bool| { if add { used.push(inter); } presence[((inter - 1) / SEMI) as usize] };
        let (mut name,majort,minort) = if has(MINOR_THIRD, true) { (minorstr,false,true) }
        else if has(MAJOR_THIRD, true) { (basestr,true,false) }
        else { (basestr,false,false) };
        let majort = majort && has(PERFECT_FIFTH, true);
        let minort = minort && has(PERFECT_FIFTH, true);
        if !has(MINOR_THIRD, false) && !has(MAJOR_THIRD, false){
            if has(PERFECT_FOURTH, true) { name.push_str("sus4"); }
            else if has(MAJOR_SECOND, true) { name.push_str("sus2"); }
        }
        let is_diminished = if has(MINOR_THIRD, true) && has(DIMINISHED_FIFTH, true) && !minort{ true }
        else { false };
        let is_augmented = if has(MAJOR_THIRD, true) && has(AUGMENTED_FIFTH, true) && !majort{ true }
        else { false };
        let seventh = if is_diminished && has(DIMINISHED_SEVENTH, true){
            name.push_str("o7"); true
        }
        else if is_diminished && has(MINOR_SEVENTH, true){
            name.push_str("ø"); true
        }
        else if is_augmented && has(MINOR_SEVENTH, true){
            name.push_str("+7"); true
        }
        else if majort && has(MAJOR_SEVENTH, true){
            name.push_str("∆"); true
        }
        else if majort && has(MINOR_SEVENTH, true){
            name.push_str("7"); true
        }
        else if minort && has(MINOR_SEVENTH, true){
            name.push_str("-"); true
        }
        else if minort && has(MAJOR_SEVENTH, true){
            name.push_str("min(maj7)"); true
        }
        else {
            if is_diminished { name.push_str("o"); }
            if is_augmented { name.push_str("+"); }
            false
        };
        let sixth = if !seventh && (minort || majort) && has(MAJOR_SIXTH, true){
            name.push_str("6"); true
        }
        else { false };
        let mut extensions = Vec::new();
        for interval in &self.0{
            if !used.contains(interval){
                extensions.push(interval);
            }
        }
        if extensions.len() > 0{
            name.push_str("(");
            for interval in extensions{
                name.push_str(&interval_chord_extension(*interval));
            }
            name.push_str(")");
        }
        name
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
        chords.push(Scale(chord).as_chord());
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

#[cfg(test)]
mod tests{
    use super::*;
}
