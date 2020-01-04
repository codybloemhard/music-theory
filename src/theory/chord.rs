use super::note::*;
use super::interval::*;
use super::scale::*;

type Chord = Vec<Note>;

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
            if inters.len() != blueprint.len(){
                return false;
            }
            let len = inters.len();
            for i in 0..len{
                if inters[i] != blueprint[i]{
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
