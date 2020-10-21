use std::convert::TryInto;
use super::interval::*;

pub const A4: Note = 5760;

pub type Note = i32;
pub type Rank = u16;

/// Keep collections of notes distinct.
/// It's all the same with different interpretation.
/// Once, it was all just ```Vec<Note>``` with different types such as ```type Scale = Vec<Note>```.
/// This provides us with compile time checks.
/// Interchanging the versions now only can be done explicitly.

pub type Notes = Vec<Note>;
#[derive(Clone)]
pub struct Steps(pub Vec<Note>);
#[derive(Clone)]
pub struct Scale(pub Vec<Note>);
#[derive(Clone)]
pub struct Chord(pub Vec<Note>);
#[derive(Clone)]
pub struct Relative(pub Vec<Note>);

impl Steps{
    pub fn empty() -> Self{
        Steps(Vec::new())
    }
}

impl Scale{
    pub fn empty() -> Self{
        Scale(Vec::new())
    }
}

impl Chord{
    pub fn empty() -> Self{
        Chord(Vec::new())
    }
}

impl Relative{
    pub fn empty(len: usize) -> Self{
        Relative(vec![0; len])
    }
}

pub trait NoteSequence{
    fn len(&self) -> usize;
}

impl NoteSequence for Steps{
    fn len(&self) -> usize{
        self.0.len()
    }
}

impl NoteSequence for Scale{
    fn len(&self) -> usize{
        self.0.len()
    }
}

impl NoteSequence for Chord{
    fn len(&self) -> usize{
        self.0.len()
    }
}

impl NoteSequence for Relative{
    fn len(&self) -> usize{
        self.0.len()
    }
}

pub trait IntoSteps{
    fn to_steps(self) -> Steps;
}

impl IntoSteps for Scale{
    fn to_steps(self) -> Steps{
        if self.0.is_empty() { return Steps::empty(); }
        let mut last = self.0[0];
        let mut intervals = Vec::new();
        for note in self.0.iter().skip(1){
            let diff = note - last;
            intervals.push(diff);
            last = *note;
        }
        Steps(intervals)
    }
}

pub const A: UCN = UCN::A;
pub const AS: UCN = UCN::As;
pub const B: UCN = UCN::B;
pub const C: UCN = UCN::C;
pub const CS: UCN = UCN::Cs;
pub const D: UCN = UCN::D;
pub const DS: UCN = UCN::Ds;
pub const E: UCN = UCN::E;
pub const F: UCN = UCN::F;
pub const FS: UCN = UCN::Fs;
pub const G: UCN = UCN::G;
pub const GS: UCN = UCN::Gs;

#[derive(Clone,Copy)]
pub enum UCN{ // unranked chromatic note for ez writing down shit
    A, As, B, C, Cs, D, Ds, E, F, Fs, G, Gs,
}

pub type UCNS = Vec<UCN>;

impl UCN{
    pub fn to_named(self, rank: Rank) -> NamedNote{
        match self{
            UCN::A  => NamedNote::A(rank),
            UCN::As => NamedNote::As(rank),
            UCN::B  => NamedNote::B(rank),
            UCN::C  => NamedNote::C(rank),
            UCN::Cs => NamedNote::Cs(rank),
            UCN::D  => NamedNote::D(rank),
            UCN::Ds => NamedNote::Ds(rank),
            UCN::E  => NamedNote::E(rank),
            UCN::F  => NamedNote::F(rank),
            UCN::Fs => NamedNote::Fs(rank),
            UCN::G  => NamedNote::G(rank),
            UCN::Gs => NamedNote::Gs(rank),
        }
    }

    pub fn to_note(self, rank: Rank) -> Note{
        self.to_named(rank).to_note()
    }
}

impl std::fmt::Display for UCN{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}", self.to_named(0).to_string_name())
    }
}

impl PartialEq for UCN{
    fn eq(&self, other: &Self) -> bool{
        self.to_note(0) == other.to_note(0)
    }
}

