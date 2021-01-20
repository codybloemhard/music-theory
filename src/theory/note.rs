use std::convert::TryInto;
use super::interval::*;
use std::collections::{ HashMap, HashSet };

pub const A4: Note = 5760;

pub type Note = i32;
pub type Rank = u16;

/// Keep collections of notes distinct.
/// It's all the same with different interpretation.
/// Once, it was all just ```Vec<Note>``` with different types such as ```type Scale = Vec<Note>```.
/// This provides us with compile time checks.
/// Interchanging the versions now only can be done explicitly.

pub type Notes = Vec<Note>;
#[derive(Clone,PartialEq,Eq,Hash)]
pub struct Steps(pub Vec<Note>);
#[derive(Clone)]
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
        Relative(vec![RelativeNote::Natural; len])
    }
}

pub trait NoteSequence{
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

impl NoteSequence for Steps{
    fn len(&self) -> usize{
        self.0.len()
    }

    fn is_empty(&self) -> bool{
        self.0.is_empty()
    }
}

impl NoteSequence for Scale{
    fn len(&self) -> usize{
        self.0.len()
    }

    fn is_empty(&self) -> bool{
        self.0.is_empty()
    }
}

impl NoteSequence for Chord{
    fn len(&self) -> usize{
        self.0.len()
    }

    fn is_empty(&self) -> bool{
        self.0.is_empty()
    }
}

impl NoteSequence for Relative{
    fn len(&self) -> usize{
        self.0.len()
    }

    fn is_empty(&self) -> bool{
        self.0.is_empty()
    }
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

pub trait IntoUCNS{
    fn into_ucns(self) -> UCNS;
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
                for _ in 0..*i{ res.push('♯'); }
            },
            RelativeNote::Flat(i) => {
                for _ in 0..*i{ res.push('♭'); }
            }
        }
        res
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

#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub enum UCN{ // unranked chromatic note for ez writing down shit
    Ab, A, As, Bb, B, Bs, Cb, C, Cs, Db, D, Ds, Eb, E, Es, Fb, F, Fs, Gb, G, Gs,
}

pub type UCNS = Vec<UCN>;

impl UCN{
    pub fn to_named(self, rank: Rank) -> NamedNote{
        match self{
            UCN::Ab => NamedNote::Ab(rank),
            UCN::A  => NamedNote::A(rank),
            UCN::As => NamedNote::As(rank),
            UCN::Bb => NamedNote::Bb(rank),
            UCN::B  => NamedNote::B(rank),
            UCN::Bs => NamedNote::Bs(rank),
            UCN::Cb => NamedNote::Cb(rank),
            UCN::C  => NamedNote::C(rank),
            UCN::Cs => NamedNote::Cs(rank),
            UCN::Db => NamedNote::Db(rank),
            UCN::D  => NamedNote::D(rank),
            UCN::Ds => NamedNote::Ds(rank),
            UCN::Eb => NamedNote::Eb(rank),
            UCN::E  => NamedNote::E(rank),
            UCN::Es => NamedNote::Es(rank),
            UCN::Fb => NamedNote::Fb(rank),
            UCN::F  => NamedNote::F(rank),
            UCN::Fs => NamedNote::Fs(rank),
            UCN::Gb => NamedNote::Gb(rank),
            UCN::G  => NamedNote::G(rank),
            UCN::Gs => NamedNote::Gs(rank),
        }
    }

    pub fn to_base_letter(&self) -> char{
        match self{
            UCN::Ab => 'a',
            UCN::A  => 'a',
            UCN::As => 'a',
            UCN::Bb => 'b',
            UCN::B  => 'b',
            UCN::Bs => 'b',
            UCN::Cb => 'c',
            UCN::C  => 'c',
            UCN::Cs => 'c',
            UCN::Db => 'd',
            UCN::D  => 'd',
            UCN::Ds => 'd',
            UCN::Eb => 'e',
            UCN::E  => 'e',
            UCN::Es => 'e',
            UCN::Fb => 'f',
            UCN::F  => 'f',
            UCN::Fs => 'f',
            UCN::Gb => 'g',
            UCN::G  => 'g',
            UCN::Gs => 'g',
        }
    }

