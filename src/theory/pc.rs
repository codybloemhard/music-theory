use super::traits::{ Cyclic, ToNote, ToPC, ToLetterTry, ToEnharmonicNote };
use super::{ Note, _Note, Letter, Interval, EnharmonicNote };

pub const A:  PC = PC::A;
pub const AS: PC = PC::As;
pub const B:  PC = PC::B;
pub const C:  PC = PC::C;
pub const CS: PC = PC::Cs;
pub const D:  PC = PC::D;
pub const DS: PC = PC::Ds;
pub const E:  PC = PC::E;
pub const F:  PC = PC::F;
pub const FS: PC = PC::Fs;
pub const G:  PC = PC::G;
pub const GS: PC = PC::Gs;

// PitchClass
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum PC{
    A = 0, As = 1, B = 2, C = 3, Cs = 4, D = 5,
    Ds = 6, E = 7, F = 8, Fs = 9, G = 10, Gs = 11
}

impl PC{
    pub const ALL: [PC; 12] = [
        PC::A, PC::As, PC::B, PC::C, PC::Cs, PC::D,
        PC::Ds, PC::E, PC::F, PC::Fs, PC::G, PC::Gs
    ];
}

// pub type PCs = Vec<PC>;

impl std::fmt::Display for PC{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let string = match self{
            PC::A  => "A",
            PC::As => "A♯",
            PC::B  => "B",
            PC::C  => "C",
            PC::Cs => "C♯",
            PC::D  => "D",
            PC::Ds => "D♯",
            PC::E  => "E",
            PC::F  => "F",
            PC::Fs => "F♯",
            PC::G  => "G",
            PC::Gs => "G♯",
        };
        write!(f, "{}", string)
    }
}

impl Cyclic for PC{
    fn next(self) -> Self{
        (self as _Note + 1).to_pc()
    }

    fn prev(self) -> Self{
        (self as _Note + 11).to_pc()
    }
}

// Conversion Traits

impl ToNote for PC{
    fn to_note(self) -> Note{
        Note::new(self as _Note)
    }
}

impl ToLetterTry for PC{
    fn to_letter_try(&self) -> Option<Letter>{
        Some(match self{
            Self::A  => Letter::A,
            Self::As => Letter::A,
            Self::B  => Letter::B,
            Self::C  => Letter::C,
            Self::Cs => Letter::C,
            Self::D  => Letter::D,
            Self::Ds => Letter::D,
            Self::E  => Letter::E,
            Self::F  => Letter::F,
            Self::Fs => Letter::F,
            Self::G  => Letter::G,
            Self::Gs => Letter::G,
        })
    }
}

impl ToEnharmonicNote for PC{
    fn to_enharmonic_note(self) -> EnharmonicNote{
        match self{
            PC::A  => EnharmonicNote{ letter: Letter::A, accidental: Interval(0) },
            PC::As => EnharmonicNote{ letter: Letter::A, accidental: Interval(1) },
            PC::B  => EnharmonicNote{ letter: Letter::B, accidental: Interval(0) },
            PC::C  => EnharmonicNote{ letter: Letter::C, accidental: Interval(0) },
            PC::Cs => EnharmonicNote{ letter: Letter::C, accidental: Interval(1) },
            PC::D  => EnharmonicNote{ letter: Letter::D, accidental: Interval(0) },
            PC::Ds => EnharmonicNote{ letter: Letter::D, accidental: Interval(1) },
            PC::E  => EnharmonicNote{ letter: Letter::E, accidental: Interval(0) },
            PC::F  => EnharmonicNote{ letter: Letter::F, accidental: Interval(0) },
            PC::Fs => EnharmonicNote{ letter: Letter::F, accidental: Interval(1) },
            PC::G  => EnharmonicNote{ letter: Letter::G, accidental: Interval(0) },
            PC::Gs => EnharmonicNote{ letter: Letter::G, accidental: Interval(1) },
        }
    }
}

