use super::traits::{ Cyclic, ToPC, ToNote, ToLetterTry, ToEnharmonicNote, ToEnharmonicNoteTry };
use super::{ Note, PC, Interval, AddInterval };

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnharmonicNote{
    pub letter: Letter,
    pub accidental: Interval,
}

impl Letter{
    pub const ALL: [Letter; 7] = [
        Letter::A, Letter::B, Letter::C, Letter::D,
        Letter::E, Letter::F, Letter::G
    ];
}

impl EnharmonicNote{
    pub fn spelled_as(&self, letter: Letter) -> Self{
        if self.letter == letter { return *self; }
        let up = {
            let mut en = *self;
            loop {
                if en.letter == letter { break en; }
                en = en.next();
            }
        };
        let down = {
            let mut en = *self;
            loop {
                if en.letter == letter { break en; }
                en = en.prev();
            }
        };
        if up.accidental.abs() > down.accidental.abs() {
            down
        } else {
            up
        }
    }
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

impl std::fmt::Display for EnharmonicNote{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}{}", self.letter, self.accidental)
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

impl Cyclic for EnharmonicNote{
    fn next(self) -> Self{
        let i1 = Interval(1);
        let i2 = Interval(2);
        match self.letter{
            Letter::A => Self{ letter: Letter::B, accidental: self.accidental - i2 }, // A = Bbb
            Letter::B => Self{ letter: Letter::C, accidental: self.accidental - i1 }, // B = Cb
            Letter::C => Self{ letter: Letter::D, accidental: self.accidental - i2 }, // C = Dbb
            Letter::D => Self{ letter: Letter::E, accidental: self.accidental - i2 }, // D = Ebb
            Letter::E => Self{ letter: Letter::F, accidental: self.accidental - i1 }, // E = Fb
            Letter::F => Self{ letter: Letter::G, accidental: self.accidental - i2 }, // F = Gbb
            Letter::G => Self{ letter: Letter::A, accidental: self.accidental - i2 }, // G = Abb
        }
    }

