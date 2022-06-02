use super::traits::{ ToNote, ToPC };
use super::note::{ Note };

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

pub const PCS: [PC; 12] = [
    PC::A, PC::As, PC::B, PC::C, PC::Cs, PC::D,
    PC::Ds, PC::E, PC::F, PC::Fs, PC::G, PC::Gs
];

// PitchClass
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum PC{
    A = 0, As = 1, B = 2, C = 3, Cs = 4, D = 5,
    Ds = 6, E = 7, F = 8, Fs = 9, G = 10, Gs = 11
}

// pub type PCs = Vec<PC>;

impl ToPC for Note{
    fn to_pc(self) -> PC{
        let index = self.0 as usize % 12;
        PCS[index]
    }
}

impl<T: ToNote> ToPC for T{
    fn to_pc(self) -> PC{
        self.to_note().to_pc()
    }
}

//
// pub trait IntoPCs{
//     fn into_pcs(self) -> PCs;
// }
//
//
// impl ToStringName for PC{
//     fn to_string_name(&self) -> String{
//         match self.0{
//             0  => "A",
//             1  => "A♯",
//             2  => "B",
//             3  => "C",
//             4  => "C♯",
//             5  => "D",
//             6  => "D♯",
//             7  => "E",
//             8  => "F",
//             9  => "F♯",
//             10 => "G",
//             11 => "G♯",
//             _ => panic!("PC::to_string_name: impossible"),
//         }.to_string()
//     }
// }
//
// impl std::fmt::Display for PC{
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
//         write!(f, "{}", self.to_string_name())
//     }
// }
//
// impl ToNote for PC{
//     fn to_note(&self) -> Note{
//         self.0
//     }
// }
//
// impl ToPC for Note{
//     fn to_pc(&self) -> PC{
//         let mut inrank = self % 12;
//         if inrank < 0 {
//             inrank += 12;
//         }
//         if inrank >= 12 { panic!("as_pc: should never happen!"); }
//         PC(inrank)
//     }
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
    use crate::theory::note::A4;

    #[test]
    fn note_to_pc(){
        assert_eq!(A4.to_pc(), PC::A);
        assert_eq!(Note::new(12).to_pc(), PC::A);
    }

    #[test]
    fn to_note_to_pc(){
        assert_eq!(13.to_pc(), PC::As);
    }
}
