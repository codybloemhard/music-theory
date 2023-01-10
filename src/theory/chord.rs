use crate::{
    utils::is_sorted,
    theory::{
        interval::{ *, note_interval::* },
        traits::{
            VecWrapper, Wrapper, ToNamedInterval, AsScale, ToNote, ToPC, ToRootedChord, AsSubs
        },
        Note, Notes, Scale, PC
    },
};

use itertools::*;

const _NUM_SUPS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
const _NUM_SUBS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

// base strings: 0 none, 1 major, 2 minor, 3 aug, 4 dim
const BASE_LONG: [&str; 5] = ["", "maj", "min", "aug", "dim"];
const BASE_SHORT: [&str; 5] = ["", "M", "m", "aug", "dim"];
const BASE_SYM: [&str; 5] = ["", "Δ", "-", "+", "°"];
const BASES: [[&str; 5]; 3] = [BASE_LONG, BASE_SHORT, BASE_SYM];

macro_rules! dcc{
    ($id: ident, $val: expr) => {
        #[allow(missing_docs)] pub const $id: &[Note] = $val;
    }
}

dcc!(POWER, &[PER5]);
dcc!(MAJOR, &[MAJ3, PER5]);
dcc!(MINOR, &[MIN3, PER5]);
dcc!(MINOR_AUGMENTED, &[MIN3, AUG5]);
dcc!(MAJOR_AUGMENTED, &[MAJ3, AUG5]);
dcc!(MINOR_DIMINISHED, &[MIN3, DIM5]);
dcc!(MAJOR_DIMINISHED, &[MAJ3, DIM5]);
dcc!(SUS2, &[MAJ2, PER5]);
dcc!(SUS4, &[PER4, PER5]);
dcc!(SUPER_SUS, &[MAJ2, PER4]);
dcc!(PHRYGIAN, &[MIN2, PER5]);
dcc!(LYDIAN, &[AUG4, PER5]);
dcc!(LOCRIAN2, &[MIN2, DIM5]);
dcc!(LOCRIAN4, &[PER4, DIM5]);
dcc!(SUPER_LOCRIAN, &[MIN2, PER4, DIM5]);
dcc!(MAJOR_SIXTH_CHORD, &[MAJ3, PER5, MAJ6]);
dcc!(MINOR_SIXTH_CHORD, &[MIN3, PER5, MAJ6]);
dcc!(MAJOR_SEVENTH_CHORD, &[MAJ3, PER5, MAJ7]);
dcc!(MINOR_SEVENTH_CHORD, &[MIN3, PER5, MIN7]);
dcc!(DOMINANT_SEVENTH, &[MAJ3, PER5, MIN7]);
dcc!(MINOR_MAJOR_SEVENTH, &[MIN3, PER5, MAJ7]);
dcc!(HALF_DIMINISHED_SEVENTH, &[MIN3, DIM5, MIN7]);
dcc!(DIMINISHED_SEVENTH_CHORD, &[MIN3, DIM5, DIM7]);
dcc!(AUGMENTED_SEVENTH_CHORD, &[MAJ3, AUG5, MIN7]);
dcc!(MU_CHORD, &[MAJ2, MAJ3, PER5]);
dcc!(SIX_NINE_CHORD, &[MAJ3, PER5, MAJ6, MAJ9]);
dcc!(MAJOR_NINTH_CHORD, &[MAJ3, PER5, MAJ7, MAJ9]);
dcc!(MINOR_NINTH_CHORD, &[MIN3, PER5, MIN7, MAJ9]);
dcc!(DOMINANT_NINTH_CHORD, &[MAJ3, PER5, MIN7, MAJ9]);
dcc!(MAJOR_ELEVENTH_CHORD, &[MAJ3, PER5, MAJ7, MAJ9, MAJ11]);
dcc!(MINOR_ELEVENTH_CHORD, &[MIN3, PER5, MIN7, MAJ9, MAJ11]);
dcc!(DOMINANT_ELEVENTH_CHORD, &[MAJ3, PER5, MIN7, MAJ9, MAJ11]);
dcc!(MAJOR_THIRTEENTH_CHORD, &[MAJ3, PER5, MAJ7, MAJ9, MAJ11, MAJ13]);
dcc!(MINOR_THIRTEENTH_CHORD, &[MIN3, PER5, MIN7, MAJ9, MAJ11, MAJ13]);
dcc!(DOMINANT_THIRTEENTH_CHORD, &[MAJ3, PER5, MIN7, MAJ9, MAJ11, MAJ13]);

// (pattern, name, base string, extended collection?)
type ChordBook = &'static [(&'static [Note], &'static str, usize, bool)];

const STD_CHORD_BOOK: ChordBook = &[
    (POWER, "power", 0, false),
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

/// A `Chord` only encodes the quality of the the chord.
/// No root from which it is build is defined.
///
/// Example:
/// ```
/// use music_theory::theory::*;
/// let chord = Chord::new(&MAJOR);
/// assert!(chord.contains_all(&POWER));
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Chord(pub Vec<Note>);

/// Wrapper around [Chord][crate::theory::chord::Chord].
/// Also has a root note and extra functionality around that.
///
/// Example:
/// ```
/// use music_theory::theory::*;
/// let chord = RootedChord::new(Note::C2, &MAJOR);
/// assert_eq!(chord.to_scale().unwrap(), vec![Note::C2, Note::E2, Note::G2]);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RootedChord{
    /// The root note from which the chord is build.
    pub root: Note,
    /// The quality of the chord.
    pub chord: Chord,
}

/// Relative chord is a chord build not from a specific root note but a scale degree.
/// Could be used for example to define chord progressions (eg. II -> V -> I).
///
/// Example:
/// ```
/// use music_theory::theory::*;
/// assert_eq!(
///     RelativeChord::new(ScaleDegree::I, MAJOR),
///     RelativeChord{ degree: ScaleDegree::I, chord: Chord::new(MAJOR) }
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelativeChord{
    /// The degree from which the chord is buld.
    pub degree: ScaleDegree,
    /// The quality of the chord.
    pub chord: Chord,
}

