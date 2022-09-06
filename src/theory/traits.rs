use super::{
    Note, PC, PCs, Octave, OctaveShift, Interval, NamedInterval, NamedOctaveInterval, Letter,
    EnharmonicNote, Mode, Scale, Steps, Intervals, ModeIterator, Chord, RootedChord, ScaleIterator
};

use std::marker::Sized;

// General Traits

/// Wrapper around existing types.
/// Used for the new type pattern.
pub trait Wrapper where Self: Sized{
    /// The type the new type wraps.
    type Inner;
    /// Try to wrap a [Self::Inner][Self::Inner] type into the [Self][Self] type.
    fn wrap(inner: Self::Inner) -> Option<Self>;
    /// Unwrap a [Self][Self] type into a [Self::Inner][Self::Inner] type.
    fn unwrap(self) -> Self::Inner;
}

/// Wrapper around existing vector types.
/// Used for the new type pattern.
pub trait VecWrapper{
    /// The type the new type wraps.
    type Item;

    /// Return the length of the inner vector.
    fn len(&self) -> usize;
    /// Return whether the inner vector is empty or not.
    fn is_empty(&self) -> bool;
    /// Iterate over the inner vector.
    fn iter(&self) -> std::slice::Iter<'_, Self::Item>;
    /// Returns whether the inner vector contains an item.
    fn contains(&self, item: &Self::Item) -> bool;
    /// Returns whether the inner vector contains all of the given items.
    fn contains_all(&self, items: &[Self::Item]) -> bool;
    /// Returns whether the inner vector contains any of the given items.
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

/// Can always generate a next and previous value.
pub trait Cyclic{
    /// Generate the next item.
    fn next(self) -> Self;
    /// Generate the previous item.
    fn prev(self) -> Self;
}

/// Like [Cyclic][Cyclic] you can generate next and previous items.
/// However, the operation might fail and return [None][Option::None].
pub trait GeneratablePartialOrder where Self: Sized{
    /// Generate the next item.
    fn next(self) -> Option<Self>;
    /// Generate the previous item.
    fn prev(self) -> Option<Self>;
}

/// Music theory types that have a notion of being in a certain octave, and can shift or set that
/// octave.
pub trait OctaveShiftable{
    /// Have the value with everything the same except change the octave to the given one.
    fn with_octave(self, octave: Octave) -> Self;
    /// Have the value with everything the same except shift the octave from the current one with
    /// the given shift.
    fn shift_octave(self, shift: OctaveShift) -> Self;
}

/// Music theory types that have the ability to have an [Interval][Interval] added to them.
pub trait AddInterval where Self: Sized{
    /// Add an [Interval][Interval].
    fn add_interval(self, interval: Interval) -> Option<Self>;
}

/// Types that have modes (rotations of scales).
pub trait ModeTrait{
    /// Modify the value so it turns into it's next mode.
    fn next_mode_mut(&mut self);
    /// Take the value and give back it's next mode.
    fn next_mode(self) -> Self;
    /// Take the value and give back it's Nth mode.
    fn mode(self, mode: Mode) -> Self;
}

/// The ability to spawn an iterator that yields modes.
pub trait ModeIteratorSpawner<T: ModeTrait + VecWrapper>{
    /// Spawn the mode iterator.
    fn mode_iter(self) -> ModeIterator<T>;
}

/// The ability to spawn an iterator that yields notes.
/// Not very useful for types that already yield notes with their natural iterator.
pub trait ScaleIteratorSpawner{
    /// Spawn the iterator, with the root or starting note.
    fn scale_iter(&self, root: Note) -> ScaleIterator;
}

/// The ability to generate subs sets of it's self.
pub trait AsSubs where Self: Sized{
    /// Generate all subs sets with optional maximal subset length.
    /// The output size of this grows really fast with the input size so be aware:
    /// 2, 5, 16, 65, 326, 1957, 13700, 109601, 986410, 9864101, 108505112.
    fn as_subs(&self, max_len: Option<usize>) -> Vec<Self>;
}

// Conversion Traits

/// Convert to [Note][Note].
pub trait ToNote{
    /// Take self and return a [Note][Note].
    fn to_note(self) -> Note;
}

/// Convert to [PC][PC].
pub trait ToPC{
    /// Take self and return [PC][PC].
    fn to_pc(self) -> PC;
}

/// Convert to [Interval][Interval].
pub trait ToInterval{
    /// Take self and return [Interval][Interval].
    fn to_interval(self) -> Interval;
}

/// Convert to [NamedInterval][NamedInterval].
pub trait ToNamedInterval{
    /// Try to convert to [NamedInterval][NamedInterval].
    fn to_named_interval_try(self) -> Option<NamedInterval>;
    /// Convert to [NamedInterval][NamedInterval] with wrapping around the octaves.
    fn to_named_interval_mod(self) -> NamedInterval;
}

/// Convert to [NamedOctaveInterval][NamedOctaveInterval].
pub trait ToNamedOctaveInterval{
    /// Try to convert to [NamedOctaveInterval][NamedOctaveInterval].
    fn to_named_octave_interval_try(self) -> Option<NamedOctaveInterval>;
    /// Convert to [NamedOctaveInterval][NamedOctaveInterval] with wrapping around the octave.
    fn to_named_octave_interval_mod(self) -> NamedOctaveInterval;
}

/// Try to convert to [Letter][Letter].
pub trait ToLetterTry{
    /// Take self and try to return [Letter][Letter].
    fn to_letter_try(&self) -> Option<Letter>;
}

/// Convert to [EnharmonicNote][EnharmonicNote].
pub trait ToEnharmonicNote{
    /// Take self and return [EnharmonicNote][EnharmonicNote].
    fn to_enharmonic_note(self) -> EnharmonicNote;
}

/// Try to convert to [EnharmonicNote][EnharmonicNote].
pub trait ToEnharmonicNoteTry{
    /// Take self and try to return [EnharmonicNote][EnharmonicNote].
    fn to_enharmonic_note_try(&self) -> Option<EnharmonicNote>;
}

/// Convert to [Scale][Scale].
pub trait AsScale{
    /// Borrow self and return [Scale][Scale].
    /// Note is either the root of the scale or the octave.
    fn as_scale(&self, note: Note) -> Scale;
}

/// Convert to [Scale][Scale].
pub trait ToScale{
    /// Take self and return [Scale][Scale].
    /// Note is either the root of the scale or the octave.
    fn to_scale(self, note: Note) -> Scale;
}

impl<T: AsScale> ToScale for T{
    fn to_scale(self, note: Note) -> Scale{
        self.as_scale(note)
    }
}

/// Try to convert to [Scale][Scale].
pub trait AsScaleTry{
    /// Borrow self and try to return [Scale][Scale].
    /// Note is either the root of the scale or the octave.
    fn as_scale_try(&self, note: Note) -> Option<Scale>;
}

/// Try to convert to [Scale][Scale].
pub trait ToScaleTry{
    /// Take self and try to return [Scale][Scale].
    /// Note is either the root of the scale or the octave.
    fn to_scale_try(self, note: Note) -> Option<Scale>;
}

impl<T: AsScaleTry> ToScaleTry for T{
    fn to_scale_try(self, note: Note) -> Option<Scale>{
        self.as_scale_try(note)
    }
}

/// Convert to [Steps][Steps].
pub trait AsSteps{
    /// Borrow self and return [Steps][Steps].
    /// `complete_octave_cycle` instructs whether the steps should wrap around the starting note to
    /// complete the octave.
    fn as_steps(&self, complete_octave_cycle: bool) -> Steps;
}

/// Convert to [Steps][Steps].
pub trait ToSteps{
    /// Take self and return [Steps][Steps].
    /// `complete_octave_cycle` instructs whether the steps should wrap around the starting note to
    /// complete the octave.
    fn to_steps(self, complete_octave_cycle: bool) -> Steps;
}

impl<T: AsSteps> ToSteps for T{
    fn to_steps(self, complete_octave_cycle: bool) -> Steps{
        self.as_steps(complete_octave_cycle)
    }
}

/// Try convert to [Steps][Steps].
pub trait AsStepsTry{
    /// Borrow self and try to return [Steps][Steps].
    /// `complete_octave_cycle` instructs whether the steps should wrap around the starting note to
    /// complete the octave.
    fn as_steps_try(&self, complete_octave_cycle: bool) -> Option<Steps>;
}

/// Try convert to [Steps][Steps].
pub trait ToStepsTry{
    /// Take self and try to return [Steps][Steps].
    /// `complete_octave_cycle` instructs whether the steps should wrap around the starting note to
    /// complete the octave.
    fn to_steps_try(self, complete_octave_cycle: bool) -> Option<Steps>;
}

impl<T: AsStepsTry> ToStepsTry for T{
    fn to_steps_try(self, complete_octave_cycle: bool) -> Option<Steps>{
        self.as_steps_try(complete_octave_cycle)
    }
}

/// Convert to PCs.
pub trait AsPCs{
    /// Borrow self and return [PCs][PCs].
    fn as_pcs(&self) -> PCs;
}

/// Convert to PCs.
pub trait ToPCs{
    /// Take self and return [PCs][PCs].
    fn to_pcs(self) -> PCs;
}

impl<T: AsPCs> ToPCs for T{
    fn to_pcs(self) -> PCs{
        self.as_pcs()
    }
}

/// Try to convert to relative intervals.
pub trait AsRelativeIntervals{
    /// Borrow self and a reference and compare piecewise.
    /// Try to return the differences as [Interval][Interval]'s.
    /// Inputs must be of equal length.
    fn as_relative_intervals(&self, reference: &Self) -> Option<Intervals>;
}

/// Try to convert to relative intervals.
pub trait ToRelativeIntervals{
    /// Borrow self and a reference and compare piecewise.
    /// Try to return the differences as [Interval][Interval]'s.
    /// Inputs must be of equal length.
    fn to_relative_intervals(self, reference: &Self) -> Option<Intervals>;
}

impl<T: AsRelativeIntervals> ToRelativeIntervals for T{
    fn to_relative_intervals(self, reference: &Self) -> Option<Intervals>{
        self.as_relative_intervals(reference)
    }
}

/// Try to convert to an Ionian relative string.
/// For example the Ionian relative string of Phrygian is 1 ♭2, ♭3, 4, 5, ♭6, ♭7.
pub trait AsIonianRelativeStringTry{
    /// Borrow self and try to return a Ionian relative string.
    /// `nonnat` determines if natural intervals have the natural '♮' accidental prefixed.
    fn as_ionian_relative_string_try(&self, nonnat: bool) -> Option<String>;
}

/// Try to convert to an Ionian relative string.
/// For example the Ionian relative string of Phrygian is 1 ♭2, ♭3, 4, 5, ♭6, ♭7.
pub trait ToIonianRelativeStringTry{
    /// Take self and try to return a Ionian relative string.
    /// `nonnat` determines if natural intervals have the natural '♮' accidental prefixed.
    fn to_ionian_relative_string_try(self, nonnat: bool) -> Option<String>;
}

impl<T: AsIonianRelativeStringTry> ToIonianRelativeStringTry for T{
    fn to_ionian_relative_string_try(self, nonnat: bool) -> Option<String>{
        self.as_ionian_relative_string_try(nonnat)
    }
}

/// Convert to a vector of [EnharmonicNote][EnharmonicNote].
pub trait AsEnharmonicNotes{
    /// Borrow self and return a vector of [EnharmonicNote][EnharmonicNote].
    fn as_enharmonic_notes(&self) -> Vec<EnharmonicNote>;
}

/// Convert to a vector of [EnharmonicNote][EnharmonicNote].
pub trait ToEnharmonicNotes{
    /// Take self and return a vector of [EnharmonicNote][EnharmonicNote].
    fn to_enharmonic_notes(self) -> Vec<EnharmonicNote>;
}

impl<T: AsEnharmonicNotes> ToEnharmonicNotes for T{
    fn to_enharmonic_notes(self) -> Vec<EnharmonicNote>{
        self.as_enharmonic_notes()
    }
}

/// Convert to a vector of [EnharmonicNote][EnharmonicNote].
pub trait AsEnharmonicNotesWithStart{
    /// Borrow self and return a vector of [EnharmonicNote][EnharmonicNote].
    /// You can optionally specify a starting note.
    /// This will affect the spelling.
    /// Subsequent notes will be spelled with subsequent letters.
    fn as_enharmonic_notes_with_start(&self, start: Option<EnharmonicNote>) -> Vec<EnharmonicNote>;
}

/// Convert to a vector of [EnharmonicNote][EnharmonicNote].
pub trait ToEnharmonicNotesWithStart{
    /// Take self and return a vector of [EnharmonicNote][EnharmonicNote].
    /// You can optionally specify a starting note.
    /// This will affect the spelling.
    /// Subsequent notes will be spelled with subsequent letters.
    fn to_enharmonic_notes_with_start(self, start: Option<EnharmonicNote>) -> Vec<EnharmonicNote>;
}

impl<T: AsEnharmonicNotesWithStart> ToEnharmonicNotesWithStart for T{
    fn to_enharmonic_notes_with_start(self, start: Option<EnharmonicNote>) -> Vec<EnharmonicNote>{
        self.as_enharmonic_notes_with_start(start)
    }
}

/// Convert to [Chord][Chord].
pub trait AsChord{
    /// Borrow self and return a [Chord][Chord].
    fn as_chord(&self) -> Chord;
}

/// Convert to [Chord][Chord].
pub trait ToChord{
    /// Take self and return [Chord][Chord].
    fn to_chord(self) -> Chord;
}

impl<T: AsChord> ToChord for T{
    fn to_chord(self) -> Chord{
        self.as_chord()
    }
}

/// Convert to [RootedChord][RootedChord].
pub trait AsRootedChord{
    /// Borrow self and return a [RootedChord][RootedChord].
    fn as_rooted_chord(&self) -> RootedChord;
}

/// Convert to [RootedChord][RootedChord].
pub trait ToRootedChord{
    /// Take self and return a [RootedChord][RootedChord].
    fn to_rooted_chord(self) -> RootedChord;
}

impl<T: AsRootedChord> ToRootedChord for T{
    fn to_rooted_chord(self) -> RootedChord{
        self.as_rooted_chord()
    }
}

