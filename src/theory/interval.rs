use super::traits::{ GeneratablePartialOrder, OctaveShiftable, AddInterval };
use super::note::{ _Note, Octave, OctaveShift };

use std::cmp::Ordering;

pub(crate) const _SEMI: _Note = 1;
pub(crate) const _WHOLE: _Note = 2;

pub(crate) const _ROOT: _Note = 0;
pub(crate) const _MIN2: _Note = 1;
pub(crate) const _MAJ2: _Note = 2;
pub(crate) const _MIN3: _Note = 3;
pub(crate) const _MAJ3: _Note = 4;
pub(crate) const _PER4: _Note = 5;
pub(crate) const _TRIT: _Note = 6;
pub(crate) const _PER5: _Note = 7;
pub(crate) const _MIN6: _Note = 8;
pub(crate) const _MAJ6: _Note = 9;
pub(crate) const _MIN7: _Note = 10;
pub(crate) const _MAJ7: _Note = 11;
pub(crate) const _OCTAVE: _Note = 12;
pub(crate) const _MIN9: _Note = 13;
pub(crate) const _MAJ9: _Note = 14;
pub(crate) const _AUG9: _Note = 15;
pub(crate) const _MIN11: _Note = 16;
pub(crate) const _MAJ11: _Note = 17;
pub(crate) const _AUG11: _Note = 18;
pub(crate) const _PER12: _Note = 19;
pub(crate) const _MIN13: _Note = 20;
pub(crate) const _MAJ13: _Note = 21;
pub(crate) const _AUG13: _Note = 22;

pub(crate) const _DIM2: _Note = 0;
pub(crate) const _AUG1: _Note = 1;
pub(crate) const _DIM3: _Note = 2;
pub(crate) const _AUG2: _Note = 3;
pub(crate) const _DIM4: _Note = 4;
pub(crate) const _AUG3: _Note = 5;
pub(crate) const _DIM5: _Note = 6;
pub(crate) const _AUG4: _Note = 6;
pub(crate) const _DIM6: _Note = 7;
pub(crate) const _AUG5: _Note = 8;
pub(crate) const _DIM7: _Note = 9;
pub(crate) const _AUG6: _Note = 10;
pub(crate) const _DIM8: _Note = 11;
pub(crate) const _AUG7: _Note = 12;

// pub const SEMI: NamedInterval = NamedInterval::Min2;
// pub const WHOLE: NamedInterval = NamedInterval::Maj2;
//
// pub const ROOT: NamedInterval = NamedInterval::Root;
// pub const MIN2: NamedInterval = NamedInterval::Min2;
// pub const MAJ2: NamedInterval = NamedInterval::Maj2;
// pub const MIN3: NamedInterval = NamedInterval::Min3;
// pub const MAJ3: NamedInterval = NamedInterval::Maj3;
// pub const PER4: NamedInterval = NamedInterval::Per4;
// pub const TRIT: NamedInterval = NamedInterval::Trit;
// pub const PER5: NamedInterval = NamedInterval::Per5;
// pub const MIN6: NamedInterval = NamedInterval::Min6;
// pub const MAJ6: NamedInterval = NamedInterval::Maj6;
// pub const MIN7: NamedInterval = NamedInterval::Min7;
// pub const MAJ7: NamedInterval = NamedInterval::Maj7;
// pub const OCTAVE: NamedInterval = NamedInterval::Per8;
// pub const MIN9: NamedInterval = NamedInterval::Min9;
// pub const MAJ9: NamedInterval = NamedInterval::Maj9;
// pub const AUG9: NamedInterval = NamedInterval::Aug9;
// pub const MIN11: NamedInterval = NamedInterval::Min11;
// pub const MAJ11: NamedInterval = NamedInterval::Maj11;
// pub const AUG11: NamedInterval = NamedInterval::Aug11;
// pub const MIN13: NamedInterval = NamedInterval::Min13;
// pub const MAJ13: NamedInterval = NamedInterval::Maj13;
// pub const AUG13: NamedInterval = NamedInterval::Aug13;
//

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interval(pub(crate) i32);