/// Scale degree, one of twelve variants for each chromatic note.
///
/// Example:
/// ```
/// use music_theory::theory::*;
/// assert_eq!(
///     RelativeChord::new(ScaleDegree::I, MAJOR),
///     RelativeChord{ degree: ScaleDegree::I, chord: Chord::new(MAJOR) }
/// );
/// ```
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScaleDegree{
    #[allow(missing_docs)] I,
    #[allow(missing_docs)] bII,
    #[allow(missing_docs)] II,
    #[allow(missing_docs)] bIII,
    #[allow(missing_docs)] III,
    #[allow(missing_docs)] IV,
    #[allow(missing_docs)] bV,
    #[allow(missing_docs)] V,
    #[allow(missing_docs)] bVI,
    #[allow(missing_docs)] VI,
    #[allow(missing_docs)] bVII,
    #[allow(missing_docs)] VII
}

/// The formatting style of the main base chord quality
///
/// Example:
/// ```
/// use music_theory::theory::*;
/// assert_eq!(MStyle::default(), MStyle::Symbol);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum MStyle{
    /// Use long qualities such as "maj", "min", "aug"
    Long = 0,
    /// Use short qualities such as "M", "m", "aug"
    Short = 1,
    /// Use symbols such as "Δ", "-", "+"
    #[default]
    Symbol = 2
}

/// The formatting style of the extra chord quality
///
/// Example:
/// ```
/// use music_theory::theory::*;
/// assert_eq!(EStyle::default(), EStyle::Symbol);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum EStyle{
    /// Use long qualities such as "maj", "min", "aug"
    Long = 0,
    /// Use symbols such as "Δ", "-", "+"
    #[default]
    Symbol = 2
}

/// Style that determines how a chord will be formatted or styled
///
/// Example:
/// ```
/// use music_theory::theory::*;
/// let std = ChordStyle::Std(MStyle::Symbol, EStyle::Symbol);
/// assert_eq!(&Chord::new(&MINOR_MAJOR_SEVENTH).as_string(std), "X-Δ");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChordStyle{
    /// Only use standard base chord variants
    Std(MStyle, EStyle),
    /// Use some extra base chord variants
    Extra(MStyle, EStyle),
    /// Spell out the chord in terms of intervals
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
    /// Create a chord from an array of unsigned note intervals.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// let chord = Chord::new(&MAJOR);
    /// assert!(chord.contains_all(&POWER));
    /// ```
    pub fn new(intervals: &[Note]) -> Self{
        let mut ints = intervals.to_owned();
        ints.sort();
        Chord(ints)
    }

    /// Returns true if the blueprint is the same as the inner wrapper value.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// let chord = Chord::new(&MAJOR);
    /// assert!(chord.same_intervals(&MAJOR));
    /// ```
    pub fn same_intervals(&self, blueprint: &[Note]) -> bool{
        self.0 == blueprint
    }

    /// Returns a normalised version of the chord.
    /// Intervals will be mapped into one octave (from Z to Z_12).
    /// The intervals will also be sorted duplicates will be removed.
    /// This means any form of voicing will be lost and the core quality will remain.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// use interval::note_interval as ni;
    /// assert_eq!(
    ///     Chord::new(&[ni::MAJ2, ni::MAJ3, ni::PER5, ni::MAJ7, ni::MAJ9, ni::PER5])
    ///         .normalized().unwrap(),
    ///     vec![ni::MAJ2, ni::MAJ3, ni::PER5, ni::MAJ7]
    /// );
    /// ```
    pub fn normalized(self) -> Self{
        let mut res = Vec::new();
        let mut grid = [false; 12];
        grid[0] = true;
        for note in self.0{
            let note = note % OCTAVE;
            if grid[note.0 as usize] { continue; }
            res.push(note);
            grid[note.0 as usize] = true;
        }
        res.sort();
        Chord(res)
    }

    /// Returns the quality of a chord as a string.
    /// Takes a [ChordStyle][crate::theory::chord::ChordStyle] to determine the formatting
    /// style.
    /// The base string is a prefix to the stringified quality.
    /// The algorithm works with normalised chords and does not take into account voicing, it aims
    /// to determine the core quality of the chord.
    /// It is not an exhaustive search style or optimalisation style algorithm and may not produce
    /// the very best quality you could assign to the input.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// let std = ChordStyle::Std(MStyle::Symbol, EStyle::Symbol);
    /// let base = String::from("BASESTRING");
    /// assert_eq!(&Chord::new(&MINOR_MAJOR_SEVENTH).quality(base, std), "BASESTRING-Δ");
    /// ```
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
        let per5 = chord.contains(&PER5);
        let has3 = chord.contains_any(&[MIN3, MAJ3]);
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
            let patsus = pattern.contains(&MAJ2) || pattern.contains(&PER4);
            for int in &pattern{
                if chord.contains(int) { continue; }
                if (*int == MIN3 || *int == MAJ3) && per5 && !patsus && !nothird && !has3{
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
                let sus2 = chord.contains(&MAJ2) && !pat.contains(&MAJ2);
                let sus4 = chord.contains(&PER4) && !pat.contains(&PER4);
                if sus2 && sus4 && extra { name.push_str("ssus"); 10 }
                else if sus2 { name.push_str("sus2"); 2 }
                else if sus4 { name.push_str("sus4"); 5 }
                else { name.push_str("no3"); 0 }
            } else { 0 };
            name.push('(');
            let mut atleastone = false;
            let highest = chord.iter().filter(|i| pat.contains(i)).max().unwrap_or(&Note::ZERO);
            for int in chord.iter(){
                if pat.contains(int) { continue; }
                if int.0 == sus || ((int.0 == 2 || int.0 == 5) && sus == 10) { continue; }
                let octave = if int.0 < highest.0 { 12 } else { 0 };
                name.push_str(&Interval(int.0 as i32 + octave).to_named_interval_mod().to_string());
                atleastone = true;
            }
            if atleastone { name.push(')'); } else { name.pop(); };
            name
        } else {
            spelled_out(basestr, &self.0)
        }
    }

    /// Stringify the chord
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// let std = ChordStyle::Std(MStyle::Symbol, EStyle::Symbol);
    /// assert_eq!(&Chord::new(&MINOR_MAJOR_SEVENTH).as_string(std), "X-Δ");
    /// ```
    pub fn as_string(&self, style: ChordStyle) -> String{
        self.quality("X".to_string(), style)
    }
}

