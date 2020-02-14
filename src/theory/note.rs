use std::convert::TryInto;
use super::interval::*;

pub const A4: Note = 5760;

pub type Note = i32;
pub type Rank = u16;

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

pub fn print_notes(vec: &Vec<Note>, seperator: &str){
    if vec.is_empty() { return; }
    let lenm1 = vec.len() - 1;
    for i in 0..lenm1{
        print!("{}{}", NamedNote::from_note(vec[i]).as_string(), seperator);
    }
    println!("{}", NamedNote::from_note(vec[lenm1]).as_string());
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_to_pitch(){
        assert_eq!(to_pitch(NamedNote::A(4).to_note()), 440.0);
    }
}
