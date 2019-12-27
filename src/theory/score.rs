use super::note::*;

pub type Score = Vec<Staff>;
pub type Staff = Vec<Bar>;

pub type TimedNote = (Note, f32);
pub type TimeSig = (u16,u16);

pub enum Clef{
    GClef,
    CClef(usize),
    FClef,
}

pub struct Key{
    base: Note,
    generator: Vec<Note>,
}

impl Key{
    pub fn std_generator() -> Vec<Note>{
        vec![0, 2, 4, 5, 7, 9, 10, 12]
    }

    pub fn new(base: Note, generator: Vec<Note>) -> Self{
        Self{
            base,
            generator,
        }
    }

    pub fn from_accidentals(acc: Vec<Accidental>) -> Self{
        let mut gen = Self::std_generator();
        let x = std::cmp::min(gen.len(), acc.len());
        for i in 0..x{
            gen[i] = (gen[i] as i16 + Self::accidental_mutation(acc[i])) as Note;
        }
        Self{
            base: NamedNote::G(4).to_note(),
            generator: gen,
        }
    }

    pub fn accidental_mutation(acc: Accidental) -> i16{
        match acc{
            Accidental::Sharp => SEMI as i16,
            Accidental::Flat => -(SEMI as i16),
            Accidental::Natural => 0,
            Accidental::DoubleSharp => WHOLE as i16,
            Accidental::DoubleFlat => -(WHOLE as i16),
        }
    }

    pub fn pitch_on_level(&self, level: usize) -> Note{
        let len = self.generator.len();
        let index = level % len;
        let stride = (level / len) as Note;
        self.base + self.generator[index] + (self.generator[len - 1] * stride)
    }
}

pub struct Bar{
    notes: Vec<TimedNote>,
    key: Key,
    tempo: f32,
    time_sig: TimeSig,
}

impl Bar{
    pub fn new(note_zero: Note, key: Key, tempo: f32, time_sig: TimeSig) -> Self{
        Self{
            notes: Vec::new(),
            key,
            tempo,
            time_sig,
        }
    }

    pub fn new_std(tempo: f32, time_sig: TimeSig) -> Self{
        Self{
            notes: Vec::new(),
            key: Key::new(NamedNote::G(4).to_note(), Key::std_generator()),
            tempo,
            time_sig,
        }
    }

    pub fn new_from_accidentals(clef: Clef, acc: Vec<Accidental>, tempo: f32, time_sig: TimeSig) -> Self{
        Self{
            notes: Vec::new(),
            key: Key::from_accidentals(acc),
            tempo,
            time_sig,
        }
    }
}
