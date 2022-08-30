use super::{
    Note, PC, PCs, Octave, OctaveShift, Interval, NamedInterval, NamedOctaveInterval, Letter,
    EnharmonicNote, Mode, Scale, Steps, Intervals, ModeIterator, Chord, RootedChord
};

use std::marker::Sized;

// General Traits

pub trait Wrapper where Self: Sized{
    type Inner;
    fn wrap(inner: Self::Inner) -> Option<Self>;
    fn unwrap(self) -> Self::Inner;
}

pub trait VecWrapper{
    type Item;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn iter(&self) -> std::slice::Iter<'_, Self::Item>;
    fn contains(&self, item: &Self::Item) -> bool;
    fn contains_all(&self, items: &[Self::Item]) -> bool;
    fn contains_any(&self, items: &[Self::Item]) -> bool;
}

macro_rules! ImplVecWrapper{
    ($type:ty, $item:ty) => {
        impl VecWrapper for $type{
            type Item = $item;

            fn len(&self) -> usize{
                self.0.len()
            }

            fn is_empty(&self) -> bool{
                self.0.is_empty()
            }

            fn iter(&self) -> std::slice::Iter<'_, Self::Item>{
                self.0.iter()
            }

            fn contains(&self, item: &Self::Item) -> bool{
                self.0.contains(item)
            }

            fn contains_all(&self, items: &[Self::Item]) -> bool{
                items.iter().all(|x| self.contains(x))
            }

            fn contains_any(&self, items: &[Self::Item]) -> bool{
                self.iter().any(|x| items.contains(x))
            }
        }

        impl crate::theory::traits::AsSubs for $type{
            fn as_subs(&self, max_len: Option<usize>) -> Vec<Self>{
                use crate::utils::sub_vecs;
                let subs = sub_vecs(&self.0, max_len);
                subs.into_iter().map(|s| Self(s)).collect::<Vec<_>>()
            }
        }

        impl std::ops::Index<usize> for $type{
            type Output = $item;

            fn index(&self, index: usize) -> &Self::Output{
                &self.0[index]
            }
        }

        impl std::ops::IndexMut<usize> for $type{
            fn index_mut(&mut self, index: usize) -> &mut Self::Output{
                &mut self.0[index]
            }
        }
    }
}

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

pub trait ModeIteratorSpawner<T: ModeTrait + VecWrapper>{
    fn mode_iter(self) -> ModeIterator<T>;
}

pub trait AsSubs where Self: Sized{
    fn as_subs(&self, max_len: Option<usize>) -> Vec<Self>;
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

pub trait AsScale{
    fn as_scale(&self, note: Note) -> Scale;
}

pub trait ToScale{
    fn to_scale(self, note: Note) -> Scale;
}

impl<T: AsScale> ToScale for T{
    fn to_scale(self, note: Note) -> Scale{
        self.as_scale(note)
    }
}

pub trait AsScaleTry{
    fn as_scale_try(&self, note: Note) -> Option<Scale>;
}

pub trait ToScaleTry{
    fn to_scale_try(self, note: Note) -> Option<Scale>;
}

impl<T: AsScaleTry> ToScaleTry for T{
    fn to_scale_try(self, note: Note) -> Option<Scale>{
        self.as_scale_try(note)
    }
}

pub trait AsSteps{
    fn as_steps(&self, complete_octave_cycle: bool) -> Steps;
}

pub trait ToSteps{
    fn to_steps(self, complete_octave_cycle: bool) -> Steps;
}

impl<T: AsSteps> ToSteps for T{
    fn to_steps(self, complete_octave_cycle: bool) -> Steps{
        self.as_steps(complete_octave_cycle)
    }
}

pub trait AsStepsTry{
    fn as_steps_try(&self, complete_octave_cycle: bool) -> Option<Steps>;
}

pub trait ToStepsTry{
    fn to_steps_try(self, complete_octave_cycle: bool) -> Option<Steps>;
}

impl<T: AsStepsTry> ToStepsTry for T{
    fn to_steps_try(self, complete_octave_cycle: bool) -> Option<Steps>{
        self.as_steps_try(complete_octave_cycle)
    }
}

pub trait AsPCs{
    fn as_pcs(&self) -> PCs;
}

pub trait ToPCs{
    fn to_pcs(self) -> PCs;
}

impl<T: AsPCs> ToPCs for T{
    fn to_pcs(self) -> PCs{
        self.as_pcs()
    }
}

pub trait AsRelativeIntervals{
    fn as_relative_intervals(&self, reference: &Self) -> Option<Intervals>;
}

pub trait ToRelativeIntervals{
    fn to_relative_intervals(self, reference: &Self) -> Option<Intervals>;
}

impl<T: AsRelativeIntervals> ToRelativeIntervals for T{
    fn to_relative_intervals(self, reference: &Self) -> Option<Intervals>{
        self.as_relative_intervals(reference)
    }
}

pub trait AsIonianRelativeStringTry{
    fn as_ionian_relative_string_try(&self, nonnat: bool) -> Option<String>;
}

pub trait ToIonianRelativeStringTry{
    fn to_ionian_relative_string_try(self, nonnat: bool) -> Option<String>;
}

impl<T: AsIonianRelativeStringTry> ToIonianRelativeStringTry for T{
    fn to_ionian_relative_string_try(self, nonnat: bool) -> Option<String>{
        self.as_ionian_relative_string_try(nonnat)
    }
}

pub trait AsEnharmonicNotes{
    fn as_enharmonic_notes(&self) -> Vec<EnharmonicNote>;
}

pub trait ToEnharmonicNotes{
    fn to_enharmonic_notes(self) -> Vec<EnharmonicNote>;
}

impl<T: AsEnharmonicNotes> ToEnharmonicNotes for T{
    fn to_enharmonic_notes(self) -> Vec<EnharmonicNote>{
        self.as_enharmonic_notes()
    }
}

pub trait AsEnharmonicNotesWithStart{
    fn as_enharmonic_notes_with_start(&self, start: Option<EnharmonicNote>) -> Vec<EnharmonicNote>;
}

pub trait ToEnharmonicNotesWithStart{
    fn to_enharmonic_notes_with_start(self, start: Option<EnharmonicNote>) -> Vec<EnharmonicNote>;
}

impl<T: AsEnharmonicNotesWithStart> ToEnharmonicNotesWithStart for T{
    fn to_enharmonic_notes_with_start(self, start: Option<EnharmonicNote>) -> Vec<EnharmonicNote>{
        self.as_enharmonic_notes_with_start(start)
    }
}

pub trait AsChord{
    fn as_chord(&self) -> Chord;
}

pub trait ToChord{
    fn to_chord(self) -> Chord;
}

impl<T: AsChord> ToChord for T{
    fn to_chord(self) -> Chord{
        self.as_chord()
    }
}

pub trait AsRootedChord{
    fn as_rooted_chord(&self) -> RootedChord;
}

pub trait ToRootedChord{
    fn to_rooted_chord(self) -> RootedChord;
}

impl<T: AsRootedChord> ToRootedChord for T{
    fn to_rooted_chord(self) -> RootedChord{
        self.as_rooted_chord()
    }
}

