use fnrs::MutFunc;
use super::note::*;
use super::interval::*;
use super::scale::*;
use crate::utils::roman_numerals::to_roman_num;
use crate::utils::misc::*;

pub const NUM_SUPS: [char; 10] = ['⁰', 'ⁱ', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
pub const NUM_SUBS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

pub const MAJOR_DYAD: [Note; 1] = [MAJOR_THIRD];
pub const MINOR_DYAD: [Note; 1] = [MINOR_THIRD];
pub const POWER_DYAD: [Note; 1] = [PERFECT_FIFTH];
pub const MAJOR_TRIAD: [Note; 2] = [MAJOR_THIRD, PERFECT_FIFTH];
pub const MINOR_TRIAD: [Note; 2] = [MINOR_THIRD, PERFECT_FIFTH];
pub const MINOR_AUGMENTED_TRIAD: [Note; 2] = [MINOR_THIRD, AUGMENTED_FIFTH];
pub const MAJOR_AUGMENTED_TRIAD: [Note; 2] = [MAJOR_THIRD, AUGMENTED_FIFTH];
pub const MINOR_DIMINISHED_TRIAD: [Note; 2] = [MINOR_THIRD, DIMINISHED_FIFTH];
pub const MAJOR_DIMINISHED_TRIAD: [Note; 2] = [MAJOR_THIRD, DIMINISHED_FIFTH];
pub const MAJOR_SIXTH_TETRAD: [Note; 3] = [MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH];
pub const MINOR_SIXTH_TETRAD: [Note; 3] = [MINOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH];
pub const DOMINANT_SEVENTH_TETRAD: [Note; 3] = [MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];
pub const AUGMENTED_SEVENTH_TETRAD: [Note; 3] = [MAJOR_THIRD, AUGMENTED_FIFTH, MINOR_SEVENTH];
pub const MAJOR_SEVENTH_TETRAD: [Note; 3] = [MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];
pub const MINOR_SEVENTH_TETRAD: [Note; 3] = [MINOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];
pub const MINOR_MAJOR_SEVENTH_TETRAD: [Note; 3] = [MINOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];
pub const DIMINISHED_SEVENTH_TETRAD: [Note; 3] = [MINOR_THIRD, DIMINISHED_FIFTH, DIMINISHED_SEVENTH];
pub const HALF_DIMINISHED_SEVENTH_TETRAD: [Note; 3] = [MINOR_THIRD, DIMINISHED_FIFTH, MINOR_SEVENTH];

pub fn chord_from_intervals(base: Note, intervals: &[Note]) -> Notes{
    let mut chord = vec![base];
    for interval in intervals{
        chord.push(base + interval);
    }
    chord
}

pub fn chord_from_equal_spacing(base: Note, interval: Note, size: usize) -> Notes{
    let mut chord = vec![base];
    let mut last = base;
    for _ in 0..size{
        last += interval;
        chord.push(last);
    }
    chord
}

pub const TRIAD_DEGREES: [usize; 2] = [3, 5];
pub const SEVENTH_DEGREES: [usize; 3] = [3, 5, 7];
pub const NINETH_DEGREES: [usize; 4] = [3, 5, 7, 9];
pub const ELEVENTH_DEGREES: [usize; 5] = [3, 5, 7, 9, 11];
pub const THIRTEENTH_DEGREES: [usize; 6] = [3, 5, 7, 9, 11, 13];

pub fn chord_from_scale(base: Note, scale: &Notes, degrees: &[usize]) -> Notes{
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

pub fn same_intervals(inters: &Notes, blueprint: &[Note]) -> bool{
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

pub fn has_intervals(inters: &Notes, blueprint: &[Note]) -> bool{
    for note in blueprint{
        if !inters.contains(note){
            return false;
        }
    }
    true
}

pub enum ChordStyle{
    SpelledOut,
    Base,
    EqualSpaced,
    Extended,
}

pub enum NamedChord{
    Arbitrary(Notes),
    Power(Note),
    Major(Note),
    Minor(Note),
    MinorAugmented(Note),
    MajorAugmented(Note),
    MinorDiminished(Note),
    MajorDiminished(Note),
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
    pub fn to_chord(&self) -> Notes{
        match self{
            Self::Arbitrary(chord) => chord.clone(),
            Self::Power(n) => chord_from_intervals(*n, &POWER_DYAD),
            Self::Major(n) => chord_from_intervals(*n, &MAJOR_TRIAD),
            Self::Minor(n) => chord_from_intervals(*n, &MINOR_TRIAD),
            Self::MinorAugmented(n) => chord_from_intervals(*n, &MINOR_AUGMENTED_TRIAD),
            Self::MajorAugmented(n) => chord_from_intervals(*n, &MAJOR_AUGMENTED_TRIAD),
            Self::MinorDiminished(n) => chord_from_intervals(*n, &MINOR_DIMINISHED_TRIAD),
            Self::MajorDiminished(n) => chord_from_intervals(*n, &MAJOR_DIMINISHED_TRIAD),
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

    pub fn from_chord(chord: &Notes) -> Self{
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
        }else if same_intervals(intervals, &MINOR_AUGMENTED_TRIAD){
            Self::MinorAugmented(root)
        }else if same_intervals(intervals, &MAJOR_AUGMENTED_TRIAD){
            Self::MajorAugmented(root)
        }else if same_intervals(intervals, &MINOR_DIMINISHED_TRIAD){
            Self::MinorDiminished(root)
        }else if same_intervals(intervals, &MAJOR_DIMINISHED_TRIAD){
            Self::MajorDiminished(root)
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
            Self::MinorAugmented(r) => r,
            Self::MajorAugmented(r) => r,
            Self::MinorDiminished(r) => r,
            Self::MajorDiminished(r) => r,
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

    pub fn base_quality(&self, basestr: String, lower: bool) -> String{
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
        match self{
            Self::Power(_) => format!("{}!", basestr),
            Self::Major(_) => format!("{}", basestr),
            Self::Minor(_) => format!("{}", minorstr),
            Self::MinorAugmented(_) => format!("{}+", minorstr),
            Self::MajorAugmented(_) => format!("{}+", basestr),
            Self::MinorDiminished(_) => format!("{}o", minorstr),
            Self::MajorDiminished(_) => format!("{}o", basestr),
            Self::MajorSixth(_) => format!("{}maj6", basestr),
            Self::MinorSixth(_) => format!("{}min6", basestr),
            Self::DominantSeventh(_) => format!("{}7", basestr),
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
                let mut st = format!("{}[", basestr);
                let intervals = intervals_from_chord(ch);
                for interval in intervals.iter().skip(1){
                    st.push_str(&format!("{}", to_chord_interval(*interval)));
                }
                st.push_str(&"]");
                st
            },
            _ => String::new(),
        }
    }

    pub fn equal_spaced_quality(&self, mut basestr: String) -> String{
        let notes = self.to_chord();
        let len = notes.len();
        if len <= 1{
            String::new()
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
            if ok && len > 0 && len < 10{
                basestr.push_str(&to_chord_interval(leap));
                basestr.push(NUM_SUPS[len]);
                basestr
            }else{
                String::new()
            }
        }else{
            String::new()
        }
    }

    pub fn get_base_chord(chord: &Notes) -> Option<Self>{
        if chord.is_empty(){
            return Option::None;
        }
        let root = chord[0];
        let intervals = &intervals_from_chord(chord);
        let all_bases = vec![POWER_DYAD.to_vec(),MAJOR_TRIAD.to_vec(),MINOR_TRIAD.to_vec(),
            MINOR_AUGMENTED_TRIAD.to_vec(),MAJOR_AUGMENTED_TRIAD.to_vec(),
            MINOR_DIMINISHED_TRIAD.to_vec(), MAJOR_DIMINISHED_TRIAD.to_vec(),
            MAJOR_SIXTH_TETRAD.to_vec(),MINOR_SIXTH_TETRAD.to_vec(),DOMINANT_SEVENTH_TETRAD.to_vec(),AUGMENTED_SEVENTH_TETRAD.to_vec(),
            MAJOR_SEVENTH_TETRAD.to_vec(),MINOR_SEVENTH_TETRAD.to_vec(),MINOR_MAJOR_SEVENTH_TETRAD.to_vec(),
            DIMINISHED_SEVENTH_TETRAD.to_vec(),HALF_DIMINISHED_SEVENTH_TETRAD.to_vec()];
        let mut biggest = 0;
        let mut pattern = Vec::new();
        for base in all_bases.iter().rev(){
            if base == intervals{
                return Option::Some(Self::from_intervals(root, &base));
            }
            if has_intervals(intervals, &base){
                let size = base.len();
                if size > biggest{
                    biggest = size;
                    pattern = base.clone();
                }
            }
        }
        if pattern.is_empty(){
            return Option::None;
        }
        Option::Some(Self::from_intervals(root, &pattern))
    }

    pub fn base_chord(&self) -> Option<Self>{
        Self::get_base_chord(&self.to_chord())
    }

    pub fn extended_quality(&self, basestr: String, lower: bool) -> String{
        let mut with_m3 = self.to_chord();
        if with_m3.len() <= 1{
            return basestr;
        }
        let root = with_m3[0];
        if !(with_m3.contains(&MINOR_THIRD) || with_m3.contains(&MAJOR_THIRD)){
            with_m3.push(root + MAJOR_THIRD);
            with_m3.sort();
        }
        let base_chord = Self::get_base_chord(&with_m3);
        if base_chord.is_none(){
            return String::new();
        }
        let base_chord = base_chord.unwrap();

        let (mut not_in_chord, mut not_in_base) =
            both_differences(&self.to_chord(), &base_chord.to_chord());
        not_in_chord.mmap(|note| *note -= root);
        not_in_base.mmap(|note| *note -= root);

        let mut res = base_chord.as_string_basic(lower);
        let mut attrs = Vec::new();
        let sus_type = if not_in_chord.contains(&MAJOR_THIRD){ // sus chord
            if not_in_base.contains(&PERFECT_FOURTH){
                attrs.push(String::from("sus♮4"));
                PERFECT_FOURTH
            }else if not_in_base.contains(&MAJOR_SECOND){
                attrs.push(String::from("sus♮2"));
                MAJOR_SECOND
            }else if not_in_base.contains(&MINOR_SECOND){
                attrs.push(String::from("sus♭2"));
                MINOR_SECOND
            }else { 0 }
        }else { 0 };
        for inter in not_in_base{
            if inter == sus_type { continue; }
            attrs.push(to_chord_interval(inter));
        }
        for attr in attrs{
            res.push_str(&attr);
        }
        res
    }

    pub fn decorate_quality_single(&self, basestr: String, lower: bool) -> String{
        let mut decoration = self.base_quality(basestr.clone(), lower);
        if &decoration == ""{
            decoration = self.equal_spaced_quality(basestr.clone());
        }
        if &decoration == ""{
            decoration = self.extended_quality(basestr.clone(), lower);
        }
        if &decoration == ""{
            decoration = self.spelled_out_quality(basestr);
        }
        decoration
    }

    pub fn as_string_style(&self, style: ChordStyle, lower: bool) -> String{
        let root = NamedNote::from_note(self.root()).to_string_name_sharp();
        match style{
            ChordStyle::SpelledOut => self.spelled_out_quality(root),
            ChordStyle::Base => self.base_quality(root, lower),
            ChordStyle::EqualSpaced => self.equal_spaced_quality(root),
            ChordStyle::Extended => self.extended_quality(root, lower),
        }
    }

    pub fn as_string_basic(&self, lower: bool) -> String{
        let root = NamedNote::from_note(self.root()).to_string_name_sharp();
        let mut decoration = self.base_quality(root.clone(), lower);
        if &decoration == ""{
            decoration = self.equal_spaced_quality(root.clone());
        }
        decoration
    }

    pub fn as_string(&self) -> String{
        let root = NamedNote::from_note(self.root()).to_string_name_sharp();
        self.decorate_quality_single(root, false)
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

pub fn chord_as_string(chord: &Notes) -> String{
    NamedChord::from_chord(chord).as_string()
}

pub fn print_chords(chords: &[Notes], sep: &str){
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

pub fn scale_chords(scale: &Notes, size: usize) -> Vec<Notes>{
    let len = scale.len();
    let mut chords = Vec::new();
    for (i, _) in note_iter(0, scale).enumerate().take(len){
        let mut chord = Vec::new();
        for note in note_iter(0, scale).skip(i).step_by(2).take(size){
            chord.push(note);
        }
        chords.push(chord);
    }
    chords
}

pub fn strs_scale_chords_roman(scale: &Notes, size: usize) -> Vec<String>{
    let chords = scale_chords(scale, size);
    let mut res = Vec::new();
    for i in 0..chords.len(){
        res.push(NamedChord::from_chord(&chords[i]).decorate_quality_single(to_roman_num(i + 1), true));
    }
    res
}

pub fn scale_chords_intervals(scale: &Notes, size: usize) -> Vec<Notes>{
    let chords_notes = scale_chords(scale, size);
    fnrs::map(&chords_notes, &intervals_from_chord)
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_chord_as_string0(){
        assert_eq!(
            NamedChord::from_intervals(A4, &vec![MAJOR_THIRD,PERFECT_FIFTH]).as_string(),
            "A");
    }
    #[test]
    fn test_chord_as_string1(){
        assert_eq!(
            NamedChord::from_intervals(A4, &vec![MINOR_THIRD,PERFECT_FIFTH]).as_string(),
            "Am");
    }
    #[test]
    fn test_chord_as_string2(){
        assert_eq!(
            NamedChord::from_intervals(A4, &vec![MAJOR_SECOND,PERFECT_FIFTH]).as_string(),
            "Asus♮2");
    }
    #[test]
    fn test_chord_as_string3(){
        assert_eq!(
            NamedChord::from_intervals(A4, &vec![MINOR_SECOND,MAJOR_SECOND,PERFECT_FOURTH,PERFECT_FIFTH]).as_string(),
            "Asus♮4♭2♮2");
    }
    #[test]
    fn test_chord_as_string4(){
        assert_eq!(
            NamedChord::from_intervals(A4, &vec![MINOR_THIRD,AUGMENTED_FIFTH]).as_string(),
            "Am+");
    }
    #[test]
    fn test_chord_as_string5(){
        assert_eq!(
            NamedChord::from_intervals(A4, &vec![MAJOR_THIRD,DIMINISHED_FIFTH]).as_string(),
            "Ao");
    }
    #[test]
    fn test_chord_as_string6(){
        assert_eq!(
            NamedChord::from_chord(&chord_from_equal_spacing(A4, PERFECT_FOURTH, 3)).as_string(),
            "A♮4⁴");
    }
    #[test]
    fn test_equal_spaced_chord0(){
        assert_eq!(
            chord_from_equal_spacing(A4, MINOR_THIRD, 3),
            chord_from_intervals(A4, &vec![MINOR_THIRD,2*MINOR_THIRD,3*MINOR_THIRD]));
    }
}
