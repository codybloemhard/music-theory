use super::traits::{ ToNote };
use super::interval::{ _OCTAVE };
use super::pc::{ PC };

pub const A4: Note = Note(48);

pub(crate) const _MAX_NOTE: u32 = 2147483648;
pub const MAX_NOTE: Note = Note(_MAX_NOTE);

pub type _Note = u32;
pub type Rank = u16;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Note(pub(crate) u32);

impl Note{
    pub fn new(note: u32) -> Self{
        Self(note.min(_MAX_NOTE))
    }

    pub fn inside(&self) -> u32{
        self.0
    }

    /*
    0   1   2   3   4   5   6   7   8   9   10  11  // rank 0
    12  13  14  15  16  17  18  19  20  21  22  23  // rank 1
    24  25  26  27  28  29  30  31  32  33  34  35  // rank 2
    36  37  38  39  40  41  42  43  44  45  46  47  // rank 3
    48                                              // A4
    */

    // note 48 is A4 at 440 hz
    pub fn to_pitch(&self) -> f32{
        let x = self.0 as i32 - 48;
        (2.0f32).powf(x as f32 / _OCTAVE as f32) * 440.0f32
    }
}

impl std::ops::Add for Note{
    type Output = Self;

    fn add(self, other: Self) -> Self{
        Self::new(self.0 + other.0)
    }
}

impl ToNote for _Note{
    fn to_note(self) -> Note{
        Note::new(self)
    }
}

impl ToNote for PC{
    fn to_note(self) -> Note{
        Note::new(self as _Note)
    }
}

// Keep collections of notes distinct.
// It's all the same with different interpretation.
// Once, it was all just ```Vec<Note>``` with different types such as ```type Scale = Vec<Note>```.
// This provides us with compile time checks.
// Interchanging the versions now only can be done explicitly.

