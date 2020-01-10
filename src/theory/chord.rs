use super::note::*;
use super::interval::*;
use super::scale::*;
use crate::utils::roman_numerals::to_roman_num;

type Chord = Vec<Note>;

pub const NUM_SUPS: [char; 10] = ['⁰', 'ⁱ', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
pub const NUM_SUBS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

pub const MAJOR_DYAD: [Note; 1] = [MAJOR_THIRD];
pub const MINOR_DYAD: [Note; 1] = [MINOR_THIRD];
pub const POWER_DYAD: [Note; 1] = [PERFECT_FIFTH];
pub const MAJOR_TRIAD: [Note; 2] = [MAJOR_THIRD, PERFECT_FIFTH];
pub const MINOR_TRIAD: [Note; 2] = [MINOR_THIRD, PERFECT_FIFTH];
pub const AUGMENTED_TRIAD: [Note; 2] = [MAJOR_THIRD, AUGMENTED_FIFTH];
pub const DIMINISHED_TRIAD: [Note; 2] = [MINOR_THIRD, DIMINISHED_FIFTH];
pub const MAJOR_SIXTH_TETRAD: [Note; 3] = [MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH];
pub const MINOR_SIXTH_TETRAD: [Note; 3] = [MINOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH];
pub const DOMINANT_SEVENTH_TETRAD: [Note; 3] = [MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];
pub const AUGMENTED_SEVENTH_TETRAD: [Note; 3] = [MAJOR_THIRD, AUGMENTED_FIFTH, MINOR_SEVENTH];
pub const MAJOR_SEVENTH_TETRAD: [Note; 3] = [MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];
pub const MINOR_SEVENTH_TETRAD: [Note; 3] = [MINOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];
pub const MINOR_MAJOR_SEVENTH_TETRAD: [Note; 3] = [MINOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];
pub const DIMINISHED_SEVENTH_TETRAD: [Note; 3] = [MINOR_THIRD, DIMINISHED_FIFTH, DIMINISHED_SEVENTH];
pub const HALF_DIMINISHED_SEVENTH_TETRAD: [Note; 3] = [MINOR_THIRD, DIMINISHED_FIFTH, MINOR_SEVENTH];

pub fn chord_from_intervals(base: Note, intervals: &[Note]) -> Chord{
    let mut chord = vec![base];
    for interval in intervals{
        chord.push(base + interval);
    }
    chord
}

pub const TRIAD_DEGREES: [usize; 2] = [3, 5];
pub const SEVENTH_DEGREES: [usize; 3] = [3, 5, 7];
pub const NINETH_DEGREES: [usize; 4] = [3, 5, 7, 9];
pub const ELEVENTH_DEGREES: [usize; 5] = [3, 5, 7, 9, 11];
pub const THIRTEENTH_DEGREES: [usize; 6] = [3, 5, 7, 9, 11, 13];

pub fn chord_from_scale(base: Note, scale: &Scale, degrees: &[usize]) -> Chord{
    let slen = scale.len();
    let mut chord = vec![base];
    let mut i = 1;
    let mut note = base;
    let mut index = 0;
    loop{
        if index >= degrees.len(){
            break;
        }
        let d = degrees[index];
        if i == d{
            chord.push(note);
            index += 1;
        }
        note += scale[(i - 1) % slen];
        i += 1;
    }
    chord
}

pub enum NamedChord{
    Arbitrary(Chord),
    Power(Note),
    Major(Note),
    Minor(Note),
    Augmented(Note),
    Diminished(Note),
    MajorSixth(Note),
    MinorSixth(Note),
    DominantSeventh(Note),
    AugmentedSeventh(Note),
    MajorSeventh(Note),
    MinorSeventh(Note),
    MinorMajorSeventh(Note),
    DiminishedSeventh(Note),
    HalfDiminishedSeventh(Note),
}

impl NamedChord{
    pub fn to_chord(&self) -> Chord{
        match self{
            Self::Arbitrary(chord) => chord.clone(),
            Self::Power(n) => chord_from_intervals(*n, &POWER_DYAD),
            Self::Major(n) => chord_from_intervals(*n, &MAJOR_TRIAD),
            Self::Minor(n) => chord_from_intervals(*n, &MINOR_TRIAD),
            Self::Augmented(n) => chord_from_intervals(*n, &AUGMENTED_TRIAD),
            Self::Diminished(n) => chord_from_intervals(*n, &DIMINISHED_TRIAD),
            Self::MajorSixth(n) => chord_from_intervals(*n, &MAJOR_SIXTH_TETRAD),
            Self::MinorSixth(n) => chord_from_intervals(*n, &MINOR_SIXTH_TETRAD),
            Self::DominantSeventh(n) => chord_from_intervals(*n, &DOMINANT_SEVENTH_TETRAD),
            Self::AugmentedSeventh(n) => chord_from_intervals(*n, &AUGMENTED_SEVENTH_TETRAD),
            Self::MajorSeventh(n) => chord_from_intervals(*n, &MAJOR_SEVENTH_TETRAD),
            Self::MinorSeventh(n) => chord_from_intervals(*n, &MINOR_SEVENTH_TETRAD),
            Self::MinorMajorSeventh(n) => chord_from_intervals(*n, &MINOR_MAJOR_SEVENTH_TETRAD),
            Self::DiminishedSeventh(n) => chord_from_intervals(*n, &DIMINISHED_SEVENTH_TETRAD),
            Self::HalfDiminishedSeventh(n) => chord_from_intervals(*n, &HALF_DIMINISHED_SEVENTH_TETRAD),
        }
    }

    pub fn from_chord(chord: &Chord) -> Self{
        fn same_intervals(inters: &Chord, blueprint: &[Note]) -> bool{
            if inters.len() != blueprint.len() + 1{
                return false;
            }
            let len = inters.len();
            for i in 1..len{
                if inters[i] != blueprint[i - 1]{
                    return false;
                }
            }
            true
        }
        if chord.len() == 0{
            return Self::Arbitrary(Vec::new());
        }
        let intervals = &intervals_from_chord(chord);
        let root = chord[0];
        if same_intervals(intervals, &POWER_DYAD){
            Self::Power(root)
        }else if same_intervals(intervals, &MAJOR_TRIAD){
            Self::Major(root)
        }else if same_intervals(intervals, &MINOR_TRIAD){
            Self::Minor(root)
        }else if same_intervals(intervals, &AUGMENTED_TRIAD){
            Self::Augmented(root)
        }else if same_intervals(intervals, &DIMINISHED_TRIAD){
            Self::Diminished(root)
        }else if same_intervals(intervals, &MAJOR_SIXTH_TETRAD){
            Self::MajorSixth(root)
        }else if same_intervals(intervals, &MINOR_SIXTH_TETRAD){
            Self::MinorSixth(root)
        }else if same_intervals(intervals, &DOMINANT_SEVENTH_TETRAD){
            Self::DominantSeventh(root)
        }else if same_intervals(intervals, &AUGMENTED_SEVENTH_TETRAD){
            Self::AugmentedSeventh(root)
        }else if same_intervals(intervals, &MAJOR_SEVENTH_TETRAD){
            Self::MajorSeventh(root)
        }else if same_intervals(intervals, &MINOR_SEVENTH_TETRAD){
            Self::MinorSeventh(root)
        }else if same_intervals(intervals, &MINOR_MAJOR_SEVENTH_TETRAD){
            Self::MinorMajorSeventh(root)
        }else if same_intervals(intervals, &DIMINISHED_SEVENTH_TETRAD){
            Self::DiminishedSeventh(root)
        }else if same_intervals(intervals, &HALF_DIMINISHED_SEVENTH_TETRAD){
            Self::HalfDiminishedSeventh(root)
        }else{
            Self::Arbitrary(chord.clone())
        }
    }

    pub fn from_intervals(base: Note, intervals: &[Note]) -> Self{
        let chord = chord_from_intervals(base, intervals);
        Self::from_chord(&chord)
    }

    pub fn root(&self) -> Note{
        *match self{
            Self::Power(r) => r,
            Self::Major(r) => r,
            Self::Minor(r) => r,
            Self::Augmented(r) => r,
            Self::Diminished(r) => r,
            Self::MajorSixth(r) => r,
            Self::MinorSixth(r) => r,
            Self::DominantSeventh(r) => r,
            Self::AugmentedSeventh(r) => r,
            Self::MajorSeventh(r) => r,
            Self::MinorSeventh(r) => r,
            Self::MinorMajorSeventh(r) => r,
            Self::DiminishedSeventh(r) => r,
            Self::HalfDiminishedSeventh(r) => r,
            Self::Arbitrary(ch) => {
                if ch.is_empty() {
                    &-1
                }else{
                    &ch[0]
                }
            },
        }
    }

    pub fn base_quality(&self, basestr: String) -> String{
        let mut lowercase = String::new();
        for c in basestr.chars(){
            for l in c.to_lowercase(){
                lowercase.push(l);
            }
        }
        match self{
            Self::Power(_) => format!("{}!", basestr),
            Self::Major(_) => format!("{}", basestr),
            Self::Minor(_) => format!("{}", lowercase),
            Self::Augmented(_) => format!("{}+", basestr),
            Self::Diminished(_) => format!("{}o", basestr),
            Self::MajorSixth(_) => format!("{}maj6", basestr),
            Self::MinorSixth(_) => format!("{}min6", basestr),
            Self::DominantSeventh(_) => format!("{}dom7", basestr),
            Self::AugmentedSeventh(_) => format!("{}+7", basestr),
            Self::MajorSeventh(_) => format!("{}∆", basestr),
            Self::MinorSeventh(_) => format!("{}-", basestr),
            Self::MinorMajorSeventh(_) => format!("{}min(maj7)", basestr),
            Self::DiminishedSeventh(_) => format!("{}o7", basestr),
            Self::HalfDiminishedSeventh(_) => format!("{}ø7", basestr),
            Self::Arbitrary(_) => String::new(),
        }
    }

    pub fn spelled_out_quality(&self, basestr: String) -> String{
        match self{
            Self::Arbitrary(ch) => {
                let mut st = format!("{}", basestr);
                let intervals = intervals_from_chord(ch);
                for interval in intervals.iter().skip(1){
                    st.push_str(&format!("{}", interval_name_short(*interval)));
                }
                st
            },
            _ => String::new(),
        }
    }

    pub fn equal_spaced_quality(&self, mut basestr: String) -> String{
        let notes = self.to_chord();
        let len = notes.len();
        if len <= 0{
            String::new()
        }else if len == 1{
            basestr
        }else if len <= 9{
            let mut last = notes[1];
            let leap = last - notes[0];
            let mut ok = true;
            for n in notes.iter().skip(2){
                if leap != n - last{
                    ok = false;
                    break;
                }
                last = *n;
            }
            if ok && leap > 0 && leap < 10{
                basestr.push(NUM_SUBS[leap as usize]);
                basestr.push(NUM_SUPS[len]);
                basestr
            }else{
                String::new()
            }
            
        }else{
            String::new()
        }
    }

    pub fn decorate_quality(&self, basestr: String) -> String{
        let decoration = self.base_quality(basestr);
        decoration
    }

    pub fn as_string(&self) -> String{
        let root = NamedNote::from_note(self.root()).to_string_name_sharp();
        self.decorate_quality(root)
    }
}

pub fn intervals_from_chord(chord: &Chord) -> Chord{
    if chord.is_empty() { return Vec::new(); }
    let root = chord[0];
    let mut intervals = vec![0];
    for note in chord.iter().skip(1){
        let diff = note - root;
        intervals.push(diff);
    }
    intervals
}

pub fn chord_as_string(chord: &Chord) -> String{
    NamedChord::from_chord(chord).as_string()
}

pub fn print_chords(chords: &[Chord], sep: &str){
    let len = chords.len();
    if len <= 0 { return; }
    for chord in chords.iter().take(len - 1){
        print!("{}{}", chord_as_string(chord), sep);
    }
    println!("{}", chord_as_string(&chords[len - 1]));
}

pub fn print_strings(strs: &[String], sep: &str){
    let len = strs.len();
    if len <= 0 { return; }
    for s in strs.iter().take(len - 1){
        print!("{}{}", s, sep);
    }
    println!("{}", strs[len - 1]);
}

pub fn scale_chords(scale: &Scale, size: usize) -> Vec<Chord>{
    let len = scale.len();
    let mut chords = Vec::new();
    for (i, root) in note_iter(0, scale).enumerate().take(len){
        let mut chord = Vec::new();
        for note in note_iter(0, scale).skip(i).step_by(2).take(size){
            chord.push(note);
        }
        chords.push(chord);
    }
    chords
}

pub fn strs_scale_chords_roman(scale: &Scale, size: usize) -> Vec<String>{
    let chords = scale_chords(scale, size);
    let mut res = Vec::new();
    for i in 0..chords.len(){
        res.push(NamedChord::from_chord(&chords[i]).decorate_quality(to_roman_num(i + 1)));
    }
    res
}

pub fn scale_chords_intervals(scale: &Scale, size: usize) -> Vec<Chord>{
    let chords_notes = scale_chords(scale, size);
    map(&chords_notes, &intervals_from_chord)
}

pub fn map<T,F>(inp: &[T], f: &F) -> Vec<T>
    where
        F: Fn(&T) -> T,
{
    let mut res = Vec::new();
    for x in inp{
        res.push(f(x));
    }
    res
}
