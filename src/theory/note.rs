use super::traits::{ ToNote, ToPC, OctaveShiftable, GeneratablePartialOrder, AddInterval, ToLetterTry };
use super::{ Interval, _OCTAVE, PC, Letter };

pub const A4: Note = Note(48);

pub type _Note = u32;
pub type Octave = u16;
pub type OctaveShift = i16;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Note(pub(crate) u32);

impl Note{
    pub const MAX: Note = Note(1073741824); // 1 << 30
    pub const MIN: Note = Note(0);

    pub fn new(note: u32) -> Self{
        Self(note.min(Self::MAX.0))
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

// General implementations

impl std::ops::Add for Note{
    type Output = Self;

    fn add(self, other: Self) -> Self{
        Self::new(self.0 + other.0)
    }
}

impl std::ops::Sub for Note {
    type Output = Interval;

    fn sub(self, other: Self) -> Interval{
        Interval(self.0 as i32 - other.0 as i32)
    }
}

impl GeneratablePartialOrder for Note{
    fn next(self) -> Option<Note>{
        if self.0 >= Self::MAX.0 { return None; }
        Some(Self(self.0 + 1))
    }

    fn prev(self) -> Option<Note>{
        let subbed = self.0.checked_sub(1)?;
        Some(Self(subbed))
    }
}

impl OctaveShiftable for Note{
    fn with_octave(self, octave: Octave) -> Note{
        (((self.0 % _OCTAVE) as i32 + octave as i32 * _OCTAVE as i32) as _Note).to_note()
    }

    fn shift_octave(self, shift: OctaveShift) -> Note{
        ((self.0 as i32 + shift as i32 * _OCTAVE as i32).max(0) as _Note).to_note()
    }
}

impl AddInterval for Note{
    fn add_interval(self, interval: Interval) -> Option<Self>{
        let res = (self.0 as i32).checked_add(interval.0)?;
        match res < 0 || res > Self::MAX.0 as i32{
            true => None,
            false => Some(Self(res as u32)),
        }
    }
}

// Conversion implementations

impl ToNote for _Note{
    fn to_note(self) -> Note{
        Note::new(self)
    }
}

impl ToPC for _Note{
    fn to_pc(self) -> PC{
        self.to_note().to_pc()
    }
}

impl ToPC for Note{
    fn to_pc(self) -> PC{
        let index = self.0 as usize % 12;
        PC::ALL[index]
    }
}

impl ToLetterTry for Note{
    fn to_letter_try(&self) -> Option<Letter>{
        self.to_pc().to_letter_try()
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

#[cfg(test)]
mod tests{
    use super::*;
    use crate::theory::*;

    #[test]
    fn to_pitch(){
        assert_eq!(A4.to_pitch().round() as i32, 440);
    }

    #[test]
    fn new(){
        assert_eq!(Note::MAX, Note::new(Note::MAX.0 + 1));
    }

    #[test]
    fn add(){
        assert_eq!(Note(0) + Note(0), Note(0));
        assert_eq!(Note(1) + Note(0), Note(1));
        assert_eq!(Note(_SEMI) + Note(_WHOLE), Note(_MIN3));
        assert_eq!(Note::MAX + Note(1), Note::MAX);
    }

    #[test]
    fn sub(){
        assert_eq!(Note(0) - Note(0), Interval(0));
        assert_eq!(Note(0) - Note(1), Interval(-1));
        assert_eq!(Note(1) - Note(0), Interval(1));
        assert_eq!(Note::MAX - Note::MIN, Interval::MAX);
        assert_eq!(Note::MIN - Note::MAX, Interval::MIN);
    }

    #[test]
    fn u32_to_note(){
        for i in 0..12345{
            assert_eq!(Note::new(i), i.to_note());
        }
    }

    #[test]
    fn note_to_pc(){
        assert_eq!(A4.to_pc(), PC::A);
        assert_eq!(Note::new(12).to_pc(), PC::A);
    }

    #[test]
    fn to_note_to_pc(){
        assert_eq!(13.to_pc(), PC::As);
    }

    #[test]
    fn octave_shiftable(){
        for i in 0..12u32{
            let note = i.to_note();
            assert_eq!(note, note.shift_octave(0));
            assert_eq!(note.inside() + 36, note.with_octave(3).inside());
            assert_eq!(note.to_pc(), note.shift_octave(12345).to_pc());
            assert_eq!(Note::MAX.with_octave(u16::MAX) < Note::MAX, true);
            assert_eq!(Note::MAX.shift_octave(i16::MAX), Note::MAX);
            assert_eq!(Note::MIN.shift_octave(i16::MIN), Note::MIN);
        }
    }

    #[test]
    fn generatable_partial_order(){
        assert_eq!(Note(0).next(), Some(Note(1)));
        assert_eq!(Note(1).prev(), Some(Note(0)));
        assert_eq!(Note::MAX.next(), None);
        assert_eq!(Note::MIN.prev(), None);
    }

    #[test]
    fn add_interval(){
        for i in 0..12{
            assert_eq!(Note(i).to_pc(), Note(i).add_interval(Interval::OCTAVE).unwrap().to_pc());
        }
        assert_eq!(Note::MAX.add_interval(Interval::SEMI), None);
        assert_eq!(Note::MIN.add_interval(-Interval::SEMI), None);
    }

    #[test]
    fn to_letter_try(){
        assert_eq!(Note(0).to_letter_try(), Some(Letter::A));
        assert_eq!(Note(1).to_letter_try(), Some(Letter::A));
        assert_eq!(Note(2).to_letter_try(), Some(Letter::B));
        assert_eq!(Note(3).to_letter_try(), Some(Letter::C));
        assert_eq!(Note(4).to_letter_try(), Some(Letter::C));
        assert_eq!(Note(5).to_letter_try(), Some(Letter::D));
        assert_eq!(Note(6).to_letter_try(), Some(Letter::D));
        assert_eq!(Note(7).to_letter_try(), Some(Letter::E));
        assert_eq!(Note(8).to_letter_try(), Some(Letter::F));
        assert_eq!(Note(9).to_letter_try(), Some(Letter::F));
        assert_eq!(Note(10).to_letter_try(), Some(Letter::G));
        assert_eq!(Note(11).to_letter_try(), Some(Letter::G));
    }
}