pub fn to_ucn(note: Note) -> UCN{
    if note % SEMI == 0 {
        let inrank = (note / SEMI) % 12;
        match inrank{
            0 => UCN::A,
            1 => UCN::As,
            2 => UCN::B,
            3 => UCN::C,
            4 => UCN::Cs,
            5 => UCN::D,
            6 => UCN::Ds,
            7 => UCN::E,
            8 => UCN::F,
            9 => UCN::Fs,
            10 => UCN::G,
            11 => UCN::Gs,
            _ => { panic!("to_ucn: should never happen!"); }
        }
    } else { // This is a microtonal note
        panic!("to_ucn: microtonal input");
    }
}

pub trait IntoUCNS{
    fn to_ucns(self) -> UCNS;
}

impl IntoUCNS for Scale{
    fn to_ucns(self) -> UCNS{
        let mut res = Vec::new();
        for n in self.0{
            res.push(to_ucn(n));
        }
        res
    }
}

pub fn ucns_to_named(ucns: &UCNS, starting_rank: Rank) -> Vec<NamedNote>{
    if ucns.is_empty() { return Vec::new(); }
    let mut rank = starting_rank;
    let start_note = ucns[0].to_named(rank);
    let mut res = vec![start_note];
    let mut last = start_note.to_note();
    for ucn in ucns.iter().skip(1){
        let note = ucn.to_named(rank);
        let note_val = note.to_note();
        let diff = note_val - last;
        if diff > 0{
            last = note_val;
            res.push(note);
            continue;
        }
        rank += 1;
        let new_note = ucn.to_named(rank);
        last = new_note.to_note();
        res.push(new_note);
    }
    res
}

pub fn ucns_to_notes(ucns: &UCNS, starting_rank: Rank) -> Scale{
    let named = ucns_to_named(ucns, starting_rank);
    // TODO: Make this possible
    // named.map(&|n| n.to_note())
    let mut res = Vec::new();
    for n in named{
        res.push(n.to_note());
    }
    Scale(res)
}

pub fn ucns_to_steps(ucns: &UCNS) -> Steps{
    let notes = ucns_to_notes(ucns, 0);
    notes.to_steps()
}

#[derive(Clone,Copy)]
pub enum NamedNote{
    A(Rank), As(Rank), B(Rank), C(Rank), Cs(Rank), D(Rank), Ds(Rank), E(Rank), F(Rank), Fs(Rank), G(Rank), Gs(Rank), MicroTonal(Note)
}

impl NamedNote{
    pub fn from_note(note: Note) -> Self{
        if note % SEMI == 0 { // This a a chromatic note
            let rank: Rank = (note / PERFECT_OCTAVE).max(0).try_into().unwrap();
            let inrank = (note / SEMI) % 12;
            match inrank{
                0 => NamedNote::A(rank),
                1 => NamedNote::As(rank),
                2 => NamedNote::B(rank),
                3 => NamedNote::C(rank),
                4 => NamedNote::Cs(rank),
                5 => NamedNote::D(rank),
                6 => NamedNote::Ds(rank),
                7 => NamedNote::E(rank),
                8 => NamedNote::F(rank),
                9 => NamedNote::Fs(rank),
                10 => NamedNote::G(rank),
                11 => NamedNote::Gs(rank),
                _ => { panic!("NamedNote::from_note: should never happen!"); }
            }
        } else { // This is a microtonal note
            NamedNote::MicroTonal(note)
        }
    }

    pub fn rank(self) -> Rank{
        match self{
            NamedNote::A(r)     => r,
            NamedNote::As(r)    => r,
            NamedNote::B(r)     => r,
            NamedNote::C(r)     => r,
            NamedNote::Cs(r)    => r,
            NamedNote::D(r)     => r,
            NamedNote::Ds(r)    => r,
            NamedNote::E(r)     => r,
            NamedNote::F(r)     => r,
            NamedNote::Fs(r)    => r,
            NamedNote::G(r)     => r,
            NamedNote::Gs(r)    => r,
            NamedNote::MicroTonal(n) => (n / 1440) as Rank,
        }
    }

    pub fn chromatic_to_index(self) -> Note{
        match self{
            NamedNote::A(_)     => 0,
            NamedNote::As(_)    => 1,
            NamedNote::B(_)     => 2,
            NamedNote::C(_)     => 3,
            NamedNote::Cs(_)    => 4,
            NamedNote::D(_)     => 5,
            NamedNote::Ds(_)    => 6,
            NamedNote::E(_)     => 7,
            NamedNote::F(_)     => 8,
            NamedNote::Fs(_)    => 9,
            NamedNote::G(_)     => 10,
            NamedNote::Gs(_)    => 11,
            _ => 0,
        }
    }