impl RootedChord{
    /// Create an new `RootedChord` from a root note and some unsigned intervals.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// let chord = RootedChord::new(Note::C2, &MAJOR);
    /// assert_eq!(chord.to_scale().unwrap(), vec![Note::C2, Note::E2, Note::G2]);
    /// ```
    pub fn new(root: Note, intervals: &[Note]) -> Self{
        Self{ root, chord: Chord::new(intervals) }
    }

    /// Create an `RootedChord` from a root note and a chord.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// let chord = RootedChord::from_chord(Note::C2, Chord::new(&MAJOR));
    /// assert_eq!(chord.to_scale().unwrap(), vec![Note::C2, Note::E2, Note::G2]);
    /// ```
    pub fn from_chord(root: Note, chord: Chord) -> Self{
        Self{ root, chord }
    }

    /// Convert the `RootedChord` to a [Scale][crate::theory::scale::Scale].
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// let chord = RootedChord::new(Note::C2, &MAJOR);
    /// assert_eq!(chord.as_scale().unwrap(), vec![Note::C2, Note::E2, Note::G2]);
    /// ```
    pub fn as_scale(&self) -> Scale{
        let mut scale = vec![self.root];
        for int in &self.chord.0{
            scale.push(self.root + *int);
        }
        Scale(scale)
    }

    /// Ownership taking version of `as_scale`.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// let chord = RootedChord::new(Note::C2, &MAJOR);
    /// assert_eq!(chord.to_scale().unwrap(), vec![Note::C2, Note::E2, Note::G2]);
    /// ```
    pub fn to_scale(self) -> Scale{
        self.as_scale()
    }

    /// Returns a normalised version of the `RootedChord`.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// use interval::note_interval as ni;
    /// assert_eq!(
    ///     RootedChord::new(Note::C1, &[ni::MAJ2, ni::MAJ3, ni::PER5, ni::MAJ7, ni::MAJ9, ni::PER5])
    ///         .normalized(),
    ///     RootedChord{
    ///         root: Note::C0,
    ///         chord: Chord::new(&[ni::MAJ2, ni::MAJ3, ni::PER5, ni::MAJ7])
    ///     }
    /// );
    /// ```
    pub fn normalized(self) -> Self{
        Self {
            root: self.root % OCTAVE,
            chord: self.chord.normalized(),
        }
    }

    /// Attempts to find a chordtone wholetone scale for the `RootedChord`.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(
    ///     RootedChord::new(Note::F1, MAJOR_SEVENTH_CHORD).as_chordtone_wholetone_scale(),
    ///     Some(Scale::wrap(
    ///         vec![Note::F1, Note::G1, Note::A2, Note::B2, Note::C2, Note::D2, Note::E2]
    ///     )).unwrap()
    /// );
    /// ```
    pub fn as_chordtone_wholetone_scale(&self) -> Option<Scale>{
        let mut res = Vec::new();
        let scale = self.as_scale();
        if scale.len() < 4 { return None; }
        for (i, note) in scale.iter().enumerate().take(4){
            res.push(*note);
            if i >= 3 { continue; }
            let between = if scale.len() > i + 4 { scale[i + 4].0 - OCTAVE.0 }
            else { note.0 + MAJ2.0 };
            res.push(Note(between));
        }
        Some(Scale(res))
    }

    /// Returns a `RootedChord` that is the next inversion of the current one.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(
    ///     Scale::wrap(vec![Note::A1, Note::C1, Note::E1, Note::G1])
    ///         .unwrap().as_rooted_chord().to_inversion(),
    ///     Scale::wrap(vec![Note::C1, Note::E1, Note::G1, Note::A2])
    ///         .unwrap().as_rooted_chord(),
    /// );
    /// ```
    pub fn as_inversion(&self) -> Self{
        let mut scale = self.as_scale();
        if scale.is_empty() { return Self::default(); }
        let mut root = scale[0];
        if scale.len() == 1 { return Self::new(root, &[]); }
        let top = scale[scale.len() - 1];
        while root < top {
            root += OCTAVE;
        }
        scale.0.remove(0);
        scale.0.push(root);
        scale.to_rooted_chord()
    }

    /// Ownership taking version of `as_inversion`.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(
    ///     Scale::wrap(vec![Note::A1, Note::C1, Note::E1, Note::G1])
    ///         .unwrap().to_rooted_chord().to_inversion(),
    ///     Scale::wrap(vec![Note::C1, Note::E1, Note::G1, Note::A2])
    ///         .unwrap().to_rooted_chord(),
    /// );
    /// ```
    pub fn to_inversion(self) -> Self{
        self.as_inversion()
    }

    /// Returns a vector of all inversions of the `RootedChord`.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(
    ///     Scale::wrap(vec![Note::A1, Note::C1, Note::E1, Note::G1])
    ///         .unwrap().to_rooted_chord().as_all_inversions(),
    ///     vec![
    ///         Scale::wrap(vec![Note::C1, Note::E1, Note::G1, Note::A2]).unwrap().to_rooted_chord(),
    ///         Scale::wrap(vec![Note::E1, Note::G1, Note::A1, Note::C2]).unwrap().to_rooted_chord(),
    ///         Scale::wrap(vec![Note::G1, Note::A2, Note::C2, Note::E2]).unwrap().to_rooted_chord(),
    ///         Scale::wrap(vec![Note::A2, Note::C2, Note::E2, Note::G2]).unwrap().to_rooted_chord(),
    ///     ]
    /// );
    /// ```
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

