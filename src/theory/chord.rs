use super::{ Note, Notes, Scale };
use super::interval::*;
use super::traits::{ VecWrapper, Wrapper, ToChord, ToNamedInterval };
use super::super::utils::{ as_lowercase };
// use super::note::*;
// use super::scale::*;
// use crate::utils::roman_numerals::to_roman_num;

use std::collections::HashSet;

pub const NUM_SUPS: [char; 10] = ['⁰', 'ⁱ', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
pub const NUM_SUBS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

pub const MAJOR: &[Note] = &[_MAJ3, _PER5];
pub const MINOR: &[Note] = &[_MIN3, _PER5];
pub const MINOR_AUGMENTED: &[Note] = &[_MIN3, _AUG5];
pub const MAJOR_AUGMENTED: &[Note] = &[_MAJ3, _AUG5];
pub const MINOR_DIMINISHED: &[Note] = &[_MIN3, _DIM5];
pub const MAJOR_DIMINISHED: &[Note] = &[_MAJ3, _DIM5];
pub const SUS2: &[Note] = &[_MAJ2, _PER5];
pub const SUS4: &[Note] = &[_PER4, _PER5];
pub const SUPER_SUS: &[Note] = &[_MAJ2, _PER4];
pub const PHRYGIAN: &[Note] = &[_MIN2, _PER5];
pub const LYDIAN: &[Note] = &[_AUG4, _PER5];
pub const LOCRIAN2: &[Note] = &[_MIN2, _DIM5];
pub const LOCRIAN4: &[Note] = &[_PER4, _DIM5];
pub const SUPER_LOCRIAN: &[Note] = &[_MIN2, _PER4, _DIM5];
pub const MAJOR_SIXTH_CHORD: &[Note] = &[_MAJ3, _PER5, _MAJ6];
pub const MINOR_SIXTH_CHORD: &[Note] = &[_MIN3, _PER5, _MAJ6];
pub const MAJOR_SEVENTH_CHORD: &[Note] = &[_MAJ3, _PER5, _MAJ7];
pub const MINOR_SEVENTH_CHORD: &[Note] = &[_MIN3, _PER5, _MIN7];
pub const DOMINANT_SEVENTH: &[Note] = &[_MAJ3, _PER5, _MIN7];
pub const MINOR_MAJOR_SEVENTH: &[Note] = &[_MIN3, _PER5, _MAJ7];
pub const HALF_DIMINISHED_SEVENTH: &[Note] = &[_MIN3, _DIM5, _MIN7];
pub const DIMINISHED_SEVENTH_CHORD: &[Note] = &[_MIN3, _DIM5, _DIM7];
pub const AUGMENTED_SEVENTH_CHORD: &[Note] = &[_MAJ3, _AUG5, _MIN7];
pub const MU_CHORD: &[Note] = &[_MAJ2, _MAJ3, _PER5];
pub const SIX_NINE_CHORD: &[Note] = &[_MAJ3, _PER5, _MAJ6, _MAJ9];
pub const MAJOR_NINTH_CHORD: &[Note] = &[_MAJ3, _PER5, _MAJ7, _MAJ9];
pub const MINOR_NINTH_CHORD: &[Note] = &[_MIN3, _PER5, _MIN7, _MIN9];
pub const MAJOR_ELEVENTH_CHORD: &[Note] = &[_MAJ3, _PER5, _MAJ7, _MAJ9, _MAJ11];
pub const MINOR_ELEVENTH_CHORD: &[Note] = &[_MIN3, _PER5, _MIN7, _MIN9, _MIN11];
pub const MAJOR_THIRTEENTH_CHORD: &[Note] = &[_MAJ3, _PER5, _MAJ7, _MAJ9, _MAJ11, _MAJ13];
pub const MINOR_THIRTEENTH_CHORD: &[Note] = &[_MIN3, _PER5, _MIN7, _MIN9, _MIN11, _MIN13];

// (pattern, name, major base string?, extended collection?)
pub type ChordBook = &'static [(&'static [Note], &'static str, bool, bool)];

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
    (SUPER_LOCRIAN, "sloc", true, true),
    (MAJOR_SIXTH_CHORD, "⁶", true, false),
    (MINOR_SIXTH_CHORD, "⁶", false, false),
    (MAJOR_SEVENTH_CHORD, "∆", true, false),
    (MINOR_SEVENTH_CHORD, "-", false, false),
    (DOMINANT_SEVENTH, "⁷", true, false),
    (MINOR_MAJOR_SEVENTH, "-∆", true, false),
    (HALF_DIMINISHED_SEVENTH, "ø", false, false),
    (DIMINISHED_SEVENTH_CHORD, "°⁷", false, false),
    (AUGMENTED_SEVENTH_CHORD, "+⁷", true, false),
    (MU_CHORD, "μ", true, true),
    (SIX_NINE_CHORD, "6/9", true, false),
    (MAJOR_NINTH_CHORD, "9", true, false),
    (MINOR_NINTH_CHORD, "9", false, false),
    (MAJOR_ELEVENTH_CHORD, "11", true, false),
    (MINOR_ELEVENTH_CHORD, "11", false, false),
    (MAJOR_THIRTEENTH_CHORD, "13", true, false),
    (MINOR_THIRTEENTH_CHORD, "13", false, false)
];

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Chord(pub Vec<Note>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChordStyling{ Std, Extended, SpelledOut }

fn is_sorted<T: PartialOrd + Copy>(v: &[T]) -> bool{
    let mut last = v[0];
    for x in v{
        if last > *x { return false; }
        last = *x;
    }
    true
}

ImplVecWrapper!(Chord, Note);

impl Wrapper for Chord{
    type Inner = Notes;

    fn wrap(scale: Self::Inner) -> Option<Self>{
        if scale.is_empty() || !is_sorted(&scale){
            None
        } else {
            Some(Self(scale))
        }
    }

    fn unwrap(self) -> Self::Inner{
        self.0
    }
}

impl Chord{
    pub fn new(intervals: &[Note]) -> Self{
        let mut ints = intervals.to_owned();
        ints.sort();
        Chord(ints)
    }

    pub fn same_intervals(&self, blueprint: &[Note]) -> bool{
        self.0 == blueprint
    }

    pub fn normalized(self) -> Self{
        let mut res = Vec::new();
        let mut grid = [false; 12];
        grid[0] = true;
        for note in self.0{
            let note = note % _OCTAVE;
            if grid[note.0 as usize] { continue; }
            res.push(note);
            grid[note.0 as usize] = true;
        }
        res.sort();
        Chord(res)
    }

    pub fn quality(&self, basestr: String, lower: bool, style: ChordStyling) -> String{
        fn spelled_out(mut basestr: String, notes: &[Note]) -> String{
            basestr.push('[');
            for int in notes{ // When spelling out literally, we want to spell a 9th at 9 not as 2
                basestr.push_str(&Interval(int.0 as i32).to_named_interval_mod().to_string());
            }
            basestr.push(']');
            basestr
        }
        if style == ChordStyling::SpelledOut{ return spelled_out(basestr, &self.0); }
        let chord = self.clone().normalized();
        let lowercase = as_lowercase(&basestr);
        let mut minorcase = String::new();
        minorcase.push_str(&basestr);
        minorcase.push('m');
        let minorstr = if lower { lowercase } else { minorcase };
        let sname = |major_base| if major_base { basestr.clone() } else { minorstr.clone() };
        // find longest pattern of which all intervals are in the chord
        let per5 = chord.contains(&_PER5);
        let (mut pat, mut pmaj, mut ppfx, mut no3) = (vec![], true, "", false);
        'outer: for (pattern, postfix, majorstr, ext) in STD_CHORD_BOOK{
            if *ext && style == ChordStyling::Std { continue; }
            let pattern = Chord(pattern.to_vec()).normalized().0;
            if pattern == chord.0 { // exact match
                let mut name = sname(*majorstr);
                name.push_str(postfix);
                return name;
            }
            let mut nothird = false;
            for int in &pattern{
                let isin = chord.contains(int);
                if isin { continue; }
                let patnotsus = !(pattern.contains(&_MAJ2) || pattern.contains(&_PER4));
                if (*int == _MIN3 || *int == _MAJ3) && per5 && patnotsus{
                    nothird = true;
                    continue;
                }
                continue 'outer;
            }
            if pattern.len() <= pat.len() { continue; }
            pmaj = *majorstr;
            ppfx = postfix;
            pat = pattern;
            no3 = nothird;
        }
        if !pat.is_empty(){ // found a usable base chord to extend
            let mut name = sname(pmaj);
            name.push_str(ppfx);
            let sus = if no3{ // kinda sus brø
                let sus2 = chord.contains(&_MAJ2);
                let sus4 = chord.contains(&_PER4);
                if sus2 && sus4 { name.push_str("ssus"); 10 }
                else if sus2 { name.push_str("sus2"); 2 }
                else if sus4 { name.push_str("sus4"); 5 }
                else { name.push_str("no3"); 0 }
            } else { 0 };
            name.push('(');
            let mut atleastone = false;
            for int in chord.iter(){
                if pat.contains(int) { continue; }
                if int.0 == sus || ((int.0 == 2 || int.0 == 5) && sus == 10) { continue; }
                name.push_str(&Interval(int.0 as i32 + 12).to_named_interval_mod().to_string());
                atleastone = true;
            }
            if atleastone { name.push(')'); } else { name.pop(); };
            name
        } else {
            spelled_out(basestr, &self.0)
        }
    }

    pub fn as_string(&self, styling: ChordStyling) -> String{
        self.quality("X".to_string(), true, styling)
    }
}
//
// impl ToScale for Chord{
//     fn to_scale(&self, root: Note) -> Scale{
//         let mut scale = vec![root];
//         for int in &self.0{
//             scale.push(root + *int);
//         }
//         Scale(scale)
//     }
// }
//
// #[derive(PartialEq, Eq, Hash, Clone, Default)]
// pub struct RootedChord{
//     pub root: Note,
//     pub chord: Chord,
// }
//
// impl RootedChord{
//     pub fn from_chord(root: Note, chord: Chord) -> Self{
//         Self{ root, chord }
//     }
//
//     pub fn from_intervals(root: Note, intervals: &[Note]) -> Self{
//         Self{ root, chord: Chord::new(intervals) }
//     }
//
//     pub fn from_scale(scale: Scale) -> Self{
//         if scale.is_empty() { Self{ root: 0, chord: Chord(Vec::new()) } }
//         else if scale.len() == 1 { Self{ root: scale.0[0], chord: Chord(Vec::new()) } }
//         else { Self::from_chord(scale.0[0], scale.into_chord()) }
//     }
//
//     pub fn to_scale(&self) -> Scale{
//         let mut scale = vec![self.root];
//         for int in &self.chord.0{
//             scale.push(self.root + *int);
//         }
//         Scale(scale)
//     }
//
//     fn normalized(self) -> Self{
//         Self {
//             root: self.root % _OCTAVE,
//             chord: self.chord.normalized(),
//         }
//     }
//
//     pub fn to_chordtone_wholetone_scale(&self) -> Scale{
//         let mut res = Vec::new();
//         let scale = self.to_scale();
//         if scale.len() < 4 { return Scale(res); }
//         for (i,note) in scale.0.iter().enumerate().take(4){
//             res.push(*note);
//             let between = if scale.len() > i + 4 { scale.0[i + 4] - _OCTAVE }
//             else { *note + _MAJ2 };
//             res.push(between);
//         }
//         Scale(res)
//     }
//
//     pub fn to_inversion(&self) -> RootedChord{
//         let mut scale = self.to_scale();
//         if scale.is_empty() { return RootedChord::default(); }
//         let mut root = scale.0[0];
//         if scale.len() == 1 { return RootedChord::from_intervals(root, &[]); }
//         let top = scale.0[scale.len() - 1];
//         while root < top {
//             root += _OCTAVE;
//         }
//         scale.0.remove(0);
//         scale.0.push(root);
//         Self::from_scale(scale)
//     }
//
//     pub fn all_inversions(&self) -> Vec<RootedChord>{
//         let len = self.chord.len() + 1;
//         let mut inv = self.clone();
//         let mut res = Vec::new();
//         for _ in 0..len{
//             inv = inv.to_inversion();
//             res.push(inv.clone());
//         }
//         res
//     }
//
//     pub fn as_string(&self, lower: bool, styling: ChordStyling) -> String{
//         let root = self.root.to_pc().to_string_name(); //NamedNote::from_note(self.root).to_string_name();
//         self.chord.quality(root, lower, styling)
//     }
// }
//
// #[derive(PartialEq, Eq, Hash, Clone)]
// pub struct RelativeChord{
//     pub root: Note,
//     pub chord: Chord,
// }
//
// impl RelativeChord{
//     pub fn from_chord(root: Note, chord: Chord) -> Self{
//         Self{ root, chord }
//     }
//
//     pub fn from_intervals(root: Note, intervals: &[Note]) -> Self{
//         Self{ root, chord: Chord::new(intervals) }
//     }
//
//     pub fn from_template(semis: Note, intervals: &[Note]) -> Self{
//         Self{ root: semis, chord: Chord::new(intervals) }
//     }
//
//     pub fn as_string(&self, lower: bool, styling: ChordStyling) -> String{
//         let root = to_degree(self.root);
//         self.chord.quality(root, lower, styling)
//     }
// }
//
// impl std::fmt::Display for RelativeChord{
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
//         let root = format!("<X{}{}>", if self.root >= 0 { "+" } else { "" }, self.root);
//         let res = self.chord.quality(root, true, ChordStyling::Extended);
//         write!(f, "{}", res)
//     }
// }
//
// pub fn print_chords(chords: &[Chord], sep: &str, styling: ChordStyling){
//     let len = chords.len();
//     if len == 0 { return; }
//     for chord in chords.iter().take(len - 1){
//         print!("{}{}", chord.as_string(styling), sep);
//     }
//     println!("{}", &chords[len - 1].as_string(styling));
// }
//
// pub fn scale_chords(steps: &Steps, chord_size: usize) -> Vec<Chord>{
//     let len = steps.len();
//     let mut chords = Vec::new();
//     for (i, _) in note_iter(0, &steps.0).enumerate().take(len){
//         let mut chord = Vec::new();
//         for note in note_iter(0, &steps.0).skip(i).step_by(2).take(chord_size){
//             chord.push(note);
//         }
//         chords.push(Scale(chord).into_chord());
//     }
//     chords
// }
//
// pub fn rooted_scale_chords(steps: &Steps, tonic: Note, chord_size: usize) -> Vec<RootedChord>{
//     let len = steps.len();
//     let mut chords = Vec::new();
//     for (i, _) in note_iter(tonic, &steps.0).enumerate().take(len){
//         let mut chord = Vec::new();
//         for note in note_iter(tonic, &steps.0).skip(i).step_by(2).take(chord_size){
//             chord.push(note);
//         }
//         chords.push(RootedChord::from_scale(Scale(chord)));
//     }
//     chords
// }
//
// pub fn strs_scale_chords_roman(steps: &Steps, size: usize, styling: ChordStyling) -> Vec<String>{
//     let chords = scale_chords(steps, size);
//     let mut res = Vec::new();
//     for (i, chord) in chords.iter().enumerate(){
//         res.push(chord.quality(to_roman_num(i + 1), true, styling));
//     }
//     res
// }
//
// pub fn strs_scale_chords(steps: &Steps, tonic: Note, size: usize, styling: ChordStyling) -> Vec<String>{
//     let chords = rooted_scale_chords(steps, tonic, size);
//     let mut res = Vec::new();
//     for chord in chords.iter(){
//         res.push(chord.as_string(true, styling));
//     }
//     res
// }

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_is_sorted(){
        assert_eq!(is_sorted(&[Note(0)]), true);
        assert_eq!(is_sorted(&[Note(0), Note(1)]), true);
        assert_eq!(is_sorted(&[Note(1), Note(1)]), true);
        assert_eq!(is_sorted(&[Note(3), Note(1)]), false);
        assert_eq!(is_sorted(&[Note(3), Note(4), Note(3), Note(5)]), false);
    }

    #[test]
    fn chord_wrap(){
        assert_eq!(Chord::wrap(vec![]), None);
        assert_eq!(Chord::wrap(vec![Note(1), Note(0)]), None);
        assert_eq!(Chord::wrap(vec![Note(0), Note(1)]), Some(Chord(vec![Note(0), Note(1)])));
    }

    #[test]
    fn chord_unwrap(){
        assert_eq!(Chord(vec![Note(0), Note(1)]).unwrap(), vec![Note(0), Note(1)]);
    }

    #[test]
    fn chord_new(){
        assert_eq!(
            Chord::new(&[Note(0), Note(1), Note(2)]),
            Chord(vec![Note(0), Note(1), Note(2)])
        );
        assert_eq!(
            Chord::new(&[Note(1), Note(0), Note(2)]),
            Chord(vec![Note(0), Note(1), Note(2)])
        );
    }

    #[test]
    fn chord_same_intervals(){
        assert_eq!(Chord(vec![Note(4), Note(7)]).same_intervals(&MAJOR), true);
    }

    #[test]
    fn chord_normalized(){
        assert_eq!(Chord::new(&MAJOR).normalized(), Chord::new(&MAJOR));
        assert_eq!(
            Chord::new(&[_MAJ3, _PER5, _OCTAVE, _MAJ9, _PER12]).normalized(),
            Chord::new(&[_MAJ2, _MAJ3, _PER5])
        );
        assert_eq!(
            Chord::new(&[_MAJ3, _MAJ9, _PER12]).normalized(),
            Chord::new(&[_MAJ2, _MAJ3, _PER5])
        );
        assert_eq!(
            Chord::new(&[_MAJ3, _MAJ3, _MAJ9, _PER12]).normalized(),
            Chord::new(&[_MAJ2, _MAJ3, _PER5])
        );
    }

    #[test]
    fn test_chords_strings(){
        let spl = ChordStyling::SpelledOut;
        let std = ChordStyling::Std;
        let ext = ChordStyling::Extended;
        assert_eq!(&Chord::new(&[_MAJ3,_PER5]).as_string(std), "X");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5]).as_string(ext), "X");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5]).as_string(spl), "X[♮3♮5]");
        assert_eq!(&Chord::new(&[_MIN3,_PER5]).as_string(std), "x");
        assert_eq!(&Chord::new(&[_MIN3,_PER5]).quality(String::from("Y"), false, std), "Ym");
        assert_eq!(&Chord::new(&[_MIN3,_PER5]).quality(String::from("Y"), true, std), "y");
        assert_eq!(&Chord::new(&[_MIN3,_DIM5]).as_string(std), "x°");
        assert_eq!(&Chord::new(&[_MAJ3,_DIM5]).as_string(std), "X[♮3♭5]");
        assert_eq!(&Chord::new(&[_MAJ3,_DIM5]).as_string(ext), "X°");
        assert_eq!(&Chord::new(&[_MIN3]).as_string(std), "X[♭3]");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5]).as_string(std), "Xsus2");
        assert_eq!(&Chord::new(&[_PER4,_PER5]).as_string(std), "Xsus4");
        assert_eq!(&Chord::new(&[_MAJ3,_AUG5]).as_string(std), "X+");
        assert_eq!(&Chord::new(&[_MIN3,_AUG5]).as_string(std), "X[♭3♭6]");
        assert_eq!(&Chord::new(&[_MIN3,_AUG5]).as_string(ext), "x+");
        assert_eq!(&Chord::new(&[_MAJ2,_PER4]).as_string(std), "X[♮2♮4]");
        assert_eq!(&Chord::new(&[_MAJ2,_PER4]).as_string(ext), "Xssus");
        assert_eq!(&Chord::new(&[_MIN2,_PER5]).as_string(std), "Xphry");
        assert_eq!(&Chord::new(&[_AUG4,_PER5]).as_string(std), "Xlyd");
        assert_eq!(&Chord::new(&[_MIN2,_DIM5]).as_string(std), "Xloc2");
        assert_eq!(&Chord::new(&[_PER4,_DIM5]).as_string(std), "Xloc4");
        assert_eq!(&Chord::new(&[_MIN2,_PER4,_DIM5]).as_string(std), "Xloc2(♮11)");
        assert_eq!(&Chord::new(&[_MIN2,_PER4,_DIM5]).as_string(ext), "Xsloc");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ6]).as_string(std), "X⁶");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MAJ6]).as_string(std), "x⁶");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ7]).as_string(std), "X∆");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MIN7]).as_string(std), "x-");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MIN7]).as_string(std), "x-");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MIN7]).as_string(std), "X⁷");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MAJ7]).as_string(std), "X-∆");
        assert_eq!(&Chord::new(&[_MIN3,_DIM5,_MIN7]).as_string(std), "xø");
        assert_eq!(&Chord::new(&[_MIN3,_DIM5,_DIM7]).as_string(std), "x°⁷");
        assert_eq!(&Chord::new(&[_MAJ3,_AUG5,_MIN7]).as_string(std), "X+⁷");
        assert_eq!(&Chord::new(&[_MAJ2,_MAJ3,_PER5]).as_string(std), "X(♮9)");
        assert_eq!(&Chord::new(&[_MAJ2,_MAJ3,_PER5]).as_string(ext), "Xμ");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ6,_MAJ9]).as_string(std), "X6/9");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MIN7,_MIN9]).as_string(std), "x9");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ7,_MAJ9]).as_string(std), "X9");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MIN7,_MIN9,_MIN11]).as_string(std), "x11");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ7,_MAJ9,_MAJ11]).as_string(std), "X11");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MIN7,_MIN9,_MIN11,_MIN13]).as_string(std), "x13");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ7,_MAJ9,_MAJ11,_MAJ13]).as_string(std), "X13");

        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MAJ6,_MAJ9]).as_string(std), "X⁶sus2");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MAJ6,_MAJ9]).as_string(std), "X⁶ssus");
        //assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MIN7,_MIN9]).as_string(std), "x9sus2");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MAJ7,_MAJ9]).as_string(std), "X∆sus2");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MAJ7,_MAJ9]).as_string(std), "X∆ssus");

        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MIN7,_MIN9,_MIN11]).as_string(std), "x11sus2");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MIN7,_MIN9,_MIN11]).as_string(std), "x11sus4");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MAJ7,_MAJ9,_MAJ11]).as_string(std), "X∆ssus");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MAJ7,_MAJ9,_MAJ11]).as_string(std), "X∆ssus");

        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MIN7,_MIN9,_MIN11,_MIN13]).as_string(std), "x13sus2");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MIN7,_MIN9,_MIN11,_MIN13]).as_string(std), "x13sus4");
        //assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MAJ7,_MAJ9,_MAJ11,_MAJ13]).as_string(std), "X13");
        //assert_eq!(&Chord::new(&[_PER4,_PER5,_MAJ7,_MAJ9,_MAJ11,_MAJ13]).as_string(std), "X13");

        assert_eq!(&Chord::new(&[_PER4,_PER5,_MIN7,_AUG9,_AUG13]).as_string(std), "x-(♮11)");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MAJ7,_MIN9,_AUG11]).as_string(std), "X∆sus2(♭9♯11)");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MIN7,_MIN9,_MIN11]).as_string(std), "x11sus4");
    }
}
