use super::note::{ _Note };
// use std::cmp::Ordering;

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
// pub fn to_relative_interval_non_nat(interval: Note) -> String{
//     let mut res = String::new();
//     let i = interval;
//     match i.cmp(&0){
//         Ordering::Less => { for _ in 0..-i{ res.push('♭'); } },
//         Ordering::Greater => { for _ in 0..i{ res.push('♯'); } },
//         Ordering::Equal => { res.push('♮'); },
//     }
//     res
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

