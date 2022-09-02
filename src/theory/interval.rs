#![deny(missing_docs)]

use super::traits::{
    GeneratablePartialOrder, OctaveShiftable, AddInterval, ToInterval, ToNamedInterval,
    Cyclic, ToNamedOctaveInterval, Wrapper, AsIonianRelativeStringTry
};
use super::note::{ Note, Octave, OctaveShift };
use crate::utils::{ impl_op, impl_op_assign };

use std::cmp::Ordering;
use std::ops::{ Add, Sub, Bound, RangeBounds };
use std::fmt::Write;

pub(crate) const _SEMI: Note = Note(1);
pub(crate) const _WHOLE: Note = Note(2);

pub(crate) const _ROOT: Note = Note(0);
pub(crate) const _MIN2: Note = Note(1);
pub(crate) const _MAJ2: Note = Note(2);
pub(crate) const _MIN3: Note = Note(3);
pub(crate) const _MAJ3: Note = Note(4);
pub(crate) const _PER4: Note = Note(5);
pub(crate) const _TRIT: Note = Note(6);
pub(crate) const _PER5: Note = Note(7);
pub(crate) const _MIN6: Note = Note(8);
pub(crate) const _MAJ6: Note = Note(9);
pub(crate) const _MIN7: Note = Note(10);
pub(crate) const _MAJ7: Note = Note(11);
pub(crate) const _OCTAVE: Note = Note(12);
pub(crate) const _MIN9: Note = Note(13);
pub(crate) const _MAJ9: Note = Note(14);
pub(crate) const _AUG9: Note = Note(15);
pub(crate) const _MIN11: Note = Note(16);
pub(crate) const _MAJ11: Note = Note(17);
pub(crate) const _AUG11: Note = Note(18);
pub(crate) const _PER12: Note = Note(19);
pub(crate) const _MIN13: Note = Note(20);
pub(crate) const _MAJ13: Note = Note(21);
pub(crate) const _AUG13: Note = Note(22);

pub(crate) const _DIM2: Note = Note(0);
pub(crate) const _AUG1: Note = Note(1);
pub(crate) const _DIM3: Note = Note(2);
pub(crate) const _AUG2: Note = Note(3);
pub(crate) const _DIM4: Note = Note(4);
pub(crate) const _AUG3: Note = Note(5);
pub(crate) const _DIM5: Note = Note(6);
pub(crate) const _AUG4: Note = Note(6);
pub(crate) const _DIM6: Note = Note(7);
pub(crate) const _AUG5: Note = Note(8);
pub(crate) const _DIM7: Note = Note(9);
pub(crate) const _AUG6: Note = Note(10);
pub(crate) const _DIM8: Note = Note(11);
pub(crate) const _AUG7: Note = Note(12);

/// An interval is a distance between notes.
///
/// Example:
/// ```
/// use music_theory::theory::*;
/// assert_eq!(Note::C2 - Note::A2, Interval::new(3));
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interval(pub(crate) i32);

/// Type defined for convenience.
pub type Intervals = Vec<Interval>;

/// An interval with a distinct name
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NamedInterval{
    /// The root interval is an interval of zero.
    Root = 0, /// Minor second interval.
    Min2 = 1, /// Major second interval.
    Maj2 = 2, /// Minor third interval.
    Min3 = 3, /// Major third interval.
    Maj3 = 4, /// Perfect fourth interval.
    Per4 = 5, /// Tritone interval.
    Trit = 6, /// Perfect fifth interval.
    Per5 = 7, /// Minor sixth interval.
    Min6 = 8, /// Major sixth interval.
    Maj6 = 9, /// Minor seventh interval.
    Min7 = 10, /// Major seventh interval.
    Maj7 = 11, /// Perfect eight interval, same as an octave.
    Per8 = 12, /// Minor nineth interval, octave equivalent to a minor second.
    Min9 = 13, /// Major nineth interval, octave equivalent to a major second.
    Maj9 = 14, /// Augmented nineth interval, octave equivalent to a minor third.
    Aug9 = 15, /// Minor eleventh interval, octave equivalent to a major third.
    Min11 = 16, /// Major eleventh interval, octave equivalent to a perfect fourth.
    Maj11 = 17, /// Augmented eleventh interval, octave equivalent to a tritone.
    Aug11 = 18, /// Minor thirteenth interval, octave equivalent to a minor sixth.
    Min13 = 20, /// Major thirteenth interval, octave equivalent to a major sixth.
    Maj13 = 21, /// Augmented thirteenth interval, octave equivalent to a minor seventh.
    Aug13 = 22,
}

/// An interval within the octave with a distinct name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NamedOctaveInterval{
    /// The root interval is an interval of zero.
    Root = 0, /// Minor second interval.
    Min2 = 1, /// Major second interval.
    Maj2 = 2, /// Minor third interval.
    Min3 = 3, /// Major third interval.
    Maj3 = 4, /// Perfect fourth interval.
    Per4 = 5, /// Tritone interval.
    Trit = 6, /// Perfect fifth interval.
    Per5 = 7, /// Minor sixth interval.
    Min6 = 8, /// Major sixth interval.
    Maj6 = 9, /// Minor seventh interval.
    Min7 = 10, /// Major seventh interval.
    Maj7 = 11,
}

impl Interval{
    /// The biggest valid interval.
    pub const MAX: Self = Self(1 << 30);
    /// The smallest valid interval.
    pub const MIN: Self = Self(-1 << 30);

