use super::interval::*;
// use std::collections::{ HashMap, HashSet };

pub const A4: Note = 48;

pub type Note = i32;
pub type Rank = u16;

/// Keep collections of notes distinct.
/// It's all the same with different interpretation.
/// Once, it was all just ```Vec<Note>``` with different types such as ```type Scale = Vec<Note>```.
/// This provides us with compile time checks.
/// Interchanging the versions now only can be done explicitly.

pub type Notes = Vec<Note>;
#[derive(Clone,PartialEq,Eq,Hash,Default)]
pub struct Steps(pub Vec<Note>);
#[derive(Clone,Default)]
pub struct Scale(pub Vec<Note>);
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash,Clone,Default)]
pub struct Chord(pub Vec<Note>);
#[derive(PartialEq,Copy,Clone)]
pub enum RelativeNote { Flat(Note), Sharp(Note), Natural, Blank }
pub const RN_BLANK: RelativeNote = RelativeNote::Blank;
pub const RN_NAT: RelativeNote = RelativeNote::Natural;
pub const RN_S: RelativeNote = RelativeNote::Sharp(1);
pub const RN_SS: RelativeNote = RelativeNote::Sharp(2);
pub const RN_B: RelativeNote = RelativeNote::Flat(1);
pub const RN_BB: RelativeNote = RelativeNote::Flat(2);
#[derive(Clone)]
pub struct Relative(pub Vec<RelativeNote>);

impl Relative{
    pub fn empty(len: usize) -> Self{
        Relative(vec![RelativeNote::Natural; len])
    }
}

pub trait NoteSequence{
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

macro_rules! ImplNoteSequence{
    ($type:ty) => {
        impl NoteSequence for $type{
            fn len(&self) -> usize{
                self.0.len()
            }

            fn is_empty(&self) -> bool{
                self.0.is_empty()
            }
        }
    }
}

ImplNoteSequence!(Steps);
ImplNoteSequence!(Scale);
ImplNoteSequence!(Chord);
ImplNoteSequence!(Relative);

pub trait ToScale{
    fn to_scale(&self, note: Note) -> Scale;
}

pub trait IntoScale{
    fn into_scale(self, note: Note) -> Scale;
}

impl<T: ToScale> IntoScale for T{
    fn into_scale(self, note: Note) -> Scale{
        self.to_scale(note)
    }
}

pub trait ToSteps{
    fn to_steps(&self) -> Steps;
}

pub trait IntoSteps{
    fn into_steps(self) -> Steps;
}

impl<T: ToSteps> IntoSteps for T{
    fn into_steps(self) -> Steps{
        self.to_steps()
    }
}

pub trait ToPC{
    fn to_pc(&self) -> PC;
}

pub trait IntoPC{
    fn into_pc(self) -> PC;
}

impl<T: ToPC> IntoPC for T{
    fn into_pc(self) -> PC{
        self.to_pc()
    }
}

pub trait IntoPCs{
    fn into_pcs(self) -> PCs;
}

pub trait ToRelative{
    fn to_relative(&self, reference: &Steps) -> Option<Relative>;
}

pub trait IntoRelative{
    fn into_relative(self, reference: &Steps) -> Option<Relative>;
}

impl<T: ToRelative> IntoRelative for T{
    fn into_relative(self, reference: &Steps) -> Option<Relative>{
        self.to_relative(reference)
    }
}

pub trait ToChord{
    fn to_chord(&self) -> Chord;
}

pub trait IntoChord{
    fn into_chord(self) -> Chord;
}

impl<T: ToChord> IntoChord for T{
    fn into_chord(self) -> Chord{
        self.to_chord()
    }
}

impl ToString for RelativeNote{
    fn to_string(&self) -> String{
        let mut res = String::new();
        match self{
            RelativeNote::Natural => {  },
            RelativeNote::Blank => { res.push('?'); },
            RelativeNote::Sharp(i) => {
                for _ in 0..*i { res.push('♯'); }
            },
            RelativeNote::Flat(i) => {
                for _ in 0..*i { res.push('♭'); }
            }
        }
        res
    }
}

pub const A:  PC = PC(0);
pub const AS: PC = PC(1);
pub const B:  PC = PC(2);
pub const C:  PC = PC(3);
pub const CS: PC = PC(4);
pub const D:  PC = PC(5);
pub const DS: PC = PC(6);
pub const E:  PC = PC(7);
pub const F:  PC = PC(8);
pub const FS: PC = PC(9);
pub const G:  PC = PC(10);
pub const GS: PC = PC(11);

#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub struct PC(Note); // PitchClass

pub type PCs = Vec<PC>;

impl PC{
    pub fn to_note(self, rank: Rank) -> Note{
        self.0 + (rank as Note * OCTAVE)
    }

