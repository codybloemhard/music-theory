use super::{ Note, Notes, Scale, Steps, scale_iter };
use super::interval::*;
use super::traits::{
    VecWrapper, Wrapper, ToNamedInterval, AsScale, ToPC, ToRootedChord, ToChord
};
use super::super::utils::{ is_sorted };

use std::collections::HashSet;

pub const NUM_SUPS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
pub const NUM_SUBS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

// base strings: 0 none, 1 major, 2 minor, 3 aug, 4 dim
pub const BASE_LONG: [&str; 5] = ["", "maj", "min", "aug", "dim"];
pub const BASE_SHORT: [&str; 5] = ["", "M", "m", "aug", "dim"];
pub const BASE_SYM: [&str; 5] = ["", "Δ", "-", "+", "°"];
pub const BASES: [[&str; 5]; 3] = [BASE_LONG, BASE_SHORT, BASE_SYM];

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
pub const MINOR_NINTH_CHORD: &[Note] = &[_MIN3, _PER5, _MIN7, _MAJ9];
pub const DOMINANT_NINTH_CHORD: &[Note] = &[_MAJ3, _PER5, _MIN7, _MAJ9];
pub const MAJOR_ELEVENTH_CHORD: &[Note] = &[_MAJ3, _PER5, _MAJ7, _MAJ9, _MAJ11];
pub const MINOR_ELEVENTH_CHORD: &[Note] = &[_MIN3, _PER5, _MIN7, _MAJ9, _MAJ11];
pub const DOMINANT_ELEVENTH_CHORD: &[Note] = &[_MAJ3, _PER5, _MIN7, _MAJ9, _MAJ11];
pub const MAJOR_THIRTEENTH_CHORD: &[Note] = &[_MAJ3, _PER5, _MAJ7, _MAJ9, _MAJ11, _MAJ13];
pub const MINOR_THIRTEENTH_CHORD: &[Note] = &[_MIN3, _PER5, _MIN7, _MAJ9, _MAJ11, _MAJ13];
pub const DOMINANT_THIRTEENTH_CHORD: &[Note] = &[_MAJ3, _PER5, _MIN7, _MAJ9, _MAJ11, _MAJ13];

// (pattern, name, base string, extended collection?)
pub type ChordBook = &'static [(&'static [Note], &'static str, usize, bool)];