    /// One semi tone
    pub const SEMI: Self = Self(1);
    /// One whole tone
    pub const WHOLE: Self = Self(2);

    /// The interval equivalent of two flats on a note.
    pub const FLAT2: Self = Self(-2);
    /// The interval equivalent of a flat on a note.
    pub const FLAT: Self = Self(-1);
    /// The interval equivelant of a natural accidental.
    pub const NAT: Self = Self(0);
    /// The interval equivelant of a sharp on a note.
    pub const SHARP: Self = Self(1);
    /// The interval equivalent of two sharps on a note.
    pub const SHARP2: Self = Self(2);

    /// The root interval, a distance of zero..
    pub const ROOT: Self = Self(0); /// Minor second.
    pub const MIN2: Self = Self(1); /// Major second.
    pub const MAJ2: Self = Self(2); /// Minor third.
    pub const MIN3: Self = Self(3); /// Major third.
    pub const MAJ3: Self = Self(4); /// Perfect fourth.
    pub const PER4: Self = Self(5); /// Tritone.
    pub const TRIT: Self = Self(6); /// Perfect fifth.
    pub const PER5: Self = Self(7); /// Minor sixth.
    pub const MIN6: Self = Self(8); /// Major sixth.
    pub const MAJ6: Self = Self(9); /// Minor seventh.
    pub const MIN7: Self = Self(10); /// Major seventh.
    pub const MAJ7: Self = Self(11); /// Octave.
    pub const OCTAVE: Self = Self(12); /// Minor nineth.
    pub const MIN9: Self = Self(13); /// Major nineth.
    pub const MAJ9: Self = Self(14); /// Augmented nineth.
    pub const AUG9: Self = Self(15); /// Minor eleventh.
    pub const MIN11: Self = Self(16); /// Major eleventh.
    pub const MAJ11: Self = Self(17); /// Augmented eleventh.
    pub const AUG11: Self = Self(18); /// Perfect twelve, octave equivalent of a perfect fifth.
    pub const PER12: Self = Self(19); /// Minor thirteenth.
    pub const MIN13: Self = Self(20); /// Major thirteenth.
    pub const MAJ13: Self = Self(21); /// Augmented thirteenth.
    pub const AUG13: Self = Self(22);

    /// Diminished second, octave equivalent of the root.
    pub const DIM2: Self = Self(0); /// Augmented first, octave equivalent of a minor second.
    pub const AUG1: Self = Self(1); /// Diminished third, octave equivalent of a major second.
    pub const DIM3: Self = Self(2); /// Augmented second, octave equivalent of a minor third.
    pub const AUG2: Self = Self(3); /// Diminished fourth, octave equivalent of a major third.
    pub const DIM4: Self = Self(4); /// Augmented third, octave equivalent of a perfect fourth.
    pub const AUG3: Self = Self(5); /// Diminished fifth, octave equivalent of a tritone.
    pub const DIM5: Self = Self(6); /// Augmented fourth, octave equivalent of a tritone.
    pub const AUG4: Self = Self(6); /// Diminished sixth, octave equivalent of a perfect fifth.
    pub const DIM6: Self = Self(7); /// Augmented fifth, octave equivalent of a minor sixth.
    pub const AUG5: Self = Self(8); /// Diminished seventh, octave equivalent of a major sixth.
    pub const DIM7: Self = Self(9); /// Augmented sixth, octave equivalent of a minor seventh.
    pub const AUG6: Self = Self(10); /// Diminished eight, octave equivalent of a major seventh.
    pub const DIM8: Self = Self(11); /// Augmented seventh, octave equivalent of an octave.
    pub const AUG7: Self = Self(12);

    /// Create a new interval from an integer.
    /// Will be clamped if needed to ensure the interval is valid.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(Note::B2 - Note::A2, Interval::new(2));
    /// ```
    pub fn new(i: i32) -> Self{
        Self(i.min(Self::MAX.0).max(Self::MIN.0))
    }

    /// Create a new interval from an integer.
    /// If the integer is out of range of valid intervals, it will return `None`.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(Interval::new_try(7), Some(Interval::PER5));
    /// assert_eq!(Interval::new_try((1 << 30) + 1), None)
    /// ```
    pub fn new_try(i: i32) -> Option<Self>{
        if i > Self::MAX.0 || i < Self::MIN.0 {
            None
        } else {
            Some(Self(i))
        }
    }

    /// Returns the absolute value of the interval.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(Interval::FLAT2.abs(), Interval::SHARP2);
    /// ```
    pub fn abs(self) -> Self{
        Self::new(self.0.abs())
    }

    /// Returns the interval spelled out in accidentals.
    /// Will spell the zero interval with a natural accidental.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(&Interval::FLAT2.to_string_nat(), "♭♭");
    /// assert_eq!(&Interval::ROOT.to_string_nat(), "♮");
    /// ```
    pub fn to_string_nat(&self) -> String{
        let mut string = String::new();
        let i = self.0.abs();
        match self.0.cmp(&0){
            Ordering::Less    => { for _ in 0..i { string.push('♭'); } },
            Ordering::Greater => { for _ in 0..i { string.push('♯'); } },
            Ordering::Equal   => { string.push('♮'); },
        }
        string
    }
}