    /// Ownership taking version of `as_all_inversions`.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(
    ///     Scale::wrap(vec![Note::A1, Note::C1, Note::E1, Note::G1])
    ///         .unwrap().to_rooted_chord().to_all_inversions(),
    ///     vec![
    ///         Scale::wrap(vec![Note::C1, Note::E1, Note::G1, Note::A2]).unwrap().to_rooted_chord(),
    ///         Scale::wrap(vec![Note::E1, Note::G1, Note::A1, Note::C2]).unwrap().to_rooted_chord(),
    ///         Scale::wrap(vec![Note::G1, Note::A2, Note::C2, Note::E2]).unwrap().to_rooted_chord(),
    ///         Scale::wrap(vec![Note::A2, Note::C2, Note::E2, Note::G2]).unwrap().to_rooted_chord(),
    ///     ]
    /// );
    /// ```
    pub fn to_all_inversions(self) -> Vec<Self>{
        self.as_all_inversions()
    }

    /// Stringify the `RootedChord`.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// let std = ChordStyle::Std(MStyle::Symbol, EStyle::Symbol);
    /// assert_eq!(&RootedChord::new(Note::C2, &MINOR_MAJOR_SEVENTH).as_string(std), "C-Δ");
    /// ```
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

impl AsSubs for RootedChord{
    fn as_subs(&self, max_len: Option<usize>) -> Vec<Self>{
        let scale = self.as_scale();
        let sub_scales = scale.as_subs(max_len);
        sub_scales.into_iter().map(|s| s.to_rooted_chord()).dedup().collect::<Vec<_>>()
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

impl ToPC for ScaleDegree{
    fn to_pc(self) -> PC{
        match self{
            Self::I    => PC::A,
            Self::bII  => PC::As,
            Self::II   => PC::B,
            Self::bIII => PC::C,
            Self::III  => PC::Cs,
            Self::IV   => PC::D,
            Self::bV   => PC::Ds,
            Self::V    => PC::E,
            Self::bVI  => PC::F,
            Self::VI   => PC::Fs,
            Self::bVII => PC::G,
            Self::VII  => PC::Gs,
        }
    }
}

impl ToNote for ScaleDegree{
    fn to_note(self) -> Note{
        self.to_pc().to_note()
    }
}

impl RelativeChord{
    /// Create a new `RelativeChord` from a [ScaleDegree][crate::theory::chord::ScaleDegree]
    /// and the chord intervals.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// let chord = RelativeChord::new(ScaleDegree::I, &MAJOR);
    /// assert!(chord.chord.contains_all(&POWER));
    /// ```
    pub fn new(degree: ScaleDegree, intervals: &[Note]) -> Self{
        Self{ degree, chord: Chord::new(intervals) }
    }

    /// Create a new 'RelativeChord' from a [ScaleDegree][crate::theory::chord::ScaleDegree]
    /// and a chord.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// let chord = RelativeChord::from_chord(ScaleDegree::I, Chord::new(&MAJOR_SEVENTH_CHORD));
    /// assert!(chord.chord.contains_all(&MAJOR));
    /// ```
    pub fn from_chord(degree: ScaleDegree, chord: Chord) -> Self{
        Self{ degree, chord }
    }

    /// Stringify the `RelativeChord`.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// let std = ChordStyle::Std(MStyle::Symbol, EStyle::Symbol);
    /// assert_eq!(&RelativeChord::new(ScaleDegree::II, &MINOR_MAJOR_SEVENTH).as_string(std), "II-Δ");
    /// ```
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
        assert!(Chord(vec![Note(4), Note(7)]).same_intervals(MAJOR));
    }

    #[test]
    fn chord_normalized(){
        assert_eq!(Chord::new(MAJOR).normalized(), Chord::new(MAJOR));
        assert_eq!(
            Chord::new(&[MAJ3, PER5, OCTAVE, MAJ9, PER12]).normalized(),
            Chord::new(&[MAJ2, MAJ3, PER5])
        );
        assert_eq!(
            Chord::new(&[MAJ3, MAJ9, PER12]).normalized(),
            Chord::new(&[MAJ2, MAJ3, PER5])
        );
        assert_eq!(
            Chord::new(&[MAJ3, MAJ3, MAJ9, PER12]).normalized(),
            Chord::new(&[MAJ2, MAJ3, PER5])
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
        // powerchord
        assert_eq!(&Chord::new(&[MIN3]).as_string(std), "X[♭3]");
        assert_eq!(&Chord::new(&[PER5]).as_string(std), "Xpower");
        // major
        assert_eq!(&Chord::new(&[MAJ3,PER5]).as_string(spl), "X[♮3♮5]");
        assert_eq!(&Chord::new(&[MAJ3,PER5]).as_string(std), "XΔ");
        assert_eq!(&Chord::new(&[MAJ3,PER5]).as_string(ext), "XΔ");
        assert_eq!(&Chord::new(&[MAJ3,PER5]).as_string(letr), "XM");
        assert_eq!(&Chord::new(&[MAJ3,PER5]).as_string(long), "Xmaj");
        assert_eq!(&Chord::new(&[MAJ3,PER5]).as_string(verbose), "Xmaj");
        // minor
        assert_eq!(&Chord::new(&[MIN3,PER5]).as_string(std), "X-");
        assert_eq!(&Chord::new(&[MIN3,PER5]).as_string(ext), "X-");
        assert_eq!(&Chord::new(&[MIN3,PER5]).as_string(letr), "Xm");
        assert_eq!(&Chord::new(&[MIN3,PER5]).as_string(long), "Xmin");
        assert_eq!(&Chord::new(&[MIN3,PER5]).as_string(verbose), "Xmin");
        // diminished
        assert_eq!(&Chord::new(&[MIN3,DIM5]).as_string(std), "X°");
        assert_eq!(&Chord::new(&[MIN3,DIM5]).as_string(ext), "X°");
        assert_eq!(&Chord::new(&[MIN3,DIM5]).as_string(letr), "X°");
        assert_eq!(&Chord::new(&[MIN3,DIM5]).as_string(long), "X°");
        assert_eq!(&Chord::new(&[MIN3,DIM5]).as_string(verbose), "Xdim");
        // major diminished
        assert_eq!(&Chord::new(&[MAJ3,DIM5]).as_string(std), "X[♮3♭5]");
        assert_eq!(&Chord::new(&[MAJ3,DIM5]).as_string(ext), "XΔ°");
        assert_eq!(&Chord::new(&[MAJ3,DIM5]).as_string(letr), "XM°");
        assert_eq!(&Chord::new(&[MAJ3,DIM5]).as_string(long), "Xmaj°");
        assert_eq!(&Chord::new(&[MAJ3,DIM5]).as_string(verbose), "Xmajdim");
        // suspended
        assert_eq!(&Chord::new(&[MAJ2,PER5]).as_string(std), "Xsus2");
        assert_eq!(&Chord::new(&[MAJ2,PER5]).as_string(ext), "Xsus2");
        assert_eq!(&Chord::new(&[MAJ2,PER5]).as_string(letr), "Xsus2");
        assert_eq!(&Chord::new(&[MAJ2,PER5]).as_string(long), "Xsus2");
        assert_eq!(&Chord::new(&[MAJ2,PER5]).as_string(verbose), "Xsus2");
        assert_eq!(&Chord::new(&[PER4,PER5]).as_string(std), "Xsus4");
        assert_eq!(&Chord::new(&[PER4,PER5]).as_string(ext), "Xsus4");
        // augmented
        assert_eq!(&Chord::new(&[MAJ3,AUG5]).as_string(std), "X+");
        assert_eq!(&Chord::new(&[MAJ3,AUG5]).as_string(ext), "X+");
        // minor augmented
        assert_eq!(&Chord::new(&[MIN3,AUG5]).as_string(std), "X[♭3♭6]");
        assert_eq!(&Chord::new(&[MIN3,AUG5]).as_string(ext), "X-+");
        // super suspended
        assert_eq!(&Chord::new(&[MAJ2,PER4]).as_string(std), "X[♮2♮4]");
        assert_eq!(&Chord::new(&[MAJ2,PER4]).as_string(ext), "Xssus");
        // phrygian
        assert_eq!(&Chord::new(&[MIN2,PER5]).as_string(std), "Xphry");
        assert_eq!(&Chord::new(&[MIN2,PER5]).as_string(ext), "Xphry");
        // lydian
        assert_eq!(&Chord::new(&[AUG4,PER5]).as_string(std), "Xlyd");
        assert_eq!(&Chord::new(&[AUG4,PER5]).as_string(ext), "Xlyd");
        // locrian
        assert_eq!(&Chord::new(&[MIN2,DIM5]).as_string(std), "Xloc2");
        assert_eq!(&Chord::new(&[MIN2,DIM5]).as_string(ext), "Xloc2");
        assert_eq!(&Chord::new(&[PER4,DIM5]).as_string(std), "Xloc4");
        assert_eq!(&Chord::new(&[PER4,DIM5]).as_string(ext), "Xloc4");
        // super locrian
        assert_eq!(&Chord::new(&[MIN2,PER4,DIM5]).as_string(std), "Xloc2(♮11)");
        assert_eq!(&Chord::new(&[MIN2,PER4,DIM5]).as_string(ext), "Xsloc");
        // major sixth
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ6]).as_string(std), "XΔ6");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ6]).as_string(ext), "XΔ6");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ6]).as_string(letr), "XM6");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ6]).as_string(long), "Xmaj6");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ6]).as_string(verbose), "Xmaj6");
        // minor sixth
        assert_eq!(&Chord::new(&[MIN3,PER5,MAJ6]).as_string(std), "X-6");
        assert_eq!(&Chord::new(&[MIN3,PER5,MAJ6]).as_string(ext), "X-6");
        // seventh (major, minor dominant)
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ7]).as_string(std), "XΔ7");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ7]).as_string(ext), "XΔ7");
        assert_eq!(&Chord::new(&[MIN3,PER5,MIN7]).as_string(std), "X-7");
        assert_eq!(&Chord::new(&[MIN3,PER5,MIN7]).as_string(ext), "X-7");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MIN7]).as_string(std), "X7");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MIN7]).as_string(ext), "X7");
        // minor major seventh
        assert_eq!(&Chord::new(&[MIN3,PER5,MAJ7]).as_string(std), "X-Δ");
        assert_eq!(&Chord::new(&[MIN3,PER5,MAJ7]).as_string(ext), "X-Δ");
        assert_eq!(&Chord::new(&[MIN3,PER5,MAJ7]).as_string(letr), "XmM");
        assert_eq!(&Chord::new(&[MIN3,PER5,MAJ7]).as_string(long), "Xminmaj");
        assert_eq!(&Chord::new(&[MIN3,PER5,MAJ7]).as_string(verbose), "Xminmaj");
        // half diminished
        assert_eq!(&Chord::new(&[MIN3,DIM5,MIN7]).as_string(std), "Xø");
        assert_eq!(&Chord::new(&[MIN3,DIM5,MIN7]).as_string(ext), "Xø");
        // diminished seventh
        assert_eq!(&Chord::new(&[MIN3,DIM5,DIM7]).as_string(std), "X°7");
        assert_eq!(&Chord::new(&[MIN3,DIM5,DIM7]).as_string(ext), "X°7");
        // augmented seventh
        assert_eq!(&Chord::new(&[MAJ3,AUG5,MIN7]).as_string(std), "X+7");
        assert_eq!(&Chord::new(&[MAJ3,AUG5,MIN7]).as_string(ext), "X+7");
        // mu chord
        assert_eq!(&Chord::new(&[MAJ2,MAJ3,PER5]).as_string(std), "XΔ(♮9)");
        assert_eq!(&Chord::new(&[MAJ2,MAJ3,PER5]).as_string(ext), "Xμ");
        // six nine chord
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ6,MAJ9]).as_string(std), "X6/9");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ6,MAJ9]).as_string(ext), "X6/9");
        // ninth (minor, major, dominant)
        assert_eq!(&Chord::new(&[MIN3,PER5,MIN7,MAJ9]).as_string(std), "X-9");
        assert_eq!(&Chord::new(&[MIN3,PER5,MIN7,MAJ9]).as_string(ext), "X-9");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ7,MAJ9]).as_string(std), "XΔ9");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ7,MAJ9]).as_string(ext), "XΔ9");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MIN7,MAJ9]).as_string(std), "X9");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MIN7,MAJ9]).as_string(ext), "X9");
        // eleventh (minor, major, dominant)
        assert_eq!(&Chord::new(&[MIN3,PER5,MIN7,MAJ9,MAJ11]).as_string(std), "X-11");
        assert_eq!(&Chord::new(&[MIN3,PER5,MIN7,MAJ9,MAJ11]).as_string(ext), "X-11");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ7,MAJ9,MAJ11]).as_string(std), "XΔ11");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ7,MAJ9,MAJ11]).as_string(ext), "XΔ11");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MIN7,MAJ9,MAJ11]).as_string(std), "X11");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MIN7,MAJ9,MAJ11]).as_string(ext), "X11");
        // thirteenth (minor, major, dominant)
        assert_eq!(&Chord::new(&[MIN3,PER5,MIN7,MAJ9,MAJ11,MAJ13]).as_string(std), "X-13");
        assert_eq!(&Chord::new(&[MIN3,PER5,MIN7,MAJ9,MAJ11,MAJ13]).as_string(ext), "X-13");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ7,MAJ9,MAJ11,MAJ13]).as_string(std), "XΔ13");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ7,MAJ9,MAJ11,MAJ13]).as_string(ext), "XΔ13");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MIN7,MAJ9,MAJ11,MAJ13]).as_string(std), "X13");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MIN7,MAJ9,MAJ11,MAJ13]).as_string(ext), "X13");
        // suspended sixth
        assert_eq!(&Chord::new(&[MAJ2,PER5,MAJ6,MAJ9]).as_string(std), "XΔ6sus2");
        assert_eq!(&Chord::new(&[MAJ2,PER5,MAJ6,MAJ9]).as_string(ext), "XΔ6sus2");
        assert_eq!(&Chord::new(&[PER4,PER5,MAJ6,MAJ9]).as_string(std), "XΔ6sus2(♮11)");
        assert_eq!(&Chord::new(&[PER4,PER5,MAJ6,MAJ9]).as_string(ext), "XΔ6ssus");
        // suspended seventh
        assert_eq!(&Chord::new(&[MAJ2,PER5,MIN7,MAJ9]).as_string(std), "X-7sus2");
        assert_eq!(&Chord::new(&[MAJ2,PER5,MIN7,MAJ9]).as_string(ext), "X-7sus2");
        assert_eq!(&Chord::new(&[MAJ2,PER5,MAJ7,MAJ9]).as_string(std), "XΔ7sus2");
        assert_eq!(&Chord::new(&[MAJ2,PER5,MAJ7,MAJ9]).as_string(ext), "XΔ7sus2");
        assert_eq!(&Chord::new(&[PER4,PER5,MAJ7,MAJ9]).as_string(std), "XΔ7sus2(♮11)");
        assert_eq!(&Chord::new(&[PER4,PER5,MAJ7,MAJ9]).as_string(ext), "XΔ7ssus");
        assert_eq!(&Chord::new(&[MAJ2,PER5,MIN7,MAJ9]).as_string(std), "X-7sus2");
        assert_eq!(&Chord::new(&[MAJ2,PER5,MIN7,MAJ9]).as_string(ext), "X-7sus2");
        assert_eq!(&Chord::new(&[PER4,PER5,MIN7,MAJ9]).as_string(std), "X-7sus2(♮11)");
        assert_eq!(&Chord::new(&[PER4,PER5,MIN7,MAJ9]).as_string(ext), "X-7ssus");
        assert_eq!(&Chord::new(&[MAJ2,PER5,MIN7,MAJ9,MAJ11]).as_string(std), "X-7sus2(♮11)");
        assert_eq!(&Chord::new(&[MAJ2,PER5,MIN7,MAJ9,MAJ11]).as_string(ext), "X-7ssus");
        assert_eq!(&Chord::new(&[PER4,PER5,MIN7,MAJ9,MAJ11]).as_string(std), "X-7sus2(♮11)");
        assert_eq!(&Chord::new(&[PER4,PER5,MIN7,MAJ9,MAJ11]).as_string(ext), "X-7ssus");
        assert_eq!(&Chord::new(&[MAJ2,PER5,MAJ7,MAJ9,MAJ11]).as_string(std), "XΔ7sus2(♮11)");
        assert_eq!(&Chord::new(&[MAJ2,PER5,MAJ7,MAJ9,MAJ11]).as_string(ext), "XΔ7ssus");
        assert_eq!(&Chord::new(&[PER4,PER5,MAJ7,MAJ9,MAJ11]).as_string(std), "XΔ7sus2(♮11)");
        assert_eq!(&Chord::new(&[PER4,PER5,MAJ7,MAJ9,MAJ11]).as_string(ext), "XΔ7ssus");
        // no 3 chords
        assert_eq!(&Chord::new(&[PER5,MAJ6]).as_string(std), "XΔ6no3");
        assert_eq!(&Chord::new(&[PER5,MAJ6]).as_string(ext), "XΔ6no3");
        assert_eq!(&Chord::new(&[PER5,MAJ7]).as_string(std), "XΔ7no3");
        assert_eq!(&Chord::new(&[PER5,MAJ7]).as_string(ext), "XΔ7no3");
        assert_eq!(&Chord::new(&[PER5,MIN7]).as_string(std), "X-7no3");
        assert_eq!(&Chord::new(&[PER5,MIN7]).as_string(ext), "X-7no3");
        // extension octave tests
        assert_eq!(&Chord::new(&[MAJ3,PER5,MIN6]).as_string(std), "XΔ(♭6)");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MIN6]).as_string(ext), "XΔ(♭6)");
        assert_eq!(&Chord::new(&[MIN3,PER5,MIN6]).as_string(std), "X-(♭6)");
        assert_eq!(&Chord::new(&[MIN3,PER5,MIN6]).as_string(ext), "X-(♭6)");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MIN9]).as_string(std), "XΔ(♭9)");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MIN9]).as_string(ext), "XΔ(♭9)");
        assert_eq!(&Chord::new(&[MIN3,PER5,MIN9]).as_string(std), "X-(♭9)");
        assert_eq!(&Chord::new(&[MIN3,PER5,MIN9]).as_string(ext), "X-(♭9)");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ9]).as_string(std), "XΔ(♮9)");
        assert_eq!(&Chord::new(&[MAJ3,PER5,MAJ9]).as_string(ext), "Xμ");
        assert_eq!(&Chord::new(&[MIN3,PER5,MAJ9]).as_string(std), "X-(♮9)");
        assert_eq!(&Chord::new(&[MIN3,PER5,MAJ9]).as_string(ext), "X-(♮9)");
        assert_eq!(&Chord::new(&[MAJ3,PER5,AUG9]).as_string(std), "XΔ(♯9)");
        assert_eq!(&Chord::new(&[MAJ3,PER5,AUG9]).as_string(ext), "XΔ(♯9)");
        assert_eq!(&Chord::new(&[MIN3,PER5,AUG9]).as_string(std), "X-"); // aug9 == min3
        assert_eq!(&Chord::new(&[MIN3,PER5,AUG9]).as_string(ext), "X-"); // aug9 == min3
        assert_eq!(&Chord::new(&[MAJ3,PER4,PER5]).as_string(std), "XΔ(♮11)");
        assert_eq!(&Chord::new(&[MAJ3,PER4,PER5]).as_string(ext), "XΔ(♮11)");
        assert_eq!(&Chord::new(&[MIN3,PER4,PER5]).as_string(std), "X-(♮11)");
        assert_eq!(&Chord::new(&[MIN3,PER4,PER5]).as_string(ext), "X-(♮11)");
        assert_eq!(&Chord::new(&[MAJ3,TRIT,PER5]).as_string(std), "XΔ(♯11)");
        assert_eq!(&Chord::new(&[MAJ3,TRIT,PER5]).as_string(ext), "XΔ(♯11)");
        assert_eq!(&Chord::new(&[MIN3,TRIT,PER5]).as_string(std), "X-(♯11)");
        assert_eq!(&Chord::new(&[MIN3,TRIT,PER5]).as_string(ext), "X-(♯11)");

        // experimental zone
        // assert_eq!(&Chord::new(&[MAJ2,PER5,MIN7,MAJ9,MAJ11,MAJ13]).as_string(std), "x13sus2");
        assert_eq!(&Chord::new(&[MAJ2,PER5,MIN7,MAJ9,MAJ11,MAJ13]).as_string(ext), "XΔ6ssus");
        // assert_eq!(&Chord::new(&[PER4,PER5,MIN7,MAJ9,MAJ11,MAJ13]).as_string(std), "x13sus4");
        assert_eq!(&Chord::new(&[PER4,PER5,MIN7,MAJ9,MAJ11,MAJ13]).as_string(ext), "XΔ6ssus");

        // assert_eq!(&Chord::new(&[MAJ2,PER5,MAJ7,MAJ9,MAJ11,MAJ13]).as_string(std), "X13");
        // assert_eq!(&Chord::new(&[MAJ2,PER5,MAJ7,MAJ9,MAJ11,MAJ13]).as_string(ext), "X13");
        // assert_eq!(&Chord::new(&[PER4,PER5,MAJ7,MAJ9,MAJ11,MAJ13]).as_string(std), "X13");

        assert_eq!(&Chord::new(&[PER4,PER5,MIN7,AUG9,AUG13]).as_string(std), "X-7(♮11)");
        assert_eq!(&Chord::new(&[PER4,PER5,MIN7,AUG9,AUG13]).as_string(ext), "X-7(♮11)");
        assert_eq!(&Chord::new(&[MAJ2,PER5,MAJ7,MIN9,AUG11]).as_string(std), "XΔ7sus2(♭9♯11)");
        assert_eq!(&Chord::new(&[MAJ2,PER5,MAJ7,MIN9,AUG11]).as_string(ext), "XΔ7sus2(♭9♯11)");
    }

    #[test]
    fn chord_as_scale(){
        assert_eq!(
            Chord(vec![]).to_scale(Note::F1),
            Scale(vec![Note::F1])
        );
        assert_eq!(
            Chord::new(MAJOR_SEVENTH_CHORD).to_scale(Note::F1),
            Scale(vec![Note::F1, Note::A2, Note::C2, Note::E2])
        );
    }

    #[test]
    fn rooted_chord_new(){
        assert_eq!(
            RootedChord::new(Note::A4, MAJOR_SIXTH_CHORD),
            RootedChord{ root: Note::A4, chord: Chord(vec![MAJ3, PER5, MAJ6]) }
        );
    }

    #[test]
    fn rooted_chord_from_chord(){
        assert_eq!(
            RootedChord::from_chord(Note::A4, Chord::new(MU_CHORD)),
            RootedChord{ root: Note::A4, chord: Chord(vec![MAJ2, MAJ3, PER5]) }
        );
    }

    #[test]
    fn rooted_chord_as_scale(){
        assert_eq!(
            RootedChord{ root: Note::A4, chord: Chord(vec![]) }.to_scale(),
            Scale(vec![Note::A4])
        );
        assert_eq!(
            RootedChord{ root: Note::A1, chord: Chord::new(MAJOR) }.to_scale(),
            Scale(vec![Note::A1, Note::CS1, Note::E1])
        );
    }

    #[test]
    fn rooted_chord_normalized(){
        assert_eq!(
            RootedChord::new(Note::A1, &[MAJ3, PER5, OCTAVE, MAJ9, PER12]).normalized(),
            RootedChord::new(Note::ZERO, &[MAJ2, MAJ3, PER5])
        );
    }

    #[test]
    fn rooted_chord_as_chordtone_wholetone_scale(){
        assert_eq!(RootedChord::new(Note::F1, MAJOR).as_chordtone_wholetone_scale(), None);
        assert_eq!(
            RootedChord::new(Note::F1, MAJOR_SEVENTH_CHORD).as_chordtone_wholetone_scale(),
            Some(Scale(vec![Note::F1, Note::G1, Note::A2, Note::B2, Note::C2, Note::D2, Note::E2]))
        );
        assert_eq!(
            RootedChord::new(Note::A1, MINOR_SEVENTH_CHORD).as_chordtone_wholetone_scale(),
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
            &RootedChord::new(Note::CS1, &[PER4,PER5,MIN7,AUG9,AUG13]).as_string(std),
            "C♯-7(♮11)"
        );
        // assert_eq!(
        //     &RootedChord::new(Note::CS1, &[MAJ2,PER5,MAJ7,MIN9,AUG13]).as_string(std),
        //     "C♯Δ7sus2(♭9♯11)"
        // );
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
    fn scale_degree_to_pc(){
        assert_eq!(ScaleDegree::I.to_pc(), PC::A);
        assert_eq!(ScaleDegree::bII.to_pc(), PC::As);
        assert_eq!(ScaleDegree::II.to_pc(), PC::B);
        assert_eq!(ScaleDegree::bIII.to_pc(), PC::C);
        assert_eq!(ScaleDegree::III.to_pc(), PC::Cs);
        assert_eq!(ScaleDegree::IV.to_pc(), PC::D);
        assert_eq!(ScaleDegree::bV.to_pc(), PC::Ds);
        assert_eq!(ScaleDegree::V.to_pc(), PC::E);
        assert_eq!(ScaleDegree::bVI.to_pc(), PC::F);
        assert_eq!(ScaleDegree::VI.to_pc(), PC::Fs);
        assert_eq!(ScaleDegree::bVII.to_pc(), PC::G);
        assert_eq!(ScaleDegree::VII.to_pc(), PC::Gs);
    }

    #[test]
    fn scale_degree_to_note(){
        assert_eq!(ScaleDegree::I.to_note(), Note::A0);
        assert_eq!(ScaleDegree::bII.to_note(), Note::AS0);
        assert_eq!(ScaleDegree::II.to_note(), Note::B0);
        assert_eq!(ScaleDegree::bIII.to_note(), Note::C0);
        assert_eq!(ScaleDegree::III.to_note(), Note::CS0);
        assert_eq!(ScaleDegree::IV.to_note(), Note::D0);
        assert_eq!(ScaleDegree::bV.to_note(), Note::DS0);
        assert_eq!(ScaleDegree::V.to_note(), Note::E0);
        assert_eq!(ScaleDegree::bVI.to_note(), Note::F0);
        assert_eq!(ScaleDegree::VI.to_note(), Note::FS0);
        assert_eq!(ScaleDegree::bVII.to_note(), Note::G0);
        assert_eq!(ScaleDegree::VII.to_note(), Note::GS0);
    }

    #[test]
    fn relative_chord_new(){
        assert_eq!(
            RelativeChord::new(ScaleDegree::I, MAJOR),
            RelativeChord{ degree: ScaleDegree::I, chord: Chord::new(MAJOR) }
        );
    }

    #[test]
    fn relative_chord_from_chord(){
        let chord = Chord::new(MINOR);
        assert_eq!(
            RelativeChord::from_chord(ScaleDegree::bV, chord.clone()),
            RelativeChord{ degree: ScaleDegree::bV, chord }
        );
    }

    #[test]
    fn relative_chord_as_string(){
        let long = ChordStyle::Std(MStyle::Long, EStyle::Long);
        assert_eq!(
            &RelativeChord::new(ScaleDegree::bVII, MAJOR_ELEVENTH_CHORD).as_string(long),
            "bVIImaj11"
        );
    }

    #[test]
    fn relative_chord_to_string(){
        assert_eq!(
            &RelativeChord::new(ScaleDegree::II, MINOR_NINTH_CHORD).to_string(),
            "II-9"
        );
    }

    #[test]
    fn rooted_chord_as_subs(){
        let (x, y, z) = (Note(0), Note(1), Note(2));
        let rc = Scale(vec![x, y, z]).to_rooted_chord();
        let mut iter = rc.as_subs(None).into_iter();
        // because (zero, []) happens to be the same as (x, []) here
        // assert_eq!(iter.next(), Some(RootedChord{ root: Note::ZERO, chord: Chord(vec![]) }));
        assert_eq!(iter.next(), Some(RootedChord{ root: x, chord: Chord(vec![]) }));
        assert_eq!(iter.next(), Some(RootedChord{ root: y, chord: Chord(vec![]) }));
        assert_eq!(iter.next(), Some(RootedChord{ root: z, chord: Chord(vec![]) }));
        assert_eq!(iter.next(), Some(RootedChord{ root: x, chord: Chord(vec![Note(1)]) }));
        assert_eq!(iter.next(), Some(RootedChord{ root: y, chord: Chord(vec![Note(11)]) }));
        assert_eq!(iter.next(), Some(RootedChord{ root: x, chord: Chord(vec![Note(2)]) }));
        assert_eq!(iter.next(), Some(RootedChord{ root: z, chord: Chord(vec![Note(10)]) }));
        assert_eq!(iter.next(), Some(RootedChord{ root: y, chord: Chord(vec![Note(1)]) }));
        assert_eq!(iter.next(), Some(RootedChord{ root: z, chord: Chord(vec![Note(11)]) }));
        assert_eq!(iter.next(), Some(RootedChord{ root: x, chord: Chord(vec![Note(1), Note(2)]) }));
        assert_eq!(iter.next(), Some(RootedChord{ root: y, chord: Chord(vec![Note(1), Note(11)]) }));
        assert_eq!(iter.next(), Some(RootedChord{ root: z, chord: Chord(vec![Note(10), Note(11)]) }));
        assert_eq!(iter.next(), None);
    }
}