    fn prev(self) -> Self{
        let i1 = Interval(1);
        let i2 = Interval(2);
        match self.letter{
            Letter::A => Self{ letter: Letter::G, accidental: self.accidental + i2 }, // A = G##
            Letter::B => Self{ letter: Letter::A, accidental: self.accidental + i2 }, // B = A##
            Letter::C => Self{ letter: Letter::B, accidental: self.accidental + i1 }, // C = B#
            Letter::D => Self{ letter: Letter::C, accidental: self.accidental + i2 }, // D = C##
            Letter::E => Self{ letter: Letter::D, accidental: self.accidental + i2 }, // E = D##
            Letter::F => Self{ letter: Letter::E, accidental: self.accidental + i1 }, // F = E#
            Letter::G => Self{ letter: Letter::F, accidental: self.accidental + i2 }, // G = F##
        }
    }
}

// Conversion Traits

impl ToNote for Letter{
    fn to_note(self) -> Note{
        self.to_pc().to_note()
    }
}

impl ToNote for EnharmonicNote{
    fn to_note(self) -> Note{
        self.to_pc().to_note()
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

impl ToPC for EnharmonicNote{
    fn to_pc(self) -> PC{
        (super::_interval_mod(self.letter.to_note().0 as i32 + (self.accidental.0 % 12)) as u32).to_pc()
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

impl ToEnharmonicNote for Letter{
    fn to_enharmonic_note(self) -> EnharmonicNote{
        EnharmonicNote{ letter: self, accidental: Interval::ROOT }
    }
}

impl ToEnharmonicNoteTry for String{
    fn to_enharmonic_note_try(&self) -> Option<EnharmonicNote>{
        let mut lowercase = String::new();
        for c in self.chars(){
            for l in c.to_lowercase(){
                lowercase.push(l);
            }
        }
        let mut iter = lowercase.chars();
        let letter_part = iter.next()?;
        let accidental_part = iter;
        let letter = letter_part.to_string().to_letter_try()?;
        let mut accidental = 0;
        for ch in accidental_part{
            match ch{
                'b' => { accidental -= 1; },
                '♭' => { accidental -= 1; },
                '#' => { accidental += 1; },
                '♯' => { accidental += 1; },
                '♮' => { accidental = 0; }
                _ => return None,
            }
        }
        let accidental = Interval::new_try(accidental)?;
        Some(EnharmonicNote{ letter, accidental })
    }
}

// impl IntoEnharmonicNotes for String{
//     fn into_enharmonic_notes(self) -> Vec<EnharmonicNote>{
//         self.split(',').into_iter().filter_map(|s| s.to_string().to_enharmonic_note()).collect::<Vec<_>>()
//     }
// }
//
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

    #[test]
    fn enharmonic_note_to_string(){
        assert_eq!(&EnharmonicNote{ letter: Letter::A, accidental: Interval(-2) }.to_string(), "A♭♭");
        for l in Letter::ALL{
            for i in -25..25{
                let res = &EnharmonicNote{ letter: l, accidental: Interval(i) }.to_string();
                assert_eq!(res.chars().next(), l.to_string().chars().next());
                assert_eq!(res.chars().count(), i.abs().max(1) as usize + 1);
            }
        }
    }

    #[test]
    fn enharmonic_note_cyclic(){
        // must for tarpaulin 100% cover
        assert_eq!(EnharmonicNote{letter: Letter::A,accidental: Interval(0) }.next().letter, Letter::B);
        assert_eq!(EnharmonicNote{letter: Letter::B,accidental: Interval(0) }.next().letter, Letter::C);
        assert_eq!(EnharmonicNote{letter: Letter::C,accidental: Interval(0) }.next().letter, Letter::D);
        assert_eq!(EnharmonicNote{letter: Letter::D,accidental: Interval(0) }.next().letter, Letter::E);
        assert_eq!(EnharmonicNote{letter: Letter::E,accidental: Interval(0) }.next().letter, Letter::F);
        assert_eq!(EnharmonicNote{letter: Letter::F,accidental: Interval(0) }.next().letter, Letter::G);
        assert_eq!(EnharmonicNote{letter: Letter::G,accidental: Interval(0) }.next().letter, Letter::A);
        assert_eq!(EnharmonicNote{letter: Letter::A,accidental: Interval(0) }.prev().letter, Letter::G);
        assert_eq!(EnharmonicNote{letter: Letter::B,accidental: Interval(0) }.prev().letter, Letter::A);
        assert_eq!(EnharmonicNote{letter: Letter::C,accidental: Interval(0) }.prev().letter, Letter::B);
        assert_eq!(EnharmonicNote{letter: Letter::D,accidental: Interval(0) }.prev().letter, Letter::C);
        assert_eq!(EnharmonicNote{letter: Letter::E,accidental: Interval(0) }.prev().letter, Letter::D);
        assert_eq!(EnharmonicNote{letter: Letter::F,accidental: Interval(0) }.prev().letter, Letter::E);
        assert_eq!(EnharmonicNote{letter: Letter::G,accidental: Interval(0) }.prev().letter, Letter::F);

        let mut en = EnharmonicNote{
            letter: Letter::A,
            accidental: Interval(0),
        };
        for _ in 0..100{
            let new = en.next();
            assert_eq!(new.letter, en.letter.next());
            assert_eq!(new.to_note(), en.to_note());
        }
        en = EnharmonicNote{
            letter: Letter::C,
            accidental: Interval(0),
        };
        for _ in 0..100{
            let new = en.prev();
            assert_eq!(new.letter, en.letter.prev());
            assert_eq!(new.to_note(), en.to_note());
        }
    }

    #[test]
    fn enharmonic_note_to_pc(){
        for l in Letter::ALL{
            assert_eq!(EnharmonicNote{ letter: l, accidental: Interval(0) }.to_pc(), l.to_pc());
            assert_eq!(EnharmonicNote{ letter: l, accidental: Interval(12) }.to_pc(), l.to_pc());
        }
        assert_eq!(EnharmonicNote{ letter: Letter::A, accidental: Interval(1) }.to_pc(), PC::As);
        assert_eq!(EnharmonicNote{ letter: Letter::A, accidental: Interval(2) }.to_pc(), PC::B);
        assert_eq!(EnharmonicNote{ letter: Letter::B, accidental: Interval(1) }.to_pc(), PC::C);
        assert_eq!(EnharmonicNote{ letter: Letter::D, accidental: Interval(14) }.to_pc(), PC::E);
    }

    #[test]
    fn spelled_as(){
        for l0 in Letter::ALL{
            for l1 in Letter::ALL{
                for i in -13..13{
                    let original = EnharmonicNote{ letter: l0, accidental: Interval(i) };
                    let respelled = original.spelled_as(l1);
                    assert_eq!(respelled.letter, l1);
                    assert_eq!(original.to_note(), respelled.to_note());
                }
            }
        }
    }

    #[test]
    fn letter_to_enharmonic_note(){
        for l in Letter::ALL{
            assert_eq!(l, l.to_enharmonic_note().letter);
        }
    }

    #[test]
    fn string_to_enharmonic_note_try(){
        assert_eq!(
            "A".to_string().to_enharmonic_note_try(),
            Some(EnharmonicNote{ letter: Letter::A, accidental: Interval(0) })
        );
        assert_eq!(
            "B".to_string().to_enharmonic_note_try(),
            Some(EnharmonicNote{ letter: Letter::B, accidental: Interval(0) })
        );
        assert_eq!(
            "C".to_string().to_enharmonic_note_try(),
            Some(EnharmonicNote{ letter: Letter::C, accidental: Interval(0) })
        );
        assert_eq!(
            "D".to_string().to_enharmonic_note_try(),
            Some(EnharmonicNote{ letter: Letter::D, accidental: Interval(0) })
        );
        assert_eq!(
            "E".to_string().to_enharmonic_note_try(),
            Some(EnharmonicNote{ letter: Letter::E, accidental: Interval(0) })
        );
        assert_eq!(
            "F".to_string().to_enharmonic_note_try(),
            Some(EnharmonicNote{ letter: Letter::F, accidental: Interval(0) })
        );
        assert_eq!(
            "G".to_string().to_enharmonic_note_try(),
            Some(EnharmonicNote{ letter: Letter::G, accidental: Interval(0) })
        );
        assert_eq!(
            "Abbbb".to_string().to_enharmonic_note_try(),
            Some(EnharmonicNote{ letter: Letter::A, accidental: Interval(-4) })
        );
        assert_eq!(
            "A#######".to_string().to_enharmonic_note_try(),
            Some(EnharmonicNote{ letter: Letter::A, accidental: Interval(7) })
        );
        assert_eq!(
            "A♭♭♭♭♭♭".to_string().to_enharmonic_note_try(),
            Some(EnharmonicNote{ letter: Letter::A, accidental: Interval(-6) })
        );
        assert_eq!(
            "A♯♯♯".to_string().to_enharmonic_note_try(),
            Some(EnharmonicNote{ letter: Letter::A, accidental: Interval(3) })
        );
        assert_eq!(
            "A♮♮♮♮".to_string().to_enharmonic_note_try(),
            Some(EnharmonicNote{ letter: Letter::A, accidental: Interval(0) })
        );
        assert_eq!(
            "A♮♮♭♭♯♯♭♭♭♯♭".to_string().to_enharmonic_note_try(),
            Some(EnharmonicNote{ letter: Letter::A, accidental: Interval(-3) })
        );
        assert_eq!(
            "A♮♭♭♯♯♭♭♭♮♯".to_string().to_enharmonic_note_try(),
            Some(EnharmonicNote{ letter: Letter::A, accidental: Interval(1) })
        );
        assert_eq!(
            "A♮♭♭♯♯♭♭♭♮nottherightcharacters♯".to_string().to_enharmonic_note_try(),
            None
        );
    }
}