impl NamedInterval{
    /// All named intervals so you iterate over them.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(NamedInterval::ALL.iter().copied().next(), Some(NamedInterval::Root));
    /// ```
    pub const ALL: [Self; 22] = [
        Self::Root, Self::Min2, Self::Maj2, Self::Min3, Self::Maj3,
        Self::Per4, Self::Trit, Self::Per5, Self::Min6, Self::Maj6,
        Self::Min7, Self::Maj7, Self::Per8, Self::Min9, Self::Maj9,
        Self::Aug9, Self::Min11, Self::Maj11, Self::Aug11,
        Self::Min13, Self::Maj13, Self::Aug13,
    ];
}

impl NamedOctaveInterval{
    /// All named octave intervals so you iterate over them.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// assert_eq!(NamedOctaveInterval::ALL.iter().copied().next(), Some(NamedOctaveInterval::Root));
    /// ```
    pub const ALL: [Self; 12] = [
        Self::Root, Self::Min2, Self::Maj2,
        Self::Min3, Self::Maj3, Self::Per4,
        Self::Trit, Self::Per5, Self::Min6,
        Self::Maj6, Self::Min7, Self::Maj7
    ];
}

pub(crate) fn _interval_mod(i: i32) -> i32{
    ((i % 12) + 12) % 12
}

impl std::ops::Neg for Interval{
    type Output = Self;

    fn neg(self) -> Self{
        Self::new(-self.0)
    }
}

impl_op!(std::ops::Add, Interval, Interval, add, add, Self::new);
impl_op!(std::ops::Sub, Interval, Interval, sub, sub, Self::new);
impl_op_assign!(std::ops::AddAssign, Interval, add_assign, add);
impl_op_assign!(std::ops::SubAssign, Interval, sub_assign, sub);


// Is tested but tarpaulin doesn't see it?
#[cfg(not(tarpaulin))]
impl RangeBounds<Interval> for Interval{
    fn start_bound(&self) -> Bound<&Self>{
        Bound::Included(&Self::MIN)
    }

    fn end_bound(&self) -> Bound<&Self>{
        Bound::Included(&Self::MAX)
    }
}

impl std::fmt::Display for Interval{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let mut string = String::new();
        let i = self.0.abs();
        match self.0.cmp(&0){
            Ordering::Less    => { for _ in 0..i { string.push('♭'); } },
            Ordering::Greater => { for _ in 0..i { string.push('♯'); } },
            Ordering::Equal   => {  }
        }
        write!(f, "{}", string)
    }
}

impl std::fmt::Display for NamedInterval{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let names = [
            "R", "♭2", "♮2", "♭3", "♮3", "♮4", "♭5", "♮5", "♭6", "♮6", "♭7", "♮7", "♮8",
            "♭9", "♮9", "♯9", "♭11", "♮11", "♯11", "", "♭13", "♮13", "♯13",
        ];
        write!(f, "{}", names[*self as usize])
    }
}

impl std::fmt::Display for NamedOctaveInterval{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let names = [
            "R", "♭2", "♮2", "♭3", "♮3", "♮4", "♭5", "♮5", "♭6", "♮6", "♭7", "♮7",
        ];
        write!(f, "{}", names[*self as usize])
    }
}

impl Wrapper for Interval{
    type Inner = i32;

    fn wrap(interval: Self::Inner) -> Option<Self>{
        if interval > Self::MAX.0 || interval < Self::MIN.0{
            None
        } else {
            Some(Self(interval))
        }
    }

    fn unwrap(self) -> Self::Inner{
        self.0
    }
}

impl GeneratablePartialOrder for Interval{
    fn next(self) -> Option<Interval>{
        if self.0 >= Self::MAX.0 { return None; }
        Some(Self(self.0 + 1))
    }

    fn prev(self) -> Option<Interval>{
        if self.0 <= Self::MIN.0 { return None; }
        Some(Self(self.0 - 1))
    }
}

fn _named_octave_interval_from_int(i: i32) -> NamedOctaveInterval{
    match i{
        0 => NamedOctaveInterval::Root,
        1 => NamedOctaveInterval::Min2,
        2 => NamedOctaveInterval::Maj2,
        3 => NamedOctaveInterval::Min3,
        4 => NamedOctaveInterval::Maj3,
        5 => NamedOctaveInterval::Per4,
        6 => NamedOctaveInterval::Trit,
        7 => NamedOctaveInterval::Per5,
        8 => NamedOctaveInterval::Min6,
        9 => NamedOctaveInterval::Maj6,
        10 => NamedOctaveInterval::Min7,
        11 => NamedOctaveInterval::Maj7,
        _ => panic!("theory::interval::_named_octave_interval_from_int(i32): should be impossible!"),
    }
}

impl Cyclic for NamedOctaveInterval{
    fn next(self) -> Self{
        _named_octave_interval_from_int(_interval_mod(self as i32 + 1))
    }

    fn prev(self) -> Self{
        _named_octave_interval_from_int(_interval_mod(self as i32 - 1))
    }
}

impl OctaveShiftable for Interval{
    fn with_octave(self, octave: Octave) -> Self{
        match self.0.cmp(&0){
            Ordering::Less =>
                Interval((self.0 % _OCTAVE.0 as i32) - octave as i32 * _OCTAVE.0 as i32),
            Ordering::Equal =>
                Interval(0),
            Ordering::Greater =>
                Interval((self.0 % _OCTAVE.0 as i32) + octave as i32 * _OCTAVE.0 as i32),
        }
    }

