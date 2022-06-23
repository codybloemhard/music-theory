use super::{
    Note, PC, Octave, OctaveShift, Interval, NamedInterval, NamedOctaveInterval, Letter,
    EnharmonicNote, Mode
};

use std::marker::Sized;

// General Traits

// You always know next, prev and it goes round n round
pub trait Cyclic{
    fn next(self) -> Self;
    fn prev(self) -> Self;
}

// Possible to generate next, prev, might fail
pub trait GeneratablePartialOrder where Self: Sized{
    fn next(self) -> Option<Self>;
    fn prev(self) -> Option<Self>;
}

pub trait OctaveShiftable{
    fn with_octave(self, octave: Octave) -> Self;
    fn shift_octave(self, shift: OctaveShift) -> Self;
}

pub trait AddInterval where Self: Sized{
    fn add_interval(self, interval: Interval) -> Option<Self>;
}

pub trait ModeTrait{
    fn next_mode_mut(&mut self);
    fn next_mode(self) -> Self;
    fn mode(self, mode: Mode) -> Self;
}

// Conversion Traits
// Note, PC, Interval, NamedInterval, NamedOctaveInterval, Letter, EnharmonicNote
// Scale

pub trait ToNote{
    fn to_note(self) -> Note;
}

pub trait ToPC{
    fn to_pc(self) -> PC;
}

pub trait ToInterval{
    fn to_interval(self) -> Interval;
}

pub trait ToNamedInterval{
    fn to_named_interval_try(self) -> Option<NamedInterval>;
    fn to_named_interval_mod(self) -> NamedInterval;
}

pub trait ToNamedOctaveInterval{
    fn to_named_octave_interval_try(self) -> Option<NamedOctaveInterval>;
    fn to_named_octave_interval_mod(self) -> NamedOctaveInterval;
}

pub trait ToLetterTry{
    fn to_letter_try(&self) -> Option<Letter>;
}

pub trait ToEnharmonicNote{
    fn to_enharmonic_note(self) -> EnharmonicNote;
}

pub trait ToEnharmonicNoteTry{
    fn to_enharmonic_note_try(&self) -> Option<EnharmonicNote>;
}

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

