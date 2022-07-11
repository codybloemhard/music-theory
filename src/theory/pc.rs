use super::traits::{
    Cyclic, ToNote, ToPC, ToLetterTry, ToEnharmonicNote, AsScaleTry, OctaveShiftable,
    AsSteps, AsStepsTry
};
use super::{ Note, _Note, Letter, Interval, EnharmonicNote, Scale, Octave, Steps };

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

pub type PCs = Vec<PC>;

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

impl AsScaleTry for PCs{
    fn as_scale_try(&self, octave: Note) -> Option<Scale>{
        if octave.0 > Octave::MAX as _Note { return None; }
        let mut octave = octave.0 as Octave;
        if self.is_empty() { return Some(Scale::default()); }
        let start_note = self[0].to_note().with_octave(octave);
        let mut res = vec![start_note];
        let mut last = start_note;
        for pc in self.iter().skip(1){
            let note = pc.to_note().with_octave(octave);
            let diff = note - last;
            if diff > Interval::ROOT{
                last = note;
                res.push(note);
            } else {
                if octave == Octave::MAX { return None; }
                octave += 1;
                last = pc.to_note().with_octave(octave);
                res.push(last);
            }
        }
        Some(Scale(res))
    }
}

impl AsStepsTry for PCs{
    fn as_steps_try(&self, complete_octave_cycle: bool) -> Option<Steps>{
        self.as_scale_try(Note::MIN).map(|x| x.as_steps(complete_octave_cycle))
    }
}

#[cfg(test)]
mod tests{
    use super::super::*;

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

    #[test]
    fn pcs_as_scale_try(){
        assert_eq!(
            vec![PC::A, PC::C, PC::E, PC::G, PC::B].to_scale_try(Note(1)),
            Some(Scale(vec![Note::A1, Note::C1, Note::E1, Note::G1, Note::B2]))
        );
        assert_eq!(vec![PC::A, PC::B].to_scale_try(Note(u16::MAX as u32)).is_some(), true);
        assert_eq!(vec![PC::A, PC::B].to_scale_try(Note(u16::MAX as u32 + 1)), None);
        assert_eq!(vec![PC::G, PC::A].to_scale_try(Note(u16::MAX as u32)).is_some(), false);
    }

    #[test]
    fn pcs_as_steps_try(){
        assert_eq!(
            vec![PC::C, PC::D, PC::E, PC::F, PC::G, PC::A, PC::B].to_steps_try(true),
            Some(Steps(vec![Interval(2), Interval(2), Interval(1), Interval(2),
                        Interval(2), Interval(2), Interval(1)]))
        );
    }
}