    fn shift_octave(self, shift: OctaveShift) -> Self{
        Interval::new(self.0 + shift as i32 * _OCTAVE.0 as i32)
    }
}

impl AddInterval for Interval{
    fn add_interval(self, interval: Interval) -> Option<Self>{
        let res = (self.0 as i32).checked_add(interval.0)?;
        match res < Self::MIN.0 || res > Self::MAX.0{
            true => None,
            false => Some(Self(res)),
        }
    }
}

// Conversion Traits

impl ToInterval for NamedInterval{
    fn to_interval(self) -> Interval{
        Interval(self as i32)
    }
}

impl ToInterval for NamedOctaveInterval{
    fn to_interval(self) -> Interval{
        Interval(self as i32)
    }
}

impl ToNamedInterval for Interval{
    fn to_named_interval_try(self) -> Option<NamedInterval>{
        match self.0{
            0 => Some(NamedInterval::Root),
            1 => Some(NamedInterval::Min2),
            2 => Some(NamedInterval::Maj2),
            3 => Some(NamedInterval::Min3),
            4 => Some(NamedInterval::Maj3),
            5 => Some(NamedInterval::Per4),
            6 => Some(NamedInterval::Trit),
            7 => Some(NamedInterval::Per5),
            8 => Some(NamedInterval::Min6),
            9 => Some(NamedInterval::Maj6),
            10 => Some(NamedInterval::Min7),
            11 => Some(NamedInterval::Maj7),
            12 => Some(NamedInterval::Per8),
            13 => Some(NamedInterval::Min9),
            14 => Some(NamedInterval::Maj9),
            15 => Some(NamedInterval::Aug9),
            16 => Some(NamedInterval::Min11),
            17 => Some(NamedInterval::Maj11),
            18 => Some(NamedInterval::Aug11),
            20 => Some(NamedInterval::Min13),
            21 => Some(NamedInterval::Maj13),
            22 => Some(NamedInterval::Aug13),
            _ => None,
        }
    }

    fn to_named_interval_mod(self) -> NamedInterval{
        let int = Self(self.0 % 24).to_named_interval_try();
        match int{
            Some(i) => i,
            None => Self(_interval_mod(self.0)).to_named_interval_try().unwrap()
        }
    }
}

impl<T> ToNamedInterval for T where T: ToInterval{
    fn to_named_interval_try(self) -> Option<NamedInterval>{
        self.to_interval().to_named_interval_try()
    }

    fn to_named_interval_mod(self) -> NamedInterval{
        self.to_interval().to_named_interval_mod()
    }
}

impl ToNamedOctaveInterval for Interval{
    fn to_named_octave_interval_try(self) -> Option<NamedOctaveInterval>{
        match self.0{
            0 => Some(NamedOctaveInterval::Root),
            1 => Some(NamedOctaveInterval::Min2),
            2 => Some(NamedOctaveInterval::Maj2),
            3 => Some(NamedOctaveInterval::Min3),
            4 => Some(NamedOctaveInterval::Maj3),
            5 => Some(NamedOctaveInterval::Per4),
            6 => Some(NamedOctaveInterval::Trit),
            7 => Some(NamedOctaveInterval::Per5),
            8 => Some(NamedOctaveInterval::Min6),
            9 => Some(NamedOctaveInterval::Maj6),
            10 => Some(NamedOctaveInterval::Min7),
            11 => Some(NamedOctaveInterval::Maj7),
            _ => None,
        }
    }

    fn to_named_octave_interval_mod(self) -> NamedOctaveInterval{
        Self(_interval_mod(self.0 % 12)).to_named_octave_interval_try().unwrap()
    }
}

impl<T> ToNamedOctaveInterval for T where T: ToInterval{
    fn to_named_octave_interval_try(self) -> Option<NamedOctaveInterval>{
        self.to_interval().to_named_octave_interval_try()
    }

    fn to_named_octave_interval_mod(self) -> NamedOctaveInterval{
        self.to_interval().to_named_octave_interval_mod()
    }
}