impl Interval{
    pub const MAX: Self = Self(1 << 30);
    pub const MIN: Self = Self(-1 << 30);

    pub const SEMI: Self = Self(1);
    pub const WHOLE: Self = Self(2);

    pub const ROOT: Self = Self(0);
    pub const MIN2: Self = Self(1);
    pub const MAJ2: Self = Self(2);
    pub const MIN3: Self = Self(3);
    pub const MAJ3: Self = Self(4);
    pub const PER4: Self = Self(5);
    pub const TRIT: Self = Self(6);
    pub const PER5: Self = Self(7);
    pub const MIN6: Self = Self(8);
    pub const MAJ6: Self = Self(9);
    pub const MIN7: Self = Self(10);
    pub const MAJ7: Self = Self(11);
    pub const OCTAVE: Self = Self(12);
    pub const MIN9: Self = Self(13);
    pub const MAJ9: Self = Self(14);
    pub const AUG9: Self = Self(15);
    pub const MIN11: Self = Self(16);
    pub const MAJ11: Self = Self(17);
    pub const AUG11: Self = Self(18);
    pub const PER12: Self = Self(19);
    pub const MIN13: Self = Self(20);
    pub const MAJ13: Self = Self(21);
    pub const AUG13: Self = Self(22);

    pub const DIM2: Self = Self(0);
    pub const AUG1: Self = Self(1);
    pub const DIM3: Self = Self(2);
    pub const AUG2: Self = Self(3);
    pub const DIM4: Self = Self(4);
    pub const AUG3: Self = Self(5);
    pub const DIM5: Self = Self(6);
    pub const AUG4: Self = Self(6);
    pub const DIM6: Self = Self(7);
    pub const AUG5: Self = Self(8);
    pub const DIM7: Self = Self(9);
    pub const AUG6: Self = Self(10);
    pub const DIM8: Self = Self(11);
    pub const AUG7: Self = Self(12);

    fn new(i: i32) -> Self{
        Interval(i.min(Self::MAX.0).max(Self::MIN.0))
    }
}

impl std::ops::Neg for Interval{
    type Output = Self;

    fn neg(self) -> Self{
        Self::new(-self.0)
    }
}

impl std::ops::Add for Interval{
    type Output = Self;

    fn add(self, other: Self) -> Self{
        Self::new(self.0 + other.0)
    }
}

impl std::ops::Sub for Interval{
    type Output = Self;

    fn sub(self, other: Self) -> Self{
        Self::new(self.0 - other.0)
    }
}

impl std::fmt::Display for Interval{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let mut string = String::new();
        let i = self.0.abs();
        match self.0.cmp(&0){
            Ordering::Less    => { for _ in 0..i { string.push('♭'); } },
            Ordering::Greater => { for _ in 0..i { string.push('♯'); } },
            Ordering::Equal   => { string.push('♮'); },
        }
        write!(f, "{}", string)
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

impl OctaveShiftable for Interval{
    fn with_octave(self, octave: Octave) -> Self{
        match self.0.cmp(&0){
            Ordering::Less => Interval((self.0 % _OCTAVE as i32) - octave as i32 * _OCTAVE as i32),
            Ordering::Equal => Interval(0),
            Ordering::Greater => Interval((self.0 % _OCTAVE as i32) + octave as i32 * _OCTAVE as i32),
        }
    }

