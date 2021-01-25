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

pub trait ToStringName{
    fn to_string_name(&self) -> String;
}

pub trait ToNote{
    fn to_note(&self, rank: Rank) -> Note;
}

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

pub trait ToEnharmonicNote{
    fn to_enharmonic_note(&self) -> Option<EnharmonicNote>;
}

pub trait IntoEnharmonicNote{
    fn into_enharmonic_note(self) -> Option<EnharmonicNote>;
}

impl<T: ToEnharmonicNote> IntoEnharmonicNote for T{
    fn into_enharmonic_note(self) -> Option<EnharmonicNote>{
        self.to_enharmonic_note()
    }
}

pub trait IntoEnharmonicNotes{
    fn into_enharmonic_notes(self) -> Vec<EnharmonicNote>;
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

impl ToStringName for PC{
    fn to_string_name(&self) -> String{
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
            10 => "G",
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

impl ToNote for PC{
    fn to_note(&self, rank: Rank) -> Note{
        self.0 + (rank as Note * OCTAVE)
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
#[derive(Clone,Copy,Default,Debug)]
pub struct EnharmonicNote{
    letter: u8,
    accidental: i8,
}

impl EnharmonicNote{
    pub fn letter(&self) -> u8{
        self.letter
    }

    pub fn accidental(&self) -> i8{
        self.accidental
    }

    pub fn next_enharmonic(&self) -> Self{
        match self.letter{
            0 => Self{ letter: 1, accidental: self.accidental - 2 }, // A = Bbb
            1 => Self{ letter: 2, accidental: self.accidental - 1 }, // B = Cb
            2 => Self{ letter: 3, accidental: self.accidental - 2 }, // C = Dbb
            3 => Self{ letter: 4, accidental: self.accidental - 2 }, // D = Ebb
            4 => Self{ letter: 5, accidental: self.accidental - 1 }, // E = Fb
            5 => Self{ letter: 6, accidental: self.accidental - 2 }, // F = Gbb
            6 => Self{ letter: 0, accidental: self.accidental - 2 }, // G = Abb
            _ => panic!("EnharmonicNote::next_enharmonic: should be impossible"),
        }
    }
}

impl ToStringName for EnharmonicNote{
    fn to_string_name(&self) -> String{
        let mut res = match self.letter{
            0 => "A",
            1 => "B",
            2 => "C",
            3 => "D",
            4 => "E",
            5 => "F",
            6 => "G",
            _ => panic!("ToStringName for Enharmonic: should be impossible")
        }.to_string();
        res.push_str(&(if self.accidental < 0 { RelativeNote::Flat((-self.accidental).into()) } else { RelativeNote::Sharp((self.accidental).into()) }.to_string()));
        res
    }
}

impl ToNote for EnharmonicNote{
    fn to_note(&self, rank: Rank) -> Note{
        ((match self.letter{
            0 => 0,  // A
            1 => 2,  // B
            2 => 3,  // C
            3 => 5,  // D
            4 => 7,  // E
            5 => 8,  // F
            6 => 10, // G
            _ => panic!("ToNote for Enharmonic: should be impossible")
        }) + self.accidental) as Note + rank as Note
    }
}

impl ToPC for EnharmonicNote{
    fn to_pc(&self) -> PC{
        self.to_note(0).to_pc()
    }
}

impl ToEnharmonicNote for String{
    fn to_enharmonic_note(&self) -> Option<EnharmonicNote>{
        let mut lowercase = String::new();
        for c in self.chars(){
            for l in c.to_lowercase(){
                lowercase.push(l);
            }
        }
        let mut chars = lowercase.chars();
        let letter = match chars.next(){
            Some('a') => 0,
            Some('b') => 1,
            Some('c') => 2,
            Some('d') => 3,
            Some('e') => 4,
            Some('f') => 5,
            Some('g') => 6,
            _ => return None,
        };
        let mut accidental = 0;
        for ch in chars{
            match ch{
                'b' => { accidental -= 1; },
                '♭' => { accidental -= 1; },
                '#' => { accidental += 1; },
                '♯' => { accidental += 1; },
                '♮' => { accidental = 0; }
                _ => return None,
            }
        }
        Some(EnharmonicNote{ letter, accidental })
    }
}

impl IntoEnharmonicNotes for String{
    fn into_enharmonic_notes(self) -> Vec<EnharmonicNote>{
        self.split(',').into_iter().map(|s| s.to_string().to_enharmonic_note()).flatten().collect::<Vec<_>>()
    }
}

impl ToEnharmonicNote for Note{
    fn to_enharmonic_note(&self) -> Option<EnharmonicNote>{
        Some(match self % OCTAVE{
            0  => EnharmonicNote{ letter: 0, accidental: 0 },
            1  => EnharmonicNote{ letter: 0, accidental: 1 },
            2  => EnharmonicNote{ letter: 1, accidental: 0 },
            3  => EnharmonicNote{ letter: 2, accidental: 0 },
            4  => EnharmonicNote{ letter: 2, accidental: 1 },
            5  => EnharmonicNote{ letter: 3, accidental: 0 },
            6  => EnharmonicNote{ letter: 3, accidental: 1 },
            7  => EnharmonicNote{ letter: 4, accidental: 0 },
            8  => EnharmonicNote{ letter: 5, accidental: 0 },
            9  => EnharmonicNote{ letter: 5, accidental: 1 },
            10 => EnharmonicNote{ letter: 6, accidental: 0 },
            11 => EnharmonicNote{ letter: 6, accidental: 1 },
            _ => panic!("ToEnharmonicNote for Note, should be impossible."),
        })
    }
}

impl IntoEnharmonicNotes for Scale{
    fn into_enharmonic_notes(self) -> Vec<EnharmonicNote>{
        let mut set = vec![0,0,0,0,0,0,0];
        let mut res = Vec::new();
        for (i, note) in self.0.into_iter().enumerate(){
            if i >= 7 { return Vec::new(); } // Impossible: no more letters.
            let en = note.to_enharmonic_note().unwrap();
            let en = if set[en.letter() as usize] == 1{
                let mut nen = en;
                loop {
                    nen = nen.next_enharmonic();
                    if set[nen.letter() as usize] == 0 { break nen; }
                }
            } else {
                en
            };
            set[en.letter() as usize] = 1;
            res.push(en);
        }
        res
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