    pub fn to_note(self) -> Note{
        let x = match self{
            NamedNote::MicroTonal(n) => n,
            _ => -1,
        };
        if x > -1 {
            return x;
        }
        (self.rank() as Note * 12 * SEMI) as Note + (self.chromatic_to_index() * SEMI)
    }

    pub fn to_string_name(self) -> String{
        match self{
            NamedNote::A(_)     => "A",
            NamedNote::As(_)    => "A♯/B♭",
            NamedNote::B(_)     => "B",
            NamedNote::C(_)     => "C",
            NamedNote::Cs(_)    => "C♯/D♭",
            NamedNote::D(_)     => "D",
            NamedNote::Ds(_)    => "D♯/E♭",
            NamedNote::E(_)     => "E",
            NamedNote::F(_)     => "F",
            NamedNote::Fs(_)    => "F♯/G♭",
            NamedNote::G(_)     => "G",
            NamedNote::Gs(_)    => "G♯/A♭",
            NamedNote::MicroTonal(_) => "X",
        }.to_string()
    }

    pub fn to_string_name_sharp(self) -> String{
        match self{
            NamedNote::A(_)     => "A",
            NamedNote::As(_)    => "A♯",
            NamedNote::B(_)     => "B",
            NamedNote::C(_)     => "C",
            NamedNote::Cs(_)    => "C♯",
            NamedNote::D(_)     => "D",
            NamedNote::Ds(_)    => "D♯",
            NamedNote::E(_)     => "E",
            NamedNote::F(_)     => "F",
            NamedNote::Fs(_)    => "F♯",
            NamedNote::G(_)     => "G",
            NamedNote::Gs(_)    => "G♯",
            NamedNote::MicroTonal(_) => "X",
        }.to_string()
    }

    pub fn is_chromatic(self) -> bool{
        match self{
            Self::MicroTonal(_) => false,
            _ => true,
        }
    }

    pub fn as_string(self) -> String{
        if self.is_chromatic() {
            format!("{}{}", self.to_string_name(), self.rank())
        }else{
            let n = self.to_note();
            let close_chromatic = (n / SEMI) * SEMI;
            let diff = n - close_chromatic;
            let chroma = Self::from_note(close_chromatic);
            format!("{}+{}",chroma.to_string(),diff as f32 / SEMI as f32)
        }
    }
}

impl std::fmt::Display for NamedNote{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f, "{}", self.as_string())
    }
}

/*
0   1   2   3   4   5   6   7   8   9   10  11  // rank 0
12  13  14  15  16  17  18  19  20  21  22  23  // rank 1
24  25  26  27  28  29  30  31  32  33  34  35  // rank 2
36  37  38  39  40  41  42  43  44  45  46  47  // rank 3
48                                              // A4
*/
#[derive(Clone,Copy)]
pub enum Accidental{
    Sharp, Flat, Natural, DoubleSharp, DoubleFlat,
}

pub fn apply_accidental_global(note: Note, acc: Accidental) -> Note{
    match acc{
        Accidental::Sharp => note + SEMI,
        Accidental::Flat => note - SEMI,
        Accidental::DoubleSharp => note + WHOLE,
        Accidental::DoubleFlat => note - WHOLE,
        Accidental::Natural => note // can't apply natural without context
    }
}

// note (48*SEMI) (48=12*4) is A4 at 440 hz
pub fn to_pitch(note: Note) -> f32{
    let x = note as i32 - (48*SEMI);
    (2.0f32).powf(x as f32 / PERFECT_OCTAVE as f32) * 440.0
}

pub fn print_notes(scale: &Notes, seperator: &str){
    if scale.is_empty() { return; }
    let lenm1 = scale.len() - 1;
    for i in 0..lenm1{
        print!("{}{}", NamedNote::from_note(scale[i]).as_string(), seperator);
    }
    println!("{}", NamedNote::from_note(scale[lenm1]).as_string());
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_to_pitch(){
        assert_eq!(to_pitch(NamedNote::A(4).to_note()), 440.0);
    }
}