impl AsIonianRelativeStringTry for Intervals{
    fn as_ionian_relative_string_try(&self, nonnat: bool) -> Option<String>{
        if self.len() != 7{
            return None;
        }
        let mut res = String::new();
        for (i, int) in self.iter().enumerate().take(7){
            let prefix = if nonnat { int.to_string() } else { int.to_string_nat() };
            res.push_str(&prefix);
            let _ = write!(res, "{} ", i + 1);
        }
        res.pop();
        Some(res)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::theory::*;

    #[test]
    fn interval_mod(){
        assert_eq!(_interval_mod(-1), 11);
        assert_eq!(_interval_mod(12), 0);
        assert_eq!(_interval_mod(13), 1);
        for i in 0..12{
            assert_eq!(i, _interval_mod(i));
        }
    }

    #[test]
    fn named_octave_interval_from_int(){
        for i in 0..12{
            assert_eq!(i, _named_octave_interval_from_int(i) as i32);
        }
    }

    #[test]
    #[should_panic]
    fn named_octave_interval_from_int_panic_0(){
        let _ = _named_octave_interval_from_int(-1);
    }

    #[test]
    #[should_panic]
    fn named_octave_interval_from_int_panic_1(){
        let _ = _named_octave_interval_from_int(12);
    }

    #[test]
    fn new(){
        assert_eq!(Interval::new(0), Interval(0));
        assert_eq!(Interval::new(1 << 30), Interval::MAX);
        assert_eq!(Interval::new(-1 << 30), Interval::MIN);
        assert_eq!(Interval::new((1 << 30) + 1), Interval::MAX);
        assert_eq!(Interval::new((-1 << 30) - 1), Interval::MIN);
    }

    #[test]
    fn new_try(){
        assert_eq!(Interval::new_try(0), Some(Interval(0)));
        assert_eq!(Interval::new_try(1 << 30), Some(Interval::MAX));
        assert_eq!(Interval::new_try(-1 << 30), Some(Interval::MIN));
        assert_eq!(Interval::new_try((1 << 30) + 1), None);
        assert_eq!(Interval::new_try((-1 << 30) - 1), None);
    }

    #[test]
    fn abs(){
        assert_eq!(Interval(0).abs(), Interval(0));
        assert_eq!(Interval(1).abs(), Interval(1));
        assert_eq!(Interval(-1).abs(), Interval(1));
        assert_eq!(Interval::MAX.abs(), Interval::MAX);
        assert_eq!(Interval::MIN.abs(), Interval::MAX);
    }

    #[test]
    #[allow(clippy::double_neg)]
    fn neg(){
        assert_eq!(-Interval(0), Interval(0));
        assert_eq!(-Interval(1), Interval(-1));
        assert_eq!(--Interval(1), Interval(1));
    }

    #[test]
    fn add(){
        assert_eq!(Interval(0) + Interval(0), Interval(0));
        assert_eq!(Interval(1) + Interval(2), Interval(3));
        assert_eq!(Interval(1) + Interval(-2), Interval(-1));
    }

    #[test]
    fn add_assign(){
        for x in 0..123{
        for y in 0..123{
            let x = Interval(x);
            let y = Interval(y);
            let mut z = x;
            z += y;
            assert_eq!(z, x + y);
        }
        }
    }

    #[test]
    fn sub(){
        assert_eq!(Interval(0) - Interval(0), Interval(0));
        assert_eq!(Interval(1) - Interval(2), Interval(-1));
        assert_eq!(Interval(1) - Interval(-2), Interval(3));
    }

    #[test]
    fn sub_assign(){
        for x in 0..123{
        for y in 0..123{
            let x = Interval(x);
            let y = Interval(y);
            let mut z = x;
            z -= y;
            assert_eq!(z, x - y);
        }
        }
    }

    #[test]
    fn neg_add_sub(){
        assert_eq!(-(Interval(13) + -Interval(24) - Interval(3)), Interval(14));
    }

    #[test]
    fn range_bounds(){
        assert_eq!((Interval(-32)..Interval(32)).start_bound(), Bound::Included(&Interval(-32)));
        assert_eq!((Interval(-32)..Interval(32)).end_bound(), Bound::Excluded(&Interval(32)));
        assert!((Interval(-32)..Interval(32)).contains(&Interval(-32)));
        assert!(!(Interval(-32)..Interval(32)).contains(&Interval(32)));
        assert!((Interval(-32)..Interval(32)).contains(&Interval(-2)));
    }

    #[test]
    fn interval_to_string(){
        assert_eq!(&Interval(0).to_string(), "");
        assert_eq!(&Interval(1).to_string(), "♯");
        assert_eq!(&Interval(10).to_string(), "♯♯♯♯♯♯♯♯♯♯");
        assert_eq!(&Interval(-1).to_string(), "♭");
        assert_eq!(&Interval(-10).to_string(), "♭♭♭♭♭♭♭♭♭♭");
    }

    #[test]
    fn interval_to_string_nat(){
        assert_eq!(&Interval(0).to_string_nat(), "♮");
        assert_eq!(&Interval(1).to_string_nat(), "♯");
        assert_eq!(&Interval(10).to_string_nat(), "♯♯♯♯♯♯♯♯♯♯");
        assert_eq!(&Interval(-1).to_string_nat(), "♭");
        assert_eq!(&Interval(-10).to_string_nat(), "♭♭♭♭♭♭♭♭♭♭");
    }

    #[test]
    fn named_interval_to_string(){
        let names = [
            "R", "♭2", "♮2", "♭3", "♮3", "♮4", "♭5", "♮5", "♭6", "♮6", "♭7", "♮7", "♮8",
            "♭9", "♮9", "♯9", "♭11", "♮11", "♯11", "♭13", "♮13", "♯13",
        ];
        for (ni, n) in NamedInterval::ALL.iter().zip(names.iter()){
            assert_eq!(&ni.to_string(), n);
        }
    }

    #[test]
    fn named_octave_interval_to_string(){
        let names = ["R", "♭2", "♮2", "♭3", "♮3", "♮4", "♭5", "♮5", "♭6", "♮6", "♭7", "♮7"];
        for (ni, n) in NamedOctaveInterval::ALL.iter().zip(names.iter()){
            assert_eq!(&ni.to_string(), n);
        }
    }

    #[test]
    fn wrap(){
        assert_eq!(Interval::wrap(0), Some(Interval(0)));
        assert_eq!(Interval::wrap(Interval::MAX.0), Some(Interval::MAX));
        assert_eq!(Interval::wrap(Interval::MIN.0), Some(Interval::MIN));
        assert_eq!(Interval::wrap(Interval::MAX.0 + 1), None);
        assert_eq!(Interval::wrap(Interval::MIN.0 - 1), None);
    }

    #[test]
    fn unwrap(){
        assert_eq!(Interval(0).unwrap(), 0);
    }

    #[test]
    fn interval_generatable_partial_order(){
        assert_eq!(Interval(0).next(), Some(Interval(1)));
        assert_eq!(Interval(0).prev(), Some(Interval(-1)));
        assert_eq!(Interval::MAX.next(), None);
        assert_eq!(Interval::MIN.prev(), None);
    }

    #[test]
    fn named_octave_interval_cyclic(){
        assert_eq!(NamedOctaveInterval::Root.prev(), NamedOctaveInterval::Maj7);
        assert_eq!(NamedOctaveInterval::Maj7.next(), NamedOctaveInterval::Root);
        for i in 0..11{
            assert_eq!(NamedOctaveInterval::ALL[i].next(), NamedOctaveInterval::ALL[i + 1]);
        }
        for i in 1..12{
            assert_eq!(NamedOctaveInterval::ALL[i].prev(), NamedOctaveInterval::ALL[i - 1]);
        }
    }

    #[test]
    fn octave_shiftable(){
        assert_eq!(Interval(0).with_octave(0), Interval(0));
        assert_eq!(Interval(0).with_octave(1), Interval(0));
        assert_eq!(Interval(1).with_octave(12), Interval(145));
        assert_eq!(Interval(12).with_octave(0), Interval(0));
        assert_eq!(Interval(-12).with_octave(0), Interval(0));
        assert_eq!(Interval(-1).with_octave(2), Interval(-25));
        assert_eq!(Interval(-38).with_octave(1), Interval(-14));
        assert_eq!(Interval(-38).with_octave(0), Interval(-2));
        assert!(Interval(0).with_octave(u16::MAX) < Interval::MAX);
        assert!(Interval(0).with_octave(u16::MAX) < Interval::MAX);
        assert_eq!(Interval(0).shift_octave(2), Interval(24));
        assert_eq!(Interval(0).shift_octave(-2), Interval(-24));
        assert_eq!(Interval(1).shift_octave(-2), Interval(-23));
        assert_eq!(Interval::MAX.shift_octave(i16::MAX), Interval::MAX);
        assert_eq!(Interval::MIN.shift_octave(i16::MIN), Interval::MIN);
    }

    #[test]
    fn add_interval(){
        for x in -123..123{
            for y in -123..123{
                let (ix, iy) = (Interval(x), Interval(y));
                assert_eq!(Some(ix + iy), ix.add_interval(iy));
            }
        }
        assert_eq!(Interval::MAX.add_interval(Interval(1)), None);
        assert_eq!(Interval::MIN.add_interval(Interval(-1)), None);
    }

    #[test]
    fn named_interval_to_interval(){
        assert_eq!(
            NamedInterval::ALL.iter().map(|ni| ni.to_interval()).collect::<Vec<_>>(),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22]
                .iter().map(|i| Interval(*i)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn named_octave_interval_to_interval(){
        assert_eq!(
            NamedOctaveInterval::ALL.iter().map(|ni| ni.to_interval()).collect::<Vec<_>>(),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].iter().map(|i| Interval(*i)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn interval_to_named_interval(){
        assert_eq!(Interval(0).to_named_interval_try(), Some(NamedInterval::Root));
        assert_eq!(Interval(1).to_named_interval_try(), Some(NamedInterval::Min2));
        assert_eq!(Interval(2).to_named_interval_try(), Some(NamedInterval::Maj2));
        assert_eq!(Interval(3).to_named_interval_try(), Some(NamedInterval::Min3));
        assert_eq!(Interval(4).to_named_interval_try(), Some(NamedInterval::Maj3));
        assert_eq!(Interval(5).to_named_interval_try(), Some(NamedInterval::Per4));
        assert_eq!(Interval(6).to_named_interval_try(), Some(NamedInterval::Trit));
        assert_eq!(Interval(7).to_named_interval_try(), Some(NamedInterval::Per5));
        assert_eq!(Interval(8).to_named_interval_try(), Some(NamedInterval::Min6));
        assert_eq!(Interval(9).to_named_interval_try(), Some(NamedInterval::Maj6));
        assert_eq!(Interval(10).to_named_interval_try(), Some(NamedInterval::Min7));
        assert_eq!(Interval(11).to_named_interval_try(), Some(NamedInterval::Maj7));
        assert_eq!(Interval(12).to_named_interval_try(), Some(NamedInterval::Per8));
        assert_eq!(Interval(13).to_named_interval_try(), Some(NamedInterval::Min9));
        assert_eq!(Interval(14).to_named_interval_try(), Some(NamedInterval::Maj9));
        assert_eq!(Interval(15).to_named_interval_try(), Some(NamedInterval::Aug9));
        assert_eq!(Interval(16).to_named_interval_try(), Some(NamedInterval::Min11));
        assert_eq!(Interval(17).to_named_interval_try(), Some(NamedInterval::Maj11));
        assert_eq!(Interval(18).to_named_interval_try(), Some(NamedInterval::Aug11));
        assert_eq!(Interval(19).to_named_interval_try(), None);
        assert_eq!(Interval(20).to_named_interval_try(), Some(NamedInterval::Min13));
        assert_eq!(Interval(21).to_named_interval_try(), Some(NamedInterval::Maj13));
        assert_eq!(Interval(22).to_named_interval_try(), Some(NamedInterval::Aug13));
        assert_eq!(Interval(23).to_named_interval_try(), None);
        assert_eq!(Interval(-1).to_named_interval_try(), None);
        for i in 0..12{
            assert_eq!(
                Interval(i).to_named_interval_try().unwrap(),
                Interval(i).to_named_interval_mod()
            );
        }
        for i in 0..24{
            let itry = Interval(i).to_named_interval_try();
            let imod = Interval(i).to_named_interval_mod();
            assert!(if itry.is_some() { itry.unwrap() == imod } else { true });
        }
        assert_eq!(Interval(-1).to_named_interval_mod(), NamedInterval::Maj7);
        assert_eq!(Interval(-2).to_named_interval_mod(), NamedInterval::Min7);
        assert_eq!(Interval(-3).to_named_interval_mod(), NamedInterval::Maj6);
        assert_eq!(Interval(-4).to_named_interval_mod(), NamedInterval::Min6);
        assert_eq!(Interval(-5).to_named_interval_mod(), NamedInterval::Per5);
        assert_eq!(Interval(-6).to_named_interval_mod(), NamedInterval::Trit);
        assert_eq!(Interval(-7).to_named_interval_mod(), NamedInterval::Per4);
        assert_eq!(Interval(-8).to_named_interval_mod(), NamedInterval::Maj3);
        assert_eq!(Interval(-9).to_named_interval_mod(), NamedInterval::Min3);
        assert_eq!(Interval(-10).to_named_interval_mod(), NamedInterval::Maj2);
        assert_eq!(Interval(-11).to_named_interval_mod(), NamedInterval::Min2);
        assert_eq!(Interval(-12).to_named_interval_mod(), NamedInterval::Root);
        assert_eq!(Interval(-13).to_named_interval_mod(), NamedInterval::Maj7);
    }

    #[test]
    fn interval_to_named_octave_interval(){
        assert_eq!(Interval(0).to_named_octave_interval_try(), Some(NamedOctaveInterval::Root));
        assert_eq!(Interval(1).to_named_octave_interval_try(), Some(NamedOctaveInterval::Min2));
        assert_eq!(Interval(2).to_named_octave_interval_try(), Some(NamedOctaveInterval::Maj2));
        assert_eq!(Interval(3).to_named_octave_interval_try(), Some(NamedOctaveInterval::Min3));
        assert_eq!(Interval(4).to_named_octave_interval_try(), Some(NamedOctaveInterval::Maj3));
        assert_eq!(Interval(5).to_named_octave_interval_try(), Some(NamedOctaveInterval::Per4));
        assert_eq!(Interval(6).to_named_octave_interval_try(), Some(NamedOctaveInterval::Trit));
        assert_eq!(Interval(7).to_named_octave_interval_try(), Some(NamedOctaveInterval::Per5));
        assert_eq!(Interval(8).to_named_octave_interval_try(), Some(NamedOctaveInterval::Min6));
        assert_eq!(Interval(9).to_named_octave_interval_try(), Some(NamedOctaveInterval::Maj6));
        assert_eq!(Interval(10).to_named_octave_interval_try(), Some(NamedOctaveInterval::Min7));
        assert_eq!(Interval(11).to_named_octave_interval_try(), Some(NamedOctaveInterval::Maj7));
        assert_eq!(Interval(12).to_named_octave_interval_try(), None);
        assert_eq!(Interval(-1).to_named_octave_interval_try(), None);
        for i in 0..12{
            assert_eq!(
                Interval(i).to_named_octave_interval_try().unwrap(),
                Interval(i).to_named_octave_interval_mod()
            );
        }
        for i in 0..24{
            let itry = Interval(i).to_named_octave_interval_try();
            let imod = Interval(i).to_named_octave_interval_mod();
            assert!(if itry.is_some() { itry.unwrap() == imod } else { true });
        }
        assert_eq!(Interval(-1).to_named_octave_interval_mod(), NamedOctaveInterval::Maj7);
        assert_eq!(Interval(-2).to_named_octave_interval_mod(), NamedOctaveInterval::Min7);
        assert_eq!(Interval(-3).to_named_octave_interval_mod(), NamedOctaveInterval::Maj6);
        assert_eq!(Interval(-4).to_named_octave_interval_mod(), NamedOctaveInterval::Min6);
        assert_eq!(Interval(-5).to_named_octave_interval_mod(), NamedOctaveInterval::Per5);
        assert_eq!(Interval(-6).to_named_octave_interval_mod(), NamedOctaveInterval::Trit);
        assert_eq!(Interval(-7).to_named_octave_interval_mod(), NamedOctaveInterval::Per4);
        assert_eq!(Interval(-8).to_named_octave_interval_mod(), NamedOctaveInterval::Maj3);
        assert_eq!(Interval(-9).to_named_octave_interval_mod(), NamedOctaveInterval::Min3);
        assert_eq!(Interval(-10).to_named_octave_interval_mod(), NamedOctaveInterval::Maj2);
        assert_eq!(Interval(-11).to_named_octave_interval_mod(), NamedOctaveInterval::Min2);
        assert_eq!(Interval(-12).to_named_octave_interval_mod(), NamedOctaveInterval::Root);
        assert_eq!(Interval(-13).to_named_octave_interval_mod(), NamedOctaveInterval::Maj7);
    }

    #[test]
    fn named_octave_interval_to_named_interval(){
        assert_eq!(NamedOctaveInterval::Root.to_named_interval_try(), Some(NamedInterval::Root));
        assert_eq!(NamedOctaveInterval::Min2.to_named_interval_try(), Some(NamedInterval::Min2));
        assert_eq!(NamedOctaveInterval::Maj2.to_named_interval_try(), Some(NamedInterval::Maj2));
        assert_eq!(NamedOctaveInterval::Min3.to_named_interval_try(), Some(NamedInterval::Min3));
        assert_eq!(NamedOctaveInterval::Maj3.to_named_interval_try(), Some(NamedInterval::Maj3));
        assert_eq!(NamedOctaveInterval::Per4.to_named_interval_try(), Some(NamedInterval::Per4));
        assert_eq!(NamedOctaveInterval::Trit.to_named_interval_try(), Some(NamedInterval::Trit));
        assert_eq!(NamedOctaveInterval::Per5.to_named_interval_try(), Some(NamedInterval::Per5));
        assert_eq!(NamedOctaveInterval::Min6.to_named_interval_try(), Some(NamedInterval::Min6));
        assert_eq!(NamedOctaveInterval::Maj6.to_named_interval_try(), Some(NamedInterval::Maj6));
        assert_eq!(NamedOctaveInterval::Min7.to_named_interval_try(), Some(NamedInterval::Min7));
        assert_eq!(NamedOctaveInterval::Maj7.to_named_interval_try(), Some(NamedInterval::Maj7));
        for noi in NamedOctaveInterval::ALL{
            let itry = noi.to_named_interval_try();
            let imod = noi.to_named_interval_mod();
            assert_eq!(itry, Some(imod));
        }
    }

    #[test]
    fn named_interval_to_named_octave_interval(){
        assert_eq!(NamedInterval::Root.to_named_octave_interval_try(), Some(NamedOctaveInterval::Root));
        assert_eq!(NamedInterval::Min2.to_named_octave_interval_try(), Some(NamedOctaveInterval::Min2));
        assert_eq!(NamedInterval::Maj2.to_named_octave_interval_try(), Some(NamedOctaveInterval::Maj2));
        assert_eq!(NamedInterval::Min3.to_named_octave_interval_try(), Some(NamedOctaveInterval::Min3));
        assert_eq!(NamedInterval::Maj3.to_named_octave_interval_try(), Some(NamedOctaveInterval::Maj3));
        assert_eq!(NamedInterval::Per4.to_named_octave_interval_try(), Some(NamedOctaveInterval::Per4));
        assert_eq!(NamedInterval::Trit.to_named_octave_interval_try(), Some(NamedOctaveInterval::Trit));
        assert_eq!(NamedInterval::Per5.to_named_octave_interval_try(), Some(NamedOctaveInterval::Per5));
        assert_eq!(NamedInterval::Min6.to_named_octave_interval_try(), Some(NamedOctaveInterval::Min6));
        assert_eq!(NamedInterval::Maj6.to_named_octave_interval_try(), Some(NamedOctaveInterval::Maj6));
        assert_eq!(NamedInterval::Min7.to_named_octave_interval_try(), Some(NamedOctaveInterval::Min7));
        assert_eq!(NamedInterval::Maj7.to_named_octave_interval_try(), Some(NamedOctaveInterval::Maj7));
        let v = [
            NamedInterval::Root, NamedInterval::Min2, NamedInterval::Maj2,
            NamedInterval::Min3, NamedInterval::Maj3, NamedInterval::Per4,
            NamedInterval::Trit, NamedInterval::Per5, NamedInterval::Min6,
            NamedInterval::Maj6, NamedInterval::Min7, NamedInterval::Maj7
        ];
        for ni in v{
            let itry = ni.to_named_octave_interval_try();
            let imod = ni.to_named_octave_interval_mod();
            assert_eq!(itry, Some(imod));
        }
    }

    #[test]
    fn intervals_as_ionian_relative_string_try(){
        assert_eq!(
            vec![Interval(0)].as_ionian_relative_string_try(false),
            None
        );
        assert_eq!(
            vec![Interval(0), Interval(0), Interval(0), Interval(0),
                    Interval(0), Interval(0), Interval(0)].as_ionian_relative_string_try(false),
            Some(String::from("♮1 ♮2 ♮3 ♮4 ♮5 ♮6 ♮7"))
        );
        assert_eq!(
            vec![Interval(-2), Interval(-1), Interval(0), Interval(1),
                    Interval(2), Interval(3), Interval(4)].as_ionian_relative_string_try(false),
            Some(String::from("♭♭1 ♭2 ♮3 ♯4 ♯♯5 ♯♯♯6 ♯♯♯♯7"))
        );
        let scalea = vec![PC::C, PC::D, PC::E, PC::F, PC::G, PC::A, PC::B]
            .to_scale_try(Note(1)).unwrap().to_steps(true);
        let scaleb = vec![PC::A, PC::B, PC::C, PC::D, PC::E, PC::F, PC::G]
            .to_scale_try(Note(1)).unwrap().to_steps(true);
        assert_eq!(
            scaleb.to_relative_intervals(&scalea).unwrap().to_ionian_relative_string_try(true),
            Some(String::from("1 2 ♭3 4 5 ♭6 ♭7"))
        );
    }
}