pub const STD_CHORD_BOOK: ChordBook = &[
    (MAJOR, "", 1, false),
    (MINOR, "", 2, false),
    (MINOR_AUGMENTED, "", 23, true),
    (MAJOR_AUGMENTED, "", 3, false),
    (MINOR_DIMINISHED, "", 4, false),
    (MAJOR_DIMINISHED, "", 14, true),
    (SUS2, "sus2", 0, false),
    (SUS4, "sus4", 0, false),
    (SUPER_SUS, "ssus", 0, true),
    (PHRYGIAN, "phry", 0, false),
    (LYDIAN, "lyd", 0, false),
    (LOCRIAN2, "loc2", 0, false),
    (LOCRIAN4, "loc4", 0, false),
    (SUPER_LOCRIAN, "sloc", 0, true),
    (MAJOR_SIXTH_CHORD, "6", 1, false),
    (MINOR_SIXTH_CHORD, "6", 2, false),
    (MAJOR_SEVENTH_CHORD, "7", 1, false),
    (MINOR_SEVENTH_CHORD, "7", 2, false),
    (DOMINANT_SEVENTH, "7", 0, false),
    (MINOR_MAJOR_SEVENTH, "", 21, false),
    (HALF_DIMINISHED_SEVENTH, "ø", 0, false),
    (DIMINISHED_SEVENTH_CHORD, "7", 4, false),
    (AUGMENTED_SEVENTH_CHORD, "7", 3, false),
    (MU_CHORD, "μ", 0, true),
    (SIX_NINE_CHORD, "6/9", 0, false),
    (MAJOR_NINTH_CHORD, "9", 1, false),
    (MINOR_NINTH_CHORD, "9", 2, false),
    (DOMINANT_NINTH_CHORD, "9", 0, false),
    (MAJOR_ELEVENTH_CHORD, "11", 1, false),
    (MINOR_ELEVENTH_CHORD, "11", 2, false),
    (DOMINANT_ELEVENTH_CHORD, "11", 0, false),
    (MAJOR_THIRTEENTH_CHORD, "13", 1, false),
    (MINOR_THIRTEENTH_CHORD, "13", 2, false),
    (DOMINANT_THIRTEENTH_CHORD, "13", 0, false),
];

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Chord(pub Vec<Note>);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RootedChord{
    pub root: Note,
    pub chord: Chord,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelativeChord{
    pub degree: ScaleDegree,
    pub chord: Chord,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScaleDegree{
    I, bII, II, bIII, III, IV, bV, V, bVI, VI, bVII, VII
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum MStyle{ Long = 0, Short = 1, Symbol = 2 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum EStyle{ Long = 0, Symbol = 2 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChordStyle{
    Std(MStyle, EStyle),
    Extra(MStyle, EStyle),
    Spelled,
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

    pub fn quality(&self, basestr: String, style: ChordStyle) -> String{
        fn spelled_out(mut basestr: String, notes: &[Note]) -> String{
            basestr.push('[');
            for int in notes{ // When spelling out literally, we want to spell a 9th at 9 not as 2
                basestr.push_str(&Interval(int.0 as i32).to_named_interval_mod().to_string());
            }
            basestr.push(']');
            basestr
        }
        let (mstyle, estyle, extra) = match style{
            ChordStyle::Spelled => return spelled_out(basestr, &self.0),
            ChordStyle::Std(ms, es) => (ms, es, false),
            ChordStyle::Extra(ms, es) => (ms, es, true),
        };
        let chord = self.clone().normalized();
        let sname = |mut bstr: String, bq| {
            let (bqa, bqb) = if bq < 10 { (bq, 0) } else { (bq % 10, bq / 10) };
            let basecat = |bq| if bq == 1 || bq == 2 { mstyle as usize } else { estyle as usize };
            bstr.push_str(BASES[basecat(bqb)][bqb]);
            bstr.push_str(BASES[basecat(bqa)][bqa]);
            bstr
        };
        // find longest pattern of which all intervals are in the chord
        let per5 = chord.contains(&_PER5);
        let mut pat = (vec![], 0, "", false);
        'outer: for (pattern, postfix, base, ext) in STD_CHORD_BOOK{
            if *ext && !extra { continue; }
            let pattern = Chord(pattern.to_vec()).normalized().0;
            if pattern == chord.0 { // exact match
                let mut name = sname(basestr, *base);
                name.push_str(postfix);
                return name;
            }
            let mut nothird = false;
            for int in &pattern{
                let isin = chord.contains(int);
                if isin { continue; }
                let patnotsus = !(pattern.contains(&_MAJ2) || pattern.contains(&_PER4));
                if (*int == _MIN3 || *int == _MAJ3) && per5 && patnotsus && !nothird{
                    nothird = true;
                    continue;
                }
                continue 'outer;
            }
            if pattern.len() <= pat.0.len() { continue; }
            pat = (pattern, *base, postfix, nothird);
        }
        let (pat, base, postfix, nothird) = pat;
        if !pat.is_empty(){ // found an usable base chord to extend
            let mut name = sname(basestr, base);
            name.push_str(postfix);
            let sus = if nothird{ // kinda sus brø
                let sus2 = chord.contains(&_MAJ2);
                let sus4 = chord.contains(&_PER4);
                if sus2 && sus4 && extra { name.push_str("ssus"); 10 }
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

    pub fn as_string(&self, style: ChordStyle) -> String{
        self.quality("X".to_string(), style)
    }
}

impl RootedChord{
    pub fn new(root: Note, intervals: &[Note]) -> Self{
        Self{ root, chord: Chord::new(intervals) }
    }

    pub fn from_chord(root: Note, chord: Chord) -> Self{
        Self{ root, chord }
    }

    pub fn as_scale(&self) -> Scale{
        let mut scale = vec![self.root];
        for int in &self.chord.0{
            scale.push(self.root + *int);
        }
        Scale(scale)
    }

    pub fn to_scale(self) -> Scale{
        self.as_scale()
    }

    pub fn normalized(self) -> Self{
        Self {
            root: self.root % _OCTAVE,
            chord: self.chord.normalized(),
        }
    }

    pub fn as_chordtone_wholetone_scale(&self) -> Option<Scale>{
        let mut res = Vec::new();
        let scale = self.as_scale();
        if scale.len() < 4 { return None; }
        for (i, note) in scale.iter().enumerate().take(4){
            res.push(*note);
            if i >= 3 { continue; }
            let between = if scale.len() > i + 4 { scale.0[i + 4].0 - _OCTAVE.0 }
            else { note.0 + _MAJ2.0 };
            res.push(Note(between));
        }
        Some(Scale(res))
    }

    pub fn as_inversion(&self) -> Self{
        let mut scale = self.as_scale();
        if scale.is_empty() { return Self::default(); }
        let mut root = scale.0[0];
        if scale.len() == 1 { return Self::new(root, &[]); }
        let top = scale.0[scale.len() - 1];
        while root < top {
            root += _OCTAVE;
        }
        scale.0.remove(0);
        scale.0.push(root);
        scale.to_rooted_chord()
    }

    pub fn to_inversion(self) -> Self{
        self.as_inversion()
    }

    pub fn as_all_inversions(&self) -> Vec<Self>{
        let len = self.chord.len() + 1;
        let mut inv = self.clone();
        let mut res = Vec::new();
        for _ in 0..len{
            inv = inv.as_inversion();
            res.push(inv.clone());
        }
        res
    }

    pub fn to_all_inversions(self) -> Vec<Self>{
        self.as_all_inversions()
    }

    pub fn as_string(&self, style: ChordStyle) -> String{
        let root = self.root.to_pc().to_string();
        self.chord.quality(root, style)
    }
}

impl AsScale for Chord{
    fn as_scale(&self, root: Note) -> Scale{
        let mut scale = vec![root];
        for int in &self.0{
            scale.push(root + *int);
        }
        Scale(scale)
    }
}

impl std::fmt::Display for ScaleDegree{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let res = match self{
            Self::I    => "I",
            Self::bII  => "bII",
            Self::II   => "II",
            Self::bIII => "bIII",
            Self::III  => "III",
            Self::IV   => "IV",
            Self::bV   => "bV",
            Self::V    => "V",
            Self::bVI  => "bVI",
            Self::VI   => "VI",
            Self::bVII => "bVII",
            Self::VII  => "VII",
        };
        write!(f, "{}", res)
    }
}

impl RelativeChord{
    pub fn new(degree: ScaleDegree, intervals: &[Note]) -> Self{
        Self{ degree, chord: Chord::new(intervals) }
    }

    pub fn from_chord(degree: ScaleDegree, chord: Chord) -> Self{
        Self{ degree, chord }
    }

    pub fn as_string(&self, style: ChordStyle) -> String{
        self.chord.quality(self.degree.to_string(), style)
    }
}

impl std::fmt::Display for RelativeChord{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let ext = ChordStyle::Extra(MStyle::Symbol, EStyle::Symbol);
        let res = self.chord.quality(self.degree.to_string(), ext);
        write!(f, "{}", res)
    }
}

pub fn scale_chords(steps: &Steps, chord_size: usize) -> Vec<Chord>{
    let len = steps.len();
    let mut chords = Vec::new();
    for (i, _) in scale_iter(Note::ZERO, &steps.0).enumerate().take(len){
        let mut chord = Vec::new();
        for note in scale_iter(Note::ZERO, &steps.0).skip(i).step_by(2).take(chord_size){
            chord.push(note);
        }
        chords.push(Scale(chord).to_chord());
    }
    chords
}

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
    use super::super::*;

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
        let spl = ChordStyle::Spelled;
        let std = ChordStyle::Std(MStyle::Symbol, EStyle::Symbol);
        let ext = ChordStyle::Extra(MStyle::Symbol, EStyle::Symbol);
        let letr = ChordStyle::Extra(MStyle::Short, EStyle::Symbol);
        let long = ChordStyle::Extra(MStyle::Long, EStyle::Symbol);
        let verbose = ChordStyle::Extra(MStyle::Long, EStyle::Long);

        assert_eq!(&Chord::new(&[_MIN3]).as_string(std), "X[♭3]");
        assert_eq!(&Chord::new(&[_PER5]).as_string(std), "XΔno3");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5]).as_string(spl), "X[♮3♮5]");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5]).as_string(std), "XΔ");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5]).as_string(ext), "XΔ");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5]).as_string(letr), "XM");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5]).as_string(long), "Xmaj");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5]).as_string(verbose), "Xmaj");

        assert_eq!(&Chord::new(&[_MIN3,_PER5]).as_string(std), "X-");
        assert_eq!(&Chord::new(&[_MIN3,_PER5]).as_string(ext), "X-");
        assert_eq!(&Chord::new(&[_MIN3,_PER5]).as_string(letr), "Xm");
        assert_eq!(&Chord::new(&[_MIN3,_PER5]).as_string(long), "Xmin");
        assert_eq!(&Chord::new(&[_MIN3,_PER5]).as_string(verbose), "Xmin");

        assert_eq!(&Chord::new(&[_MIN3,_DIM5]).as_string(std), "X°");
        assert_eq!(&Chord::new(&[_MIN3,_DIM5]).as_string(ext), "X°");
        assert_eq!(&Chord::new(&[_MIN3,_DIM5]).as_string(letr), "X°");
        assert_eq!(&Chord::new(&[_MIN3,_DIM5]).as_string(long), "X°");
        assert_eq!(&Chord::new(&[_MIN3,_DIM5]).as_string(verbose), "Xdim");

        assert_eq!(&Chord::new(&[_MAJ3,_DIM5]).as_string(std), "X[♮3♭5]");
        assert_eq!(&Chord::new(&[_MAJ3,_DIM5]).as_string(ext), "XΔ°");
        assert_eq!(&Chord::new(&[_MAJ3,_DIM5]).as_string(letr), "XM°");
        assert_eq!(&Chord::new(&[_MAJ3,_DIM5]).as_string(long), "Xmaj°");
        assert_eq!(&Chord::new(&[_MAJ3,_DIM5]).as_string(verbose), "Xmajdim");

        assert_eq!(&Chord::new(&[_MAJ2,_PER5]).as_string(std), "Xsus2");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5]).as_string(ext), "Xsus2");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5]).as_string(letr), "Xsus2");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5]).as_string(long), "Xsus2");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5]).as_string(verbose), "Xsus2");

        assert_eq!(&Chord::new(&[_PER4,_PER5]).as_string(std), "Xsus4");
        assert_eq!(&Chord::new(&[_PER4,_PER5]).as_string(ext), "Xsus4");
        assert_eq!(&Chord::new(&[_MAJ3,_AUG5]).as_string(std), "X+");
        assert_eq!(&Chord::new(&[_MAJ3,_AUG5]).as_string(ext), "X+");
        assert_eq!(&Chord::new(&[_MIN3,_AUG5]).as_string(std), "X[♭3♭6]");
        assert_eq!(&Chord::new(&[_MIN3,_AUG5]).as_string(ext), "X-+");
        assert_eq!(&Chord::new(&[_MAJ2,_PER4]).as_string(std), "X[♮2♮4]");
        assert_eq!(&Chord::new(&[_MAJ2,_PER4]).as_string(ext), "Xssus");
        assert_eq!(&Chord::new(&[_MIN2,_PER5]).as_string(std), "Xphry");
        assert_eq!(&Chord::new(&[_MIN2,_PER5]).as_string(ext), "Xphry");
        assert_eq!(&Chord::new(&[_AUG4,_PER5]).as_string(std), "Xlyd");
        assert_eq!(&Chord::new(&[_AUG4,_PER5]).as_string(ext), "Xlyd");
        assert_eq!(&Chord::new(&[_MIN2,_DIM5]).as_string(std), "Xloc2");
        assert_eq!(&Chord::new(&[_MIN2,_DIM5]).as_string(ext), "Xloc2");
        assert_eq!(&Chord::new(&[_PER4,_DIM5]).as_string(std), "Xloc4");
        assert_eq!(&Chord::new(&[_PER4,_DIM5]).as_string(ext), "Xloc4");
        assert_eq!(&Chord::new(&[_MIN2,_PER4,_DIM5]).as_string(std), "Xloc2(♮11)");
        assert_eq!(&Chord::new(&[_MIN2,_PER4,_DIM5]).as_string(ext), "Xsloc");

        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ6]).as_string(std), "XΔ6");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ6]).as_string(ext), "XΔ6");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ6]).as_string(letr), "XM6");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ6]).as_string(long), "Xmaj6");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ6]).as_string(verbose), "Xmaj6");

        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MAJ6]).as_string(std), "X-6");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MAJ6]).as_string(ext), "X-6");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ7]).as_string(std), "XΔ7");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ7]).as_string(ext), "XΔ7");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MIN7]).as_string(std), "X-7");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MIN7]).as_string(ext), "X-7");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MIN7]).as_string(std), "X7");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MIN7]).as_string(ext), "X7");

        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MAJ7]).as_string(std), "X-Δ");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MAJ7]).as_string(ext), "X-Δ");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MAJ7]).as_string(letr), "XmM");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MAJ7]).as_string(long), "Xminmaj");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MAJ7]).as_string(verbose), "Xminmaj");

        assert_eq!(&Chord::new(&[_MIN3,_DIM5,_MIN7]).as_string(std), "Xø");
        assert_eq!(&Chord::new(&[_MIN3,_DIM5,_MIN7]).as_string(ext), "Xø");
        assert_eq!(&Chord::new(&[_MIN3,_DIM5,_DIM7]).as_string(std), "X°7");
        assert_eq!(&Chord::new(&[_MIN3,_DIM5,_DIM7]).as_string(ext), "X°7");
        assert_eq!(&Chord::new(&[_MAJ3,_AUG5,_MIN7]).as_string(std), "X+7");
        assert_eq!(&Chord::new(&[_MAJ3,_AUG5,_MIN7]).as_string(ext), "X+7");
        assert_eq!(&Chord::new(&[_MAJ2,_MAJ3,_PER5]).as_string(std), "XΔ(♮9)");
        assert_eq!(&Chord::new(&[_MAJ2,_MAJ3,_PER5]).as_string(ext), "Xμ");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ6,_MAJ9]).as_string(std), "X6/9");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ6,_MAJ9]).as_string(ext), "X6/9");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MIN7,_MAJ9]).as_string(std), "X-9");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MIN7,_MAJ9]).as_string(ext), "X-9");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ7,_MAJ9]).as_string(std), "XΔ9");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ7,_MAJ9]).as_string(ext), "XΔ9");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MIN7,_MAJ9]).as_string(std), "X9");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MIN7,_MAJ9]).as_string(ext), "X9");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MIN7,_MAJ9,_MAJ11]).as_string(std), "X-11");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MIN7,_MAJ9,_MAJ11]).as_string(ext), "X-11");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ7,_MAJ9,_MAJ11]).as_string(std), "XΔ11");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ7,_MAJ9,_MAJ11]).as_string(ext), "XΔ11");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MIN7,_MAJ9,_MAJ11]).as_string(std), "X11");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MIN7,_MAJ9,_MAJ11]).as_string(ext), "X11");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MIN7,_MAJ9,_MAJ11,_MAJ13]).as_string(std), "X-13");
        assert_eq!(&Chord::new(&[_MIN3,_PER5,_MIN7,_MAJ9,_MAJ11,_MAJ13]).as_string(ext), "X-13");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ7,_MAJ9,_MAJ11,_MAJ13]).as_string(std), "XΔ13");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MAJ7,_MAJ9,_MAJ11,_MAJ13]).as_string(ext), "XΔ13");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MIN7,_MAJ9,_MAJ11,_MAJ13]).as_string(std), "X13");
        assert_eq!(&Chord::new(&[_MAJ3,_PER5,_MIN7,_MAJ9,_MAJ11,_MAJ13]).as_string(ext), "X13");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MAJ6,_MAJ9]).as_string(std), "XΔ6sus2");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MAJ6,_MAJ9]).as_string(ext), "XΔ6sus2");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MAJ6,_MAJ9]).as_string(std), "XΔ6sus2(♮11)");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MAJ6,_MAJ9]).as_string(ext), "XΔ6ssus");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MIN7,_MAJ9]).as_string(std), "X-7sus2");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MIN7,_MAJ9]).as_string(ext), "X-7sus2");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MAJ7,_MAJ9]).as_string(std), "XΔ7sus2");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MAJ7,_MAJ9]).as_string(ext), "XΔ7sus2");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MAJ7,_MAJ9]).as_string(std), "XΔ7sus2(♮11)");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MAJ7,_MAJ9]).as_string(ext), "XΔ7ssus");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MIN7,_MAJ9]).as_string(std), "X-7sus2");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MIN7,_MAJ9]).as_string(ext), "X-7sus2");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MIN7,_MAJ9]).as_string(std), "X-7sus2(♮11)");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MIN7,_MAJ9]).as_string(ext), "X-7ssus");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MIN7,_MAJ9,_MAJ11]).as_string(std), "X-7sus2(♮11)");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MIN7,_MAJ9,_MAJ11]).as_string(ext), "X-7ssus");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MIN7,_MAJ9,_MAJ11]).as_string(std), "X-7sus2(♮11)");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MIN7,_MAJ9,_MAJ11]).as_string(ext), "X-7ssus");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MAJ7,_MAJ9,_MAJ11]).as_string(std), "XΔ7sus2(♮11)");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MAJ7,_MAJ9,_MAJ11]).as_string(ext), "XΔ7ssus");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MAJ7,_MAJ9,_MAJ11]).as_string(std), "XΔ7sus2(♮11)");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MAJ7,_MAJ9,_MAJ11]).as_string(ext), "XΔ7ssus");

        // assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MIN7,_MAJ9,_MAJ11,_MAJ13]).as_string(std), "x13sus2");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MIN7,_MAJ9,_MAJ11,_MAJ13]).as_string(ext), "XΔ6ssus");
        // assert_eq!(&Chord::new(&[_PER4,_PER5,_MIN7,_MAJ9,_MAJ11,_MAJ13]).as_string(std), "x13sus4");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MIN7,_MAJ9,_MAJ11,_MAJ13]).as_string(ext), "XΔ6ssus");

        // assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MAJ7,_MAJ9,_MAJ11,_MAJ13]).as_string(std), "X13");
        // assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MAJ7,_MAJ9,_MAJ11,_MAJ13]).as_string(ext), "X13");
        // assert_eq!(&Chord::new(&[_PER4,_PER5,_MAJ7,_MAJ9,_MAJ11,_MAJ13]).as_string(std), "X13");

        assert_eq!(&Chord::new(&[_PER4,_PER5,_MIN7,_AUG9,_AUG13]).as_string(std), "X-7(♮11)");
        assert_eq!(&Chord::new(&[_PER4,_PER5,_MIN7,_AUG9,_AUG13]).as_string(ext), "X-7(♮11)");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MAJ7,_MIN9,_AUG11]).as_string(std), "XΔ7sus2(♭9♯11)");
        assert_eq!(&Chord::new(&[_MAJ2,_PER5,_MAJ7,_MIN9,_AUG11]).as_string(ext), "XΔ7sus2(♭9♯11)");
        // assert_eq!(&Chord::new(&[_PER4,_PER5,_MIN7,_MIN9,_MIN11]).as_string(std), "x11sus4");
        // assert_eq!(&Chord::new(&[_PER4,_PER5,_MIN7,_MIN9,_MIN11]).as_string(ext), "x11sus4");
    }

    #[test]
    fn chord_as_scale(){
        assert_eq!(
            Chord(vec![]).to_scale(Note::F1),
            Scale(vec![Note::F1])
        );
        assert_eq!(
            Chord::new(&MAJOR_SEVENTH_CHORD).to_scale(Note::F1),
            Scale(vec![Note::F1, Note::A2, Note::C2, Note::E2])
        );
    }

    #[test]
    fn rooted_chord_new(){
        assert_eq!(
            RootedChord::new(Note::A4, &MAJOR_SIXTH_CHORD),
            RootedChord{ root: Note::A4, chord: Chord(vec![_MAJ3, _PER5, _MAJ6]) }
        );
    }

    #[test]
    fn rooted_chord_from_chord(){
        assert_eq!(
            RootedChord::from_chord(Note::A4, Chord::new(&MU_CHORD)),
            RootedChord{ root: Note::A4, chord: Chord(vec![_MAJ2, _MAJ3, _PER5]) }
        );
    }

    #[test]
    fn rooted_chord_as_scale(){
        assert_eq!(
            RootedChord{ root: Note::A4, chord: Chord(vec![]) }.to_scale(),
            Scale(vec![Note::A4])
        );
        assert_eq!(
            RootedChord{ root: Note::A1, chord: Chord::new(&MAJOR) }.to_scale(),
            Scale(vec![Note::A1, Note::CS1, Note::E1])
        );
    }

    #[test]
    fn rooted_chord_normalized(){
        assert_eq!(
            RootedChord::new(Note::A1, &[_MAJ3, _PER5, _OCTAVE, _MAJ9, _PER12]).normalized(),
            RootedChord::new(Note::ZERO, &[_MAJ2, _MAJ3, _PER5])
        );
    }

    #[test]
    fn rooted_chord_as_chordtone_wholetone_scale(){
        assert_eq!(RootedChord::new(Note::F1, &MAJOR).as_chordtone_wholetone_scale(), None);
        assert_eq!(
            RootedChord::new(Note::F1, &MAJOR_SEVENTH_CHORD).as_chordtone_wholetone_scale(),
            Some(Scale(vec![Note::F1, Note::G1, Note::A2, Note::B2, Note::C2, Note::D2, Note::E2]))
        );
        assert_eq!(
            RootedChord::new(Note::A1, &MINOR_SEVENTH_CHORD).as_chordtone_wholetone_scale(),
            Some(Scale(vec![Note::A1, Note::B1, Note::C1, Note::D1, Note::E1, Note::FS1, Note::G1]))
        );
    }

    #[test]
    fn rooted_chord_as_inversion(){
        assert_eq!(
            Scale(vec![Note::A1, Note::C1, Note::E1, Note::G1]).to_rooted_chord().to_inversion(),
            Scale(vec![Note::C1, Note::E1, Note::G1, Note::A2]).to_rooted_chord(),
        );
        assert_eq!(
            Scale(vec![Note::C1, Note::E1, Note::G1, Note::A2]).to_rooted_chord().to_inversion(),
            Scale(vec![Note::E1, Note::G1, Note::A1, Note::C2]).to_rooted_chord(),
        );
        assert_eq!(
            Scale(vec![Note::E1, Note::G1, Note::A1, Note::C2]).to_rooted_chord().to_inversion(),
            Scale(vec![Note::G1, Note::A2, Note::C2, Note::E2]).to_rooted_chord(),
        );
        assert_eq!(
            Scale(vec![Note::G1, Note::A2, Note::C2, Note::E2]).to_rooted_chord().to_inversion(),
            Scale(vec![Note::A2, Note::C2, Note::E2, Note::G2]).to_rooted_chord(),
        );
    }

    #[test]
    fn rooted_chord_as_all_inversions(){
        assert_eq!(
            Scale(vec![Note::A1, Note::C1, Note::E1, Note::G1]).to_rooted_chord().to_all_inversions(),
            vec![
                Scale(vec![Note::C1, Note::E1, Note::G1, Note::A2]).to_rooted_chord(),
                Scale(vec![Note::E1, Note::G1, Note::A1, Note::C2]).to_rooted_chord(),
                Scale(vec![Note::G1, Note::A2, Note::C2, Note::E2]).to_rooted_chord(),
                Scale(vec![Note::A2, Note::C2, Note::E2, Note::G2]).to_rooted_chord(),
            ]
        );
    }

    #[test]
    fn rooted_chord_as_string(){
        let std = ChordStyle::Std(MStyle::Symbol, EStyle::Symbol);
        assert_eq!(
            &RootedChord::new(Note::CS1, &[_PER4,_PER5,_MIN7,_AUG9,_AUG13]).as_string(std),
            "C♯-7(♮11)"
        );
        assert_eq!(
            &RootedChord::new(Note::CS1, &[_MAJ2,_PER5,_MAJ7,_MIN9,_AUG11]).as_string(std),
            "C♯Δ7sus2(♭9♯11)"
        );
    }

    #[test]
    fn scale_degree_to_string(){
        assert_eq!(&ScaleDegree::I.to_string(), "I");
        assert_eq!(&ScaleDegree::bII.to_string(), "bII");
        assert_eq!(&ScaleDegree::II.to_string(), "II");
        assert_eq!(&ScaleDegree::bIII.to_string(), "bIII");
        assert_eq!(&ScaleDegree::III.to_string(), "III");
        assert_eq!(&ScaleDegree::IV.to_string(), "IV");
        assert_eq!(&ScaleDegree::bV.to_string(), "bV");
        assert_eq!(&ScaleDegree::V.to_string(), "V");
        assert_eq!(&ScaleDegree::bVI.to_string(), "bVI");
        assert_eq!(&ScaleDegree::VI.to_string(), "VI");
        assert_eq!(&ScaleDegree::bVII.to_string(), "bVII");
        assert_eq!(&ScaleDegree::VII.to_string(), "VII");
    }

    #[test]
    fn relative_chord_new(){
        assert_eq!(
            RelativeChord::new(ScaleDegree::I, &MAJOR),
            RelativeChord{ degree: ScaleDegree::I, chord: Chord::new(&MAJOR) }
        );
    }

    #[test]
    fn relative_chord_from_chord(){
        let chord = Chord::new(&MINOR);
        assert_eq!(
            RelativeChord::from_chord(ScaleDegree::bV, chord.clone()),
            RelativeChord{ degree: ScaleDegree::bV, chord: chord }
        );
    }

    #[test]
    fn relative_chord_as_string(){
        let long = ChordStyle::Std(MStyle::Long, EStyle::Long);
        assert_eq!(
            &RelativeChord::new(ScaleDegree::bVII, &MAJOR_ELEVENTH_CHORD).as_string(long),
            "bVIImaj11"
        );
    }

    #[test]
    fn relative_chord_to_string(){
        assert_eq!(
            &RelativeChord::new(ScaleDegree::II, &MINOR_NINTH_CHORD).to_string(),
            "II-9"
        );
    }

    #[test]
    fn test_scale_chords(){
        let chords = scale_chords(&crate::libr::ionian::steps(), 3);
        assert_eq!(
            chords,
            vec![
                Chord::new(&MAJOR),
                Chord::new(&MINOR),
                Chord::new(&MINOR),
                Chord::new(&MAJOR),
                Chord::new(&MAJOR),
                Chord::new(&MINOR),
                Chord::new(&MINOR_DIMINISHED)
            ]
        );
    }
}