    pub fn to_string_name(self) -> String{
        match self.0{
            0  => "A",
            1  => "A♯",
            2  => "B",
            3  => "C",
            4  => "C♯",
            5  => "D",
            6  => "D♯",
            7  => "E",
            8  => "F",
            9  => "F♯",
            10  => "G",
            11 => "G♯",
            _ => panic!("PC::to_string_name: impossible"),
        }.to_string()
    }
}

impl std::fmt::Display for PC{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}", self.to_string_name())
    }
}

impl std::fmt::Debug for PC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_name())
    }
}

impl ToPC for Note{
    fn to_pc(&self) -> PC{
        let inrank = self % 12;
        if inrank >= 12 { panic!("as_pc: should never happen!"); }
        PC(inrank)
    }
}

impl IntoPCs for Scale{
    fn into_pcs(self) -> PCs{
        let mut res = Vec::new();
        for n in self.0{
            res.push(n.to_pc());
        }
        res
    }
}

// pub fn scale_to_ucns_enharmonic(scale: &Scale, ucns: &[UCN]) -> UCNS{
//     let mut map = HashMap::new();
//     let mut set = HashSet::new();
//     for ucn in ucns{
//         map.insert(ucn.to_named(0).chromatic_to_index(), ucn);
//         set.insert(ucn.to_base_letter());
//     }
//     let mut res = Vec::new();
//     for n in &scale.0{
//         let ucn = as_ucn(*n);
//         let ucn = if let Some(ucn_fixed) = map.get(&ucn.to_named(0).chromatic_to_index()){
//             **ucn_fixed
//         } else {
//             let base_letter = ucn.to_base_letter();
//             if set.contains(&base_letter){
//                 ucn.to_alternative().expect("Expect: scale_to_ucns_enharmonic")
//             } else {
//                 set.insert(base_letter);
//                 ucn
//             }
//         };
//         res.push(ucn);
//     }
//     res
// }

impl IntoPCs for String{
    fn into_pcs(self) -> PCs{
        let mut lowercase = String::new();
        for c in self.chars(){
            for l in c.to_lowercase(){
                lowercase.push(l);
            }
        }
        fn str_to_pc(s: &str) -> Option<PC>{
            match s{
                "ab" => Some(PC(11)),
                "a"  => Some(PC(0)),
                "a#" => Some(PC(1)),
                "bb" => Some(PC(1)),
                "b"  => Some(PC(2)),
                "b#" => Some(PC(3)),
                "cb" => Some(PC(2)),
                "c"  => Some(PC(3)),
                "c#" => Some(PC(4)),
                "db" => Some(PC(4)),
                "d"  => Some(PC(5)),
                "d#" => Some(PC(6)),
                "eb" => Some(PC(6)),
                "e"  => Some(PC(7)),
                "e#" => Some(PC(8)),
                "fb" => Some(PC(7)),
                "f"  => Some(PC(8)),
                "f#" => Some(PC(9)),
                "gb" => Some(PC(9)),
                "g"  => Some(PC(10)),
                "g#" => Some(PC(11)),
                _ => None,
            }
        }
        lowercase.split(',').into_iter().map(|s| str_to_pc(&s.chars().map(|c| match c { '♯' => '#', '♭' => 'b', x => x }).collect::<String>())).flatten().collect::<Vec<_>>()
    }
}

impl ToScale for PCs{
    fn to_scale(&self, rank: Note) -> Scale{
        let mut rank = rank as Rank;
        if self.is_empty() { return Scale::default(); }
        let start_note = self[0].to_note(rank);
        let mut res = vec![start_note];
        let mut last = start_note;
        for pc in self.iter().skip(1){
            let note = pc.to_note(rank);
            let diff = note - last;
            if diff > 0{
                last = note;
                res.push(note);
                continue;
            }
            rank += 1;
            last = pc.to_note(rank);
            res.push(last);
        }
        Scale(res)
    }
}

impl IntoSteps for PCs{
    fn into_steps(self) -> Steps{
        self.to_scale(0).into_steps()
    }
}

/*
0   1   2   3   4   5   6   7   8   9   10  11  // rank 0
12  13  14  15  16  17  18  19  20  21  22  23  // rank 1
24  25  26  27  28  29  30  31  32  33  34  35  // rank 2
36  37  38  39  40  41  42  43  44  45  46  47  // rank 3
48                                              // A4
*/

// note (48*SEMI) (48=12*4) is A4 at 440 hz
pub fn to_pitch(note: Note) -> f32{
    let x = note as i32 - 48;
    (2.0f32).powf(x as f32 / OCTAVE as f32) * 440.0
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_to_pitch(){
        assert_eq!(to_pitch(NamedNote::A(4).to_note()).round() as i32, 440);
    }
}
