#![deny(missing_docs)]

use super::traits::{
    ToNote, ToPC, OctaveShiftable, GeneratablePartialOrder, AddInterval, ToLetterTry,
    ToEnharmonicNote, Wrapper
};
use super::{ Interval, _OCTAVE, PC, Letter, EnharmonicNote };
use crate::utils::{ impl_op, impl_op_assign };

use std::ops::{ Add, Sub, Mul, Rem, AddAssign, RemAssign, MulAssign, RangeBounds, Bound };

// only used internally
pub(crate) type _Note = u32;
/// Octave.
/// ```
/// use music_theory::theory::*;
/// assert_eq!(Note::A4.with_octave(5), Note::A5);
/// ```
pub type Octave = u16;
/// Can be negative to shift down into the octave range.
/// ```
/// use music_theory::theory::*;
/// assert_eq!(Note::A4.shift_octave(-1), Note::A3);
/// ```
pub type OctaveShift = i16;

/// The main note type.
/// Does not take into account enharmonic spelling.
/// ```
/// use music_theory::theory::*;
/// assert_eq!(Note::new(0), Note::A0);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Note(pub(crate) u32);

macro_rules! define_notes{
    ($octave: expr,
        $an: ident, $as: ident, $bn: ident, $cn: ident, $cs: ident, $dn: ident, $ds: ident,
        $en: ident, $fn: ident, $fs: ident, $gn: ident, $gs: ident
    ) => {
        #[allow(missing_docs)] pub const $an: Note = Note(0 + $octave * 12);
        #[allow(missing_docs)] pub const $as: Note = Note(1 + $octave * 12);
        #[allow(missing_docs)] pub const $bn: Note = Note(2 + $octave * 12);
        #[allow(missing_docs)] pub const $cn: Note = Note(3 + $octave * 12);
        #[allow(missing_docs)] pub const $cs: Note = Note(4 + $octave * 12);
        #[allow(missing_docs)] pub const $dn: Note = Note(5 + $octave * 12);
        #[allow(missing_docs)] pub const $ds: Note = Note(6 + $octave * 12);
        #[allow(missing_docs)] pub const $en: Note = Note(7 + $octave * 12);
        #[allow(missing_docs)] pub const $fn: Note = Note(8 + $octave * 12);
        #[allow(missing_docs)] pub const $fs: Note = Note(9 + $octave * 12);
        #[allow(missing_docs)] pub const $gn: Note = Note(10 + $octave * 12);
        #[allow(missing_docs)] pub const $gs: Note = Note(11 + $octave * 12);
    }
}

impl Note{
    /// Biggest valid `Note` value.
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(Note::MAX + Note::C3, Note::MAX);
    /// ```
    pub const MAX: Note = Note(1073741824); // 1 << 30
    /// Smallest valid `Note` value.
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(Note::MIN.shift_octave(-1), Note::MIN);
    /// ```
    pub const MIN: Note = Note(0);
    /// The zeroth `Note`, the same as `Note::MIN`.
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(Note::ZERO, Note::MIN);
    /// ```
    pub const ZERO: Note = Note(0);

    define_notes!(0, A0, AS0, B0, C0, CS0, D0, DS0, E0, F0, FS0, G0, GS0);
    define_notes!(1, A1, AS1, B1, C1, CS1, D1, DS1, E1, F1, FS1, G1, GS1);
    define_notes!(2, A2, AS2, B2, C2, CS2, D2, DS2, E2, F2, FS2, G2, GS2);
    define_notes!(3, A3, AS3, B3, C3, CS3, D3, DS3, E3, F3, FS3, G3, GS3);
    define_notes!(4, A4, AS4, B4, C4, CS4, D4, DS4, E4, F4, FS4, G4, GS4);
    define_notes!(5, A5, AS5, B5, C5, CS5, D5, DS5, E5, F5, FS5, G5, GS5);
    define_notes!(6, A6, AS6, B6, C6, CS6, D6, DS6, E6, F6, FS6, G6, GS6);
    define_notes!(7, A7, AS7, B7, C7, CS7, D7, DS7, E7, F7, FS7, G7, GS7);
    define_notes!(8, A8, AS8, B8, C8, CS8, D8, DS8, E8, F8, FS8, G8, GS8);
    define_notes!(9, A9, AS9, B9, C9, CS9, D9, DS9, E9, F9, FS9, G9, GS9);

    /// Create a new valid note.
    /// Will be clamped if nessesary.
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(Note::new(12), Note::A1);
    /// ```
    pub fn new(note: u32) -> Self{
        Self(note.min(Self::MAX.0))
    }

    /// Returns the inner value.
    /// You can not set the inner value directly to prevent invalid notes.
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(Note::A2.inside(), 24);
    /// ```
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

    /// Return the pitch in hertz.
    /// `Note::A4` has an inner value of 48 and a pitch of 440 hz.
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(Note::A4.to_pitch(), 440.0);
    /// ```
    pub fn to_pitch(&self) -> f32{
        let x = self.0 as i32 - 48;
        (2.0f32).powf(x as f32 / _OCTAVE.0 as f32) * 440.0f32
    }
}