// pub type Notes = Vec<Note>;
//
// #[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Steps(pub Vec<Note>);
//
// #[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Scale(pub Vec<Note>);
//
// #[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Chord(pub Vec<Note>);
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum RelativeNote { Flat(u32), Sharp(u32), Natural }
//
// pub const RN_NAT: RelativeNote = RelativeNote::Natural;
// pub const RN_S:   RelativeNote = RelativeNote::Sharp(1);
// pub const RN_SS:  RelativeNote = RelativeNote::Sharp(2);
// pub const RN_B:   RelativeNote = RelativeNote::Flat(1);
// pub const RN_BB:  RelativeNote = RelativeNote::Flat(2);
//
// #[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Relative(pub Vec<RelativeNote>);
//
// impl Relative{
//     pub fn empty(len: usize) -> Self{
//         Relative(vec![RelativeNote::Natural; len])
//     }
// }
//
// pub trait NoteSequence{
//     fn len(&self) -> usize;
//     fn is_empty(&self) -> bool;
// }
//
// macro_rules! ImplNoteSequence{
//     ($type:ty) => {
//         impl NoteSequence for $type{
//             fn len(&self) -> usize{
//                 self.0.len()
//             }
//
//             fn is_empty(&self) -> bool{
//                 self.0.is_empty()
//             }
//         }
//     }
// }
//
// ImplNoteSequence!(Steps);
// ImplNoteSequence!(Scale);
// ImplNoteSequence!(Chord);
// ImplNoteSequence!(Relative);
//
// pub trait ToStringName{
//     fn to_string_name(&self) -> String;
// }
//
// pub trait ToNote{
//     fn to_note(&self) -> Note;
// }
//
// pub trait HasRank{
//     fn with_rank(self, rank: Rank) -> Self;
// }
//
// impl HasRank for Note{
//     fn with_rank(self, rank: Rank) -> Note{
//         (self % _OCTAVE) + rank as Note * _OCTAVE
//     }
// }
//
// pub trait ToScale{
//     fn to_scale(&self, note: Note) -> Scale;
// }
//
// pub trait IntoScale{
//     fn into_scale(self, note: Note) -> Scale;
// }
//
// impl<T: ToScale> IntoScale for T{
//     fn into_scale(self, note: Note) -> Scale{
//         self.to_scale(note)
//     }
// }
//
// pub trait ToSteps{
//     fn to_steps(&self) -> Steps;
// }
//
// pub trait IntoSteps{
//     fn into_steps(self) -> Steps;
// }
//
// impl<T: ToSteps> IntoSteps for T{
//     fn into_steps(self) -> Steps{
//         self.to_steps()
//     }
// }
//
// pub trait ToRelative{
//     fn to_relative(&self, reference: &Steps) -> Option<Relative>;
// }
//
// pub trait IntoRelative{
//     fn into_relative(self, reference: &Steps) -> Option<Relative>;
// }
//
// impl<T: ToRelative> IntoRelative for T{
//     fn into_relative(self, reference: &Steps) -> Option<Relative>{
//         self.to_relative(reference)
//     }
// }
//
// pub trait ToChord{
//     fn to_chord(&self) -> Chord;
// }
//
// pub trait IntoChord{
//     fn into_chord(self) -> Chord;
// }
//
// impl<T: ToChord> IntoChord for T{
//     fn into_chord(self) -> Chord{
//         self.to_chord()
//     }
// }
//
// pub trait ToEnharmonicNote{
//     fn to_enharmonic_note(&self) -> Option<EnharmonicNote>;
// }
//
// pub trait IntoEnharmonicNote{
//     fn into_enharmonic_note(self) -> Option<EnharmonicNote>;
// }
//
// impl<T: ToEnharmonicNote> IntoEnharmonicNote for T{
//     fn into_enharmonic_note(self) -> Option<EnharmonicNote>{
//         self.to_enharmonic_note()
//     }
// }
//
// pub trait IntoEnharmonicNotes{
//     fn into_enharmonic_notes(self) -> Vec<EnharmonicNote>;
// }
//
// pub trait IntoEnharmonicNotesWithStart{
//     fn into_enharmonic_notes_with_start(self, start: Option<EnharmonicNote>) -> Vec<EnharmonicNote>;
// }
//
// impl ToString for RelativeNote{
//     fn to_string(&self) -> String{
//         let mut res = String::new();
//         match self{
//             RelativeNote::Natural => {  },
//             RelativeNote::Sharp(i) => {
//                 for _ in 0..*i { res.push('♯'); }
//             },
//             RelativeNote::Flat(i) => {
//                 for _ in 0..*i { res.push('♭'); }
//             }
//         }
//         res
//     }
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum Letter{
//     A = 0, B = 1, C = 2, D = 3, E = 4, F = 5, G = 6
// }
//
// impl ToString for Letter{
//     fn to_string(&self) -> String{
//         match self{
//             Self::A => "A",
//             Self::B => "B",
//             Self::C => "C",
//             Self::D => "D",
//             Self::E => "E",
//             Self::F => "F",
//             Self::G => "G",
//         }.to_string()
//     }
// }
//
// impl Cycle for Letter{
//     fn next(&self) -> Self{
//         match self{
//             Self::A => Self::B,
//             Self::B => Self::C,
//             Self::C => Self::D,
//             Self::D => Self::E,
//             Self::E => Self::F,
//             Self::F => Self::G,
//             Self::G => Self::A,
//         }
//     }
//
//     fn prev(&self) -> Self{
//         match self{
//             Self::A => Self::G,
//             Self::B => Self::A,
//             Self::C => Self::B,
//             Self::D => Self::C,
//             Self::E => Self::D,
//             Self::F => Self::E,
//             Self::G => Self::F,
//         }
//     }
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct EnharmonicNote{
//     pub letter: Letter,
//     pub accidental: Note,
// }
//
// impl std::fmt::Display for EnharmonicNote{
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
//         write!(f, "{}", self.to_string_name())
//     }
// }
//
// impl EnharmonicNote{
//     pub fn spelled_as(&self, letter: Letter) -> Self{
//         if self.letter == letter { return *self; }
//         let up = {
//             let mut en = *self;
//             loop {
//                 if en.letter == letter { break en; }
//                 en = en.next();
//             }
//         };
//         let down = {
//             let mut en = *self;
//             loop {
//                 if en.letter == letter { break en; }
//                 en = en.prev();
//             }
//         };
//         if up.accidental.abs() > down.accidental.abs() {
//             down
//         } else {
//             up
//         }
//     }
// }
//
// impl Cycle for EnharmonicNote{
//     fn next(&self) -> Self{
//         match self.letter{
//             Letter::A => Self{ letter: Letter::B, accidental: self.accidental - 2 }, // A = Bbb
//             Letter::B => Self{ letter: Letter::C, accidental: self.accidental - 1 }, // B = Cb
//             Letter::C => Self{ letter: Letter::D, accidental: self.accidental - 2 }, // C = Dbb
//             Letter::D => Self{ letter: Letter::E, accidental: self.accidental - 2 }, // D = Ebb
//             Letter::E => Self{ letter: Letter::F, accidental: self.accidental - 1 }, // E = Fb
//             Letter::F => Self{ letter: Letter::G, accidental: self.accidental - 2 }, // F = Gbb
//             Letter::G => Self{ letter: Letter::A, accidental: self.accidental - 2 }, // G = Abb
//         }
//     }
//
//     fn prev(&self) -> Self{
//         match self.letter{
//             Letter::A => Self{ letter: Letter::G, accidental: self.accidental + 2 }, // A = G##
//             Letter::B => Self{ letter: Letter::A, accidental: self.accidental + 2 }, // B = A##
//             Letter::C => Self{ letter: Letter::B, accidental: self.accidental + 1 }, // C = B#
//             Letter::D => Self{ letter: Letter::C, accidental: self.accidental + 2 }, // D = C##
//             Letter::E => Self{ letter: Letter::D, accidental: self.accidental + 2 }, // E = D##
//             Letter::F => Self{ letter: Letter::E, accidental: self.accidental + 1 }, // F = E#
//             Letter::G => Self{ letter: Letter::F, accidental: self.accidental + 2 }, // G = F##
//         }
//     }
// }
//
// impl ToStringName for EnharmonicNote{
//     fn to_string_name(&self) -> String{
//         let mut res = self.letter.to_string();
//         res.push_str(&(
//             if self.accidental < 0 {
//                 RelativeNote::Flat(self.accidental.unsigned_abs())
//             } else {
//                 RelativeNote::Sharp(self.accidental.unsigned_abs())
//             }.to_string()
//         ));
//         res
//     }
// }
//
// impl ToNote for Letter{
//     fn to_note(&self) -> Note{
//         match self{
//             Letter::A => 0,
//             Letter::B => 2,
//             Letter::C => 3,
//             Letter::D => 5,
//             Letter::E => 7,
//             Letter::F => 8,
//             Letter::G => 10,
//         }
//     }
// }
//
// impl ToNote for EnharmonicNote{
//     fn to_note(&self) -> Note{
//         (self.letter.to_note() + self.accidental) as Note
//     }
// }
//
// impl ToPC for EnharmonicNote{
//     fn to_pc(&self) -> PC{
//         self.to_note().to_pc()
//     }
// }
//
// pub trait ToLetterTry{
//     fn to_letter_try(&self) -> Option<Letter>;
// }
//
// impl ToLetterTry for String{
//     fn to_letter_try(&self) -> Option<Letter>{
//         match self.chars().next().map(|c| c.to_lowercase().next()){
//             Some(Some('a')) => Some(Letter::A),
//             Some(Some('b')) => Some(Letter::B),
//             Some(Some('c')) => Some(Letter::C),
//             Some(Some('d')) => Some(Letter::D),
//             Some(Some('e')) => Some(Letter::E),
//             Some(Some('f')) => Some(Letter::F),
//             Some(Some('g')) => Some(Letter::G),
//             _ => None
//         }
//     }
// }
//
// impl ToLetterTry for usize{
//     fn to_letter_try(&self) -> Option<Letter>{
//         match self{
//             0 => Some(Letter::A),
//             1 => Some(Letter::B),
//             2 => Some(Letter::C),
//             3 => Some(Letter::D),
//             4 => Some(Letter::E),
//             5 => Some(Letter::F),
//             6 => Some(Letter::G),
//             _ => None,
//         }
//     }
// }
//
// impl ToEnharmonicNote for String{
//     fn to_enharmonic_note(&self) -> Option<EnharmonicNote>{
//         let mut lowercase = String::new();
//         for c in self.chars(){
//             for l in c.to_lowercase(){
//                 lowercase.push(l);
//             }
//         }
//         let chars = lowercase.chars();
//         let letter = self.to_letter_try()?;
//         let mut accidental = 0;
//         for ch in chars{
//             match ch{
//                 'b' => { accidental -= 1; },
//                 '♭' => { accidental -= 1; },
//                 '#' => { accidental += 1; },
//                 '♯' => { accidental += 1; },
//                 '♮' => { accidental = 0; }
//                 _ => return None,
//             }
//         }
//         Some(EnharmonicNote{ letter, accidental })
//     }
// }
//
// impl IntoEnharmonicNotes for String{
//     fn into_enharmonic_notes(self) -> Vec<EnharmonicNote>{
//         self.split(',').into_iter().filter_map(|s| s.to_string().to_enharmonic_note()).collect::<Vec<_>>()
//     }
// }
//
// impl ToEnharmonicNote for Note{
//     fn to_enharmonic_note(&self) -> Option<EnharmonicNote>{
//         Some(match self % _OCTAVE{
//             0  => EnharmonicNote{ letter: Letter::A, accidental: 0 },
//             1  => EnharmonicNote{ letter: Letter::A, accidental: 1 },
//             2  => EnharmonicNote{ letter: Letter::B, accidental: 0 },
//             3  => EnharmonicNote{ letter: Letter::C, accidental: 0 },
//             4  => EnharmonicNote{ letter: Letter::C, accidental: 1 },
//             5  => EnharmonicNote{ letter: Letter::D, accidental: 0 },
//             6  => EnharmonicNote{ letter: Letter::D, accidental: 1 },
//             7  => EnharmonicNote{ letter: Letter::E, accidental: 0 },
//             8  => EnharmonicNote{ letter: Letter::F, accidental: 0 },
//             9  => EnharmonicNote{ letter: Letter::F, accidental: 1 },
//             10 => EnharmonicNote{ letter: Letter::G, accidental: 0 },
//             11 => EnharmonicNote{ letter: Letter::G, accidental: 1 },
//             _ => panic!("ToEnharmonicNote for Note, should be impossible."),
//         })
//     }
// }
// // Could be used for hexatonics etc?
// fn _into_enharmonic_notes_with_start_subheptatonic(scale: Scale, start: Option<EnharmonicNote>) -> Vec<EnharmonicNote>{
//     let mut set = vec![0, 0, 0, 0, 0, 0, 0];
//     let mut res = Vec::new();
//     let skip = if let Some(en) = start{
//         set[en.letter as usize] = 1;
//         res.push(en);
//         1
//     } else {
//         0
//     };
//     for (i, note) in scale.0.into_iter().enumerate().skip(skip){
//         if i >= 7 { return Vec::new(); } // Impossible: no more letters.
//         let en = note.to_enharmonic_note().unwrap();
//         let en = if set[en.letter as usize] == 1{
//             let mut nen = en;
//             loop {
//                 nen = nen.next();
//                 if set[nen.letter as usize] == 0 { break nen; }
//             }
//         } else {
//             en
//         };
//         set[en.letter as usize] = 1;
//         res.push(en);
//     }
//     res
// }
//
// fn into_enharmonic_notes_with_start_heptatonic(scale: Scale, start: Option<EnharmonicNote>) -> Vec<EnharmonicNote>{
//     let mut res = Vec::new();
//     if scale.len() != 7 { return res; }
//     let (skip, mut target_letter) = if let Some(en) = start{
//         res.push(en);
//         (1, en.next().letter)
//     } else {
//         (0, scale.0[0].to_enharmonic_note().unwrap().letter)
//     };
//     for (i, note) in scale.0.into_iter().enumerate().skip(skip){
//         if i >= 7 { return Vec::new(); } // Impossible: no more letters.
//         let en = note.to_enharmonic_note().unwrap();
//         let new_en = if en.letter == target_letter {
//             en
//         } else {
//             en.spelled_as(target_letter)
//         };
//         res.push(new_en);
//         target_letter = target_letter.next();
//     }
//     res
// }
//
// impl IntoEnharmonicNotes for Scale{
//     fn into_enharmonic_notes(self) -> Vec<EnharmonicNote>{
//         into_enharmonic_notes_with_start_heptatonic(self, None)
//     }
// }
//
// impl IntoEnharmonicNotesWithStart for Scale{
//     fn into_enharmonic_notes_with_start(self, start: Option<EnharmonicNote>) -> Vec<EnharmonicNote>{
//         into_enharmonic_notes_with_start_heptatonic(self, start)
//     }
// }

#[cfg(test)]
mod tests{
    use super::*;
    use crate::theory::*;

    #[test]
    fn note_to_pitch(){
        assert_eq!(A4.to_pitch().round() as i32, 440);
    }

    #[test]
    fn note_new(){
        assert_eq!(MAX_NOTE, Note::new(_MAX_NOTE + 1));
    }

    #[test]
    fn note_add(){
        assert_eq!(Note(0) + Note(0), Note(0));
        assert_eq!(Note(1) + Note(0), Note(1));
        assert_eq!(Note(_SEMI) + Note(_WHOLE), Note(_MIN3));
        assert_eq!(MAX_NOTE + Note(1), MAX_NOTE);
    }

    #[test]
    fn u32_to_note(){
        for i in 0..12345{
            assert_eq!(Note::new(i), i.to_note());
        }
    }

    #[test]
    fn pc_to_note(){
        for (i, pc) in PCS.iter().enumerate(){
            assert_eq!(pc.to_note().inside() as usize, i);
        }
    }
}
