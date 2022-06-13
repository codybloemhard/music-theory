use super::traits::{ Cyclic, ToPC, ToNote, ToLetterTry };
use super::{ Note, PC };
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Letter{
    A = 0, B = 1, C = 2, D = 3, E = 4, F = 5, G = 6
}

impl Letter{
    pub const ALL: [Letter; 7] = [
        Letter::A, Letter::B, Letter::C, Letter::D,
        Letter::E, Letter::F, Letter::G
    ];
}

impl std::fmt::Display for Letter{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let string = match self{
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
        };
        write!(f, "{}", string)
    }
}

impl Cyclic for Letter{
    fn next(self) -> Self{
        match self{
            Self::A => Self::B,
            Self::B => Self::C,
            Self::C => Self::D,
            Self::D => Self::E,
            Self::E => Self::F,
            Self::F => Self::G,
            Self::G => Self::A,
        }
    }

    fn prev(self) -> Self{
        match self{
            Self::A => Self::G,
            Self::B => Self::A,
            Self::C => Self::B,
            Self::D => Self::C,
            Self::E => Self::D,
            Self::F => Self::E,
            Self::G => Self::F,
        }
    }
}

impl ToPC for Letter{
    fn to_pc(self) -> PC{
        match self{
            Self::A => PC::A,
            Self::B => PC::B,
            Self::C => PC::C,
            Self::D => PC::D,
            Self::E => PC::E,
            Self::F => PC::F,
            Self::G => PC::G,
        }
    }
}

impl ToNote for Letter{
    fn to_note(self) -> Note{
        self.to_pc().to_note()
    }
}

impl ToLetterTry for String{
    fn to_letter_try(&self) -> Option<Letter>{
        match self.chars().next().map(|c| c.to_lowercase().next()){
            Some(Some('a')) => Some(Letter::A),
            Some(Some('b')) => Some(Letter::B),
            Some(Some('c')) => Some(Letter::C),
            Some(Some('d')) => Some(Letter::D),
            Some(Some('e')) => Some(Letter::E),
            Some(Some('f')) => Some(Letter::F),
            Some(Some('g')) => Some(Letter::G),
            _ => None
        }
    }
}

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

    #[test]
    fn letter_to_string(){
        for (l, s) in Letter::ALL.iter().zip(["A", "B", "C", "D", "E", "F", "G"].iter()){
            assert_eq!(&l.to_string(), s);
        }
    }

    #[test]
    fn letter_cyclic(){
        assert_eq!(Letter::A.next(), Letter::B);
        assert_eq!(Letter::B.next(), Letter::C);
        assert_eq!(Letter::C.next(), Letter::D);
        assert_eq!(Letter::D.next(), Letter::E);
        assert_eq!(Letter::E.next(), Letter::F);
        assert_eq!(Letter::F.next(), Letter::G);
        assert_eq!(Letter::G.next(), Letter::A);
        assert_eq!(Letter::A.prev(), Letter::G);
        assert_eq!(Letter::B.prev(), Letter::A);
        assert_eq!(Letter::C.prev(), Letter::B);
        assert_eq!(Letter::D.prev(), Letter::C);
        assert_eq!(Letter::E.prev(), Letter::D);
        assert_eq!(Letter::F.prev(), Letter::E);
        assert_eq!(Letter::G.prev(), Letter::F);
    }

    #[test]
    fn letter_to_pc(){
        for l in Letter::ALL{
            assert_eq!(l.to_pc().to_string(), l.to_string());
        }
    }

    #[test]
    fn letter_to_note(){
        for (l, n) in Letter::ALL.iter().zip([0, 2, 3, 5, 7, 8, 10].iter()){
            assert_eq!(l.to_note().0, *n);
        }
    }

    #[test]
    fn string_to_letter_try(){
        assert_eq!("A".to_string().to_letter_try(), Some(Letter::A));
        assert_eq!("B".to_string().to_letter_try(), Some(Letter::B));
        assert_eq!("C".to_string().to_letter_try(), Some(Letter::C));
        assert_eq!("D".to_string().to_letter_try(), Some(Letter::D));
        assert_eq!("E".to_string().to_letter_try(), Some(Letter::E));
        assert_eq!("F".to_string().to_letter_try(), Some(Letter::F));
        assert_eq!("G".to_string().to_letter_try(), Some(Letter::G));
        assert_eq!("H".to_string().to_letter_try(), None);
        assert_eq!("a".to_string().to_letter_try(), Some(Letter::A));
        assert_eq!("a8904tdiae902(@#)*@#".to_string().to_letter_try(), Some(Letter::A));
        assert_eq!("abcdefg".to_string().to_letter_try(), Some(Letter::A));
        assert_eq!("".to_string().to_letter_try(), None);
    }
}