// General implementations

impl Sub for Note{
    type Output = Interval;

    fn sub(self, other: Self) -> Interval{
        Interval(self.0 as i32 - other.0 as i32)
    }
}

impl_op!(Add, Note, Note, add, add, Self::new);
impl_op!(Rem, Note, Note, rem, rem, Self);
impl_op!(Mul, Note, Note, mul, mul, Self::new);
impl_op_assign!(AddAssign, Note, add_assign, add);
impl_op_assign!(RemAssign, Note, rem_assign, rem);
impl_op_assign!(MulAssign, Note, mul_assign, mul);

// Is tested but tarpaulin doesn't see it?
#[cfg(not(tarpaulin))]
impl RangeBounds<Note> for Note{
    fn start_bound(&self) -> Bound<&Self>{
        Bound::Included(&Self::MIN)
    }

    fn end_bound(&self) -> Bound<&Self>{
        Bound::Excluded(&Self::MAX)
    }
}

impl Wrapper for Note{
    type Inner = _Note;

    fn wrap(note: Self::Inner) -> Option<Self>{
        if note > Self::MAX.0 || note < Self::MIN.0{
            None
        } else {
            Some(Self(note))
        }
    }

    fn unwrap(self) -> Self::Inner{
        self.0
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
        (((self.0 % _OCTAVE.0) as i32 + octave as i32 * _OCTAVE.0 as i32) as _Note).to_note()
    }

    fn shift_octave(self, shift: OctaveShift) -> Note{
        ((self.0 as i32 + shift as i32 * _OCTAVE.0 as i32).max(0) as _Note).to_note()
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

impl ToEnharmonicNote for Note{
    fn to_enharmonic_note(self) -> EnharmonicNote{
        self.to_pc().to_enharmonic_note()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::theory::*;

    #[test]
    fn to_pitch(){
        assert_eq!(Note::A4.to_pitch().round() as i32, 440);
    }

    #[test]
    fn new(){
        assert_eq!(Note::MAX, Note::new(Note::MAX.0 + 1));
    }

    #[test]
    fn wrap(){
        assert_eq!(Note::wrap(0), Some(Note(0)));
        assert_eq!(Note::wrap(Note::MAX.0), Some(Note::MAX));
        assert_eq!(Note::wrap(Note::MIN.0), Some(Note::MIN));
        assert_eq!(Note::wrap(Note::MAX.0 + 1), None);
    }

    #[test]
    fn unwrap(){
        assert_eq!(Note(0).unwrap(), 0);
    }

    #[test]
    fn add(){
        assert_eq!(Note(0) + Note(0), Note(0));
        assert_eq!(Note(1) + Note(0), Note(1));
        assert_eq!(_SEMI + _WHOLE, _MIN3);
        assert_eq!(Note::MAX + Note(1), Note::MAX);
    }

    #[test]
    fn add_assign(){
        for x in 0..123{
        for y in 0..123{
            let x = Note(x);
            let y = Note(y);
            let mut z = x;
            z += y;
            assert_eq!(z, x + y);
        }
        }
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
    fn rem(){
        assert_eq!(Note(13) % Note(12), Note(1));
    }

    #[test]
    fn rem_assign(){
        for x in 0..123{
        for y in 1..123{
            let x = Note(x);
            let y = Note(y);
            let mut z = x;
            z %= y;
            assert_eq!(z, x % y);
        }
        }
    }

    #[test]
    fn mul(){
        assert_eq!(Note(2) * Note(2) * Note(2), Note(8));
    }

    #[test]
    fn mul_assign(){
        for x in 0..123{
        for y in 0..123{
            let x = Note(x);
            let y = Note(y);
            let mut z = x;
            z *= y;
            assert_eq!(z, x * y);
        }
        }
    }

    #[test]
    fn range_bounds(){
        assert_eq!((Note::F1..Note::C2).start_bound(), Bound::Included(&Note::F1));
        assert_eq!((Note::F1..Note::C2).end_bound(), Bound::Excluded(&Note::C2));
        assert!((Note::F1..Note::C2).contains(&Note::F1));
        assert!((Note::F1..Note::C2).contains(&Note::G1));
        assert!(!(Note::F1..Note::C2).contains(&Note::C2));
        // assert_eq!(
        //     (Note::A1..Note::C1).collect::<Vec<_>>(),
        //     vec![Note::A1, Note::AS1, Note::B1]
        // );
    }

    #[test]
    fn u32_to_note(){
        for i in 0..12345{
            assert_eq!(Note::new(i), i.to_note());
        }
    }

    #[test]
    fn note_to_pc(){
        assert_eq!(Note::A4.to_pc(), PC::A);
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
            assert!(Note::MAX.with_octave(u16::MAX) < Note::MAX);
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

    #[test]
    fn to_enharmonic_note(){
        for i in 0..123{
            assert_eq!(Note(i).to_pc(), Note(i).to_enharmonic_note().to_pc());
        }
    }
}