// pub trait IntoPCs{
//     fn into_pcs(self) -> PCs;
// }
//
// impl IntoPCs for Scale{
//     fn into_pcs(self) -> PCs{
//         let mut res = Vec::new();
//         for n in self.0{
//             res.push(n.to_pc());
//         }
//         res
//     }
// }
//
// impl ToScale for PCs{
//     fn to_scale(&self, rank: Note) -> Scale{
//         let mut rank = rank as Rank;
//         if self.is_empty() { return Scale::default(); }
//         let start_note = self[0].to_note().with_rank(rank);
//         let mut res = vec![start_note];
//         let mut last = start_note;
//         for pc in self.iter().skip(1){
//             let note = pc.to_note().with_rank(rank);
//             let diff = note - last;
//             if diff > 0{
//                 last = note;
//                 res.push(note);
//                 continue;
//             }
//             rank += 1;
//             last = pc.to_note().with_rank(rank);
//             res.push(last);
//         }
//         Scale(res)
//     }
// }
//
// impl IntoSteps for PCs{
//     fn into_steps(self) -> Steps{
//         self.to_scale(0).into_steps()
//     }
// }
//

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn to_note(){
        for (i, pc) in PC::ALL.iter().enumerate(){
            assert_eq!(pc.to_note().inside() as usize, i);
        }
    }

    #[test]
    fn cyclic(){
        assert_eq!(PC::A.next(),  PC::As);
        assert_eq!(PC::As.next(), PC::B);
        assert_eq!(PC::B.next(),  PC::C);
        assert_eq!(PC::C.next(),  PC::Cs);
        assert_eq!(PC::Cs.next(), PC::D);
        assert_eq!(PC::D.next(),  PC::Ds);
        assert_eq!(PC::Ds.next(), PC::E);
        assert_eq!(PC::E.next(),  PC::F);
        assert_eq!(PC::F.next(),  PC::Fs);
        assert_eq!(PC::Fs.next(), PC::G);
        assert_eq!(PC::G.next(),  PC::Gs);
        assert_eq!(PC::Gs.next(), PC::A);

        assert_eq!(PC::A.prev(),  PC::Gs);
        assert_eq!(PC::As.prev(), PC::A);
        assert_eq!(PC::B.prev(),  PC::As);
        assert_eq!(PC::C.prev(),  PC::B);
        assert_eq!(PC::Cs.prev(), PC::C);
        assert_eq!(PC::D.prev(),  PC::Cs);
        assert_eq!(PC::Ds.prev(), PC::D);
        assert_eq!(PC::E.prev(),  PC::Ds);
        assert_eq!(PC::F.prev(),  PC::E);
        assert_eq!(PC::Fs.prev(), PC::F);
        assert_eq!(PC::G.prev(),  PC::Fs);
        assert_eq!(PC::Gs.prev(), PC::G);
    }

    #[test]
    fn to_string(){
        assert_eq!(&PC::A.to_string(),  "A");
        assert_eq!(&PC::As.to_string(), "A♯");
        assert_eq!(&PC::B.to_string(),  "B");
        assert_eq!(&PC::C.to_string(),  "C");
        assert_eq!(&PC::Cs.to_string(), "C♯");
        assert_eq!(&PC::D.to_string(),  "D");
        assert_eq!(&PC::Ds.to_string(), "D♯");
        assert_eq!(&PC::E.to_string(),  "E");
        assert_eq!(&PC::F.to_string(),  "F");
        assert_eq!(&PC::Fs.to_string(), "F♯");
        assert_eq!(&PC::G.to_string(),  "G");
        assert_eq!(&PC::Gs.to_string(), "G♯");
    }

    #[test]
    fn to_letter_try(){
        for pc in PC::ALL{
            assert_eq!(pc.to_letter_try().is_some(), true);
        }
    }

    #[test]
    fn to_enharmonic_note(){
        for pc in PC::ALL{
            assert_eq!(pc.to_note(), pc.to_enharmonic_note().to_note());
            assert_eq!(pc, pc.to_enharmonic_note().to_pc());
        }
    }
}