    pub fn to_alternative(&self) -> Option<Self>{
        match self{
            UCN::Ab => Some(UCN::Gs),
            UCN::A  => None,
            UCN::As => Some(UCN::Bb),
            UCN::Bb => Some(UCN::As),
            UCN::B  => None,
            UCN::Bs => Some(UCN::Cb),
            UCN::Cb => Some(UCN::Bs),
            UCN::C  => None,
            UCN::Cs => Some(UCN::Db),
            UCN::Db => Some(UCN::Cs),
            UCN::D  => None,
            UCN::Ds => Some(UCN::Eb),
            UCN::Eb => Some(UCN::Ds),
            UCN::E  => None,
            UCN::Es => Some(UCN::Fb),
            UCN::Fb => Some(UCN::Es),
            UCN::F  => None,
            UCN::Fs => Some(UCN::Gb),
            UCN::Gb => Some(UCN::Fs),
            UCN::G  => None,
            UCN::Gs => Some(UCN::Ab),
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

impl std::fmt::Debug for UCN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_named(0).to_string_name())
    }
}

pub fn as_ucn(note: Note) -> UCN{
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

impl IntoUCNS for Scale{
    fn into_ucns(self) -> UCNS{
        let mut res = Vec::new();
        for n in self.0{
            res.push(as_ucn(n));
        }
        res
    }
}

pub fn scale_to_ucns_enharmonic(scale: &Scale, ucns: &[UCN]) -> UCNS{
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    for ucn in ucns{
        map.insert(ucn.to_named(0).chromatic_to_index(), ucn);
        set.insert(ucn.to_base_letter());
    }
    let mut res = Vec::new();
    for n in &scale.0{
        let ucn = as_ucn(*n);
        let ucn = if let Some(ucn_fixed) = map.get(&ucn.to_named(0).chromatic_to_index()){
            **ucn_fixed
        } else {
            let base_letter = ucn.to_base_letter();
            if set.contains(&base_letter){
                ucn.to_alternative().expect("Expect: scale_to_ucns_enharmonic")
            } else {
                set.insert(base_letter);
                ucn
            }
        };
        res.push(ucn);
    }
    res
}

impl IntoUCNS for String{
    fn into_ucns(self) -> UCNS{
        let mut lowercase = String::new();
        for c in self.chars(){
            for l in c.to_lowercase(){
                lowercase.push(l);
            }
        }
        fn str_to_ucn(s: &str) -> Option<UCN>{
            match s{
                "ab" => Some(UCN::Ab),
                "a" => Some(UCN::A),
                "a#" => Some(UCN::As),
                "bb" => Some(UCN::Bb),
                "b" => Some(UCN::B),
                "b#" => Some(UCN::Bs),
                "cb" => Some(UCN::Cb),
                "c" => Some(UCN::C),
                "c#" => Some(UCN::Cs),
                "db" => Some(UCN::Db),
                "d" => Some(UCN::D),
                "d#" => Some(UCN::Ds),
                "eb" => Some(UCN::Eb),
                "e" => Some(UCN::E),
                "e#" => Some(UCN::Es),
                "fb" => Some(UCN::Fb),
                "f" => Some(UCN::F),
                "f#" => Some(UCN::Fs),
                "gb" => Some(UCN::Gb),
                "g" => Some(UCN::G),
                "g#" => Some(UCN::Gs),
                _ => None,
            }
        }
        lowercase.split(',').into_iter().map(|s| str_to_ucn(&s.chars().map(|c| match c { '♯' => '#', '♭' => 'b', x => x }).collect::<String>())).flatten().collect::<Vec<_>>()
    }
}

pub fn ucns_to_named(ucns: &[UCN], starting_rank: Rank) -> Vec<NamedNote>{
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

impl ToScale for UCNS{
    fn to_scale(&self, rank: Note) -> Scale{
        let named = ucns_to_named(self, rank as Rank);
        // TODO: Make this possible
        // named.map(&|n| n.to_note())
        let mut res = Vec::new();
        for n in named{
            res.push(n.to_note());
        }
        Scale(res)
    }
}

impl IntoSteps for UCNS{
    fn into_steps(self) -> Steps{
        self.to_scale(0).into_steps()
    }
}

#[derive(Clone,Copy)]
pub enum NamedNote{
    Ab(Rank), A(Rank), As(Rank), Bb(Rank), B(Rank), Bs(Rank), Cb(Rank), C(Rank), Cs(Rank), Db(Rank), D(Rank), Ds(Rank), Eb(Rank), E(Rank), Es(Rank),
        Fb(Rank), F(Rank), Fs(Rank), Gb(Rank), G(Rank), Gs(Rank), MicroTonal(Note)
}

impl NamedNote{
    pub fn from_note(note: Note) -> Self{
        if note % SEMI == 0 { // This a a chromatic note
            let rank: Rank = (note / PERFECT_OCTAVE).max(0).try_into().unwrap();
            let mut inrank = (note / SEMI) % 12;
            if inrank < 0 { inrank += 12; }
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
            NamedNote::Ab(r)    => r,
            NamedNote::A(r)     => r,
            NamedNote::As(r)    => r,
            NamedNote::Bb(r)    => r,
            NamedNote::B(r)     => r,
            NamedNote::Bs(r)    => r,
            NamedNote::Cb(r)    => r,
            NamedNote::C(r)     => r,
            NamedNote::Cs(r)    => r,
            NamedNote::Db(r)    => r,
            NamedNote::D(r)     => r,
            NamedNote::Ds(r)    => r,
            NamedNote::Eb(r)    => r,
            NamedNote::E(r)     => r,
            NamedNote::Es(r)    => r,
            NamedNote::Fb(r)    => r,
            NamedNote::F(r)     => r,
            NamedNote::Fs(r)    => r,
            NamedNote::Gb(r)    => r,
            NamedNote::G(r)     => r,
            NamedNote::Gs(r)    => r,
            NamedNote::MicroTonal(n) => (n / 1440) as Rank,
        }
    }

    pub fn chromatic_to_index(self) -> Note{
        match self{
            NamedNote::Ab(_)    => 11,
            NamedNote::A(_)     => 0,
            NamedNote::As(_)    => 1,
            NamedNote::Bb(_)    => 1,
            NamedNote::B(_)     => 2,
            NamedNote::Bs(_)    => 3,
            NamedNote::Cb(_)    => 2,
            NamedNote::C(_)     => 3,
            NamedNote::Cs(_)    => 4,
            NamedNote::Db(_)    => 4,
            NamedNote::D(_)     => 5,
            NamedNote::Ds(_)    => 6,
            NamedNote::Eb(_)    => 6,
            NamedNote::E(_)     => 7,
            NamedNote::Es(_)    => 8,
            NamedNote::Fb(_)    => 7,
            NamedNote::F(_)     => 8,
            NamedNote::Fs(_)    => 9,
            NamedNote::Gb(_)    => 9,
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
            NamedNote::Ab(_)    => "A♭",
            NamedNote::A(_)     => "A",
            NamedNote::As(_)    => "A♯",
            NamedNote::Bb(_)    => "B♭",
            NamedNote::B(_)     => "B",
            NamedNote::Bs(_)    => "B♯",
            NamedNote::Cb(_)    => "C♭",
            NamedNote::C(_)     => "C",
            NamedNote::Cs(_)    => "C♯",
            NamedNote::Db(_)    => "D♭",
            NamedNote::D(_)     => "D",
            NamedNote::Ds(_)    => "D♯",
            NamedNote::Eb(_)    => "E♭",
            NamedNote::E(_)     => "E",
            NamedNote::Es(_)    => "E♯",
            NamedNote::Fb(_)    => "F♭",
            NamedNote::F(_)     => "F",
            NamedNote::Fs(_)    => "F♯",
            NamedNote::Gb(_)    => "G♭",
            NamedNote::G(_)     => "G",
            NamedNote::Gs(_)    => "G♯",
            NamedNote::MicroTonal(_) => "X",
        }.to_string()
    }

    pub fn is_chromatic(self) -> bool{
        !matches!(self, Self::MicroTonal(_))
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

pub fn print_notes(scale: &[Note], seperator: &str){
    if scale.is_empty() { return; }
    let lenm1 = scale.len() - 1;
    for note in scale.iter().take(lenm1){
        print!("{}{}", NamedNote::from_note(*note).as_string(), seperator);
    }
    println!("{}", NamedNote::from_note(scale[lenm1]).as_string());
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_to_pitch(){
        assert_eq!(to_pitch(NamedNote::A(4).to_note()).round() as i32, 440);
    }
}