    fn shift_octave(self, shift: OctaveShift) -> Self{
        Interval::new(self.0 + shift as i32 * _OCTAVE as i32)
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

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum NamedInterval{
//     Root = 0,
//     Min2 = 1,
//     Maj2 = 2,
//     Min3 = 3,
//     Maj3 = 4,
//     Per4 = 5,
//     Trit = 6,
//     Per5 = 7,
//     Min6 = 8,
//     Maj6 = 9,
//     Min7 = 10,
//     Maj7 = 11,
//     Per8 = 12,
//     Min9 = 13,
//     Maj9 = 14,
//     Aug9 = 15,
//     Min11 = 16,
//     Maj11 = 17,
//     Aug11 = 18,
//     Min13 = 20,
//     Maj13 = 21,
//     Aug13 = 22,
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum NamedOctaveInterval{
//     Root = 0,
//     Min2 = 1,
//     Maj2 = 2,
//     Min3 = 3,
//     Maj3 = 4,
//     Per4 = 5,
//     Trit = 6,
//     Per5 = 7,
//     Min6 = 8,
//     Maj6 = 9,
//     Min7 = 10,
//     Maj7 = 11,
// }
//
// impl ToNote for NamedInterval{
//     fn to_note(&self) -> Note{
//         *self as Note
//     }
// }
//
// pub trait ToNamedInterval{
//     fn to_interval_try(self) -> Option<NamedInterval>;
//     fn to_interval_mod(self) -> NamedInterval;
// }
//
// impl ToNamedInterval for Note{
//     fn to_interval_try(self) -> Option<NamedInterval>{
//         match self{
//             0 => Some(NamedInterval::Root),
//             1 => Some(NamedInterval::Min2),
//             2 => Some(NamedInterval::Maj2),
//             3 => Some(NamedInterval::Min3),
//             4 => Some(NamedInterval::Maj3),
//             5 => Some(NamedInterval::Per4),
//             6 => Some(NamedInterval::Trit),
//             7 => Some(NamedInterval::Per5),
//             8 => Some(NamedInterval::Min6),
//             9 => Some(NamedInterval::Maj6),
//             10 => Some(NamedInterval::Min7),
//             11 => Some(NamedInterval::Maj7),
//             12 => Some(NamedInterval::Per8),
//             13 => Some(NamedInterval::Min9),
//             14 => Some(NamedInterval::Maj9),
//             15 => Some(NamedInterval::Aug9),
//             16 => Some(NamedInterval::Min11),
//             17 => Some(NamedInterval::Maj11),
//             18 => Some(NamedInterval::Aug11),
//             20 => Some(NamedInterval::Min13),
//             21 => Some(NamedInterval::Maj13),
//             22 => Some(NamedInterval::Aug13),
//             _ => None,
//         }
//     }
//
//     fn to_interval_mod(self) -> NamedInterval{
//         let int = (self % 24).to_interval_try();
//         match int{
//             Some(i) => i,
//             None => (self % 12).to_interval_try().unwrap()
//         }
//     }
// }
//
// impl ToNamedInterval for NamedOctaveInterval{
//     fn to_interval_try(self) -> Option<NamedInterval>{
//         (self as Note).to_interval_try()
//     }
//
//     fn to_interval_mod(self) -> NamedInterval{
//         (self as Note).to_interval_mod()
//     }
// }
//
// impl ToNamedOctaveInterval for Note{
//     fn to_octave_interval_try(self) -> Option<NamedOctaveInterval>{
//         match self{
//             0 => Some(NamedOctaveInterval::Root),
//             1 => Some(NamedOctaveInterval::Min2),
//             2 => Some(NamedOctaveInterval::Maj2),
//             3 => Some(NamedOctaveInterval::Min3),
//             4 => Some(NamedOctaveInterval::Maj3),
//             5 => Some(NamedOctaveInterval::Per4),
//             6 => Some(NamedOctaveInterval::Trit),
//             7 => Some(NamedOctaveInterval::Per5),
//             8 => Some(NamedOctaveInterval::Min6),
//             9 => Some(NamedOctaveInterval::Maj6),
//             10 => Some(NamedOctaveInterval::Min7),
//             11 => Some(NamedOctaveInterval::Maj7),
//             _ => None,
//         }
//     }
//
//     fn to_octave_interval_mod(self) -> NamedOctaveInterval{
//         (self % 12).to_octave_interval_try().unwrap()
//     }
// }
//
// impl ToNamedOctaveInterval for NamedInterval{
//     fn to_octave_interval_try(self) -> Option<NamedOctaveInterval>{
//         (self as Note).to_octave_interval_try()
//     }
//
//     fn to_octave_interval_mod(self) -> NamedOctaveInterval{
//         (self as Note).to_octave_interval_mod()
//     }
// }
//
// pub trait ToNamedOctaveInterval{
//     fn to_octave_interval_try(self) -> Option<NamedOctaveInterval>;
//     fn to_octave_interval_mod(self) -> NamedOctaveInterval;
// }
//
// pub trait ToIntervalChordExtension{
//     fn to_interval_chord_extension(self) -> String;
// }
//
// impl ToIntervalChordExtension for NamedInterval{
//     fn to_interval_chord_extension(self) -> String{
//         let names = [
//             "R", "♭2", "♮2", "♭3", "♮3", "♮4", "♭5", "♮5", "♭6", "♮6", "♭7", "♮7", "♮8",
//             "♭9", "♮9", "♯9", "♭11", "♮11", "♯11", "", "♭13", "♮13", "♯13",
//         ];
//         names[self as usize].to_string()
//     }
// }
//
// impl ToIntervalChordExtension for Option<NamedInterval>{
//     fn to_interval_chord_extension(self) -> String{
//         match self{
//             None => "x".to_string(),
//             Some(int) => int.to_interval_chord_extension(),
//         }
//     }
// }
//
// pub fn to_degree(interval: Note) -> String{
//     match interval{
//         0 => "I",
//         1 => "bII",
//         2 => "II",
//         3 => "bIII",
//         4 => "III",
//         5 => "IV",
//         6 => "bV",
//         7 => "V",
//         8 => "bVI",
//         9 => "VI",
//         10 => "bVII",
//         11 => "VII",
//         _ => "[outofrange]",
//     }.to_string()
// }

#[cfg(test)]
mod tests{
    use super::*;
    use crate::theory::*;

    #[test]
    fn new(){
        assert_eq!(Interval::new(0), Interval(0));
        assert_eq!(Interval::new(1 << 30), Interval::MAX);
        assert_eq!(Interval::new(-1 << 30), Interval::MIN);
        assert_eq!(Interval::new((1 << 30) + 1), Interval::MAX);
        assert_eq!(Interval::new((-1 << 30) - 1), Interval::MIN);
    }

    #[test]
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
    fn sub(){
        assert_eq!(Interval(0) - Interval(0), Interval(0));
        assert_eq!(Interval(1) - Interval(2), Interval(-1));
        assert_eq!(Interval(1) - Interval(-2), Interval(3));
    }

    #[test]
    fn neg_add_sub(){
        assert_eq!(-(Interval(13) + -Interval(24) - Interval(3)), Interval(14));
    }

    #[test]
    fn to_string(){
        assert_eq!(&Interval(0).to_string(), "♮");
        assert_eq!(&Interval(1).to_string(), "♯");
        assert_eq!(&Interval(10).to_string(), "♯♯♯♯♯♯♯♯♯♯");
        assert_eq!(&Interval(-1).to_string(), "♭");
        assert_eq!(&Interval(-10).to_string(), "♭♭♭♭♭♭♭♭♭♭");
    }

    #[test]
    fn generatable_partial_order(){
        assert_eq!(Interval(0).next(), Some(Interval(1)));
        assert_eq!(Interval(0).prev(), Some(Interval(-1)));
        assert_eq!(Interval::MAX.next(), None);
        assert_eq!(Interval::MIN.prev(), None);
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
        assert_eq!(Interval(0).with_octave(u16::MAX) < Interval::MAX, true);
        assert_eq!(Interval(0).with_octave(u16::MAX) < Interval::MAX, true);
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
}
