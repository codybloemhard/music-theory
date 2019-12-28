use super::note::*;

pub struct Score{
    pub bars: Vec<Vec<Bar>>,
}

impl Score{
    pub fn new() -> Self{
        Self{
            bars: Vec::new(),
        }
    }

    pub fn add_note(&mut self, note: BarNote, staff: usize){
        if staff >= self.bars.len(){ return; } // need some logging
        if self.bars[staff].is_empty() { return; }
        self.append_if_needed(staff);
        let residue = self.append_note(staff, note);
        if let Some(rnote) = residue{
            self.append_if_needed(staff);
            let rr = self.append_note(staff, rnote); // TODO: make while loop
        }
    }

    pub fn staff_head_unsafe(&self, staff: usize) -> usize{
        self.bars[staff].len() - 1
    }

    pub fn need_new(&self, staff: usize) -> bool{
        self.bars[staff][self.staff_head_unsafe(staff)].has_left()
    }

    pub fn append_from_prev(&mut self, staff: usize){
        let new = self.bars[staff][self.staff_head_unsafe(staff)].clone_into_new();
        self.bars[staff].push(new)
    }

    pub fn append_if_needed(&mut self, staff: usize){
        if self.need_new(staff){
            self.append_from_prev(staff);
        }
    }

    pub fn append_note(&mut self, staff: usize, note: BarNote) -> Option<BarNote>{
        let index = self.staff_head_unsafe(staff);
        self.bars[staff][index].add_note(note)
    }
}

#[derive(Clone,Copy)]
pub struct TimeSig{
    beats: u16,
    size: f32,
}

impl TimeSig{
    pub fn new(beats: u16, size: f32) -> Self{
        Self{
            beats,
            size,
        }
    }

    pub fn from_notation(beats: u16, size: u16) -> Self{
        Self{
            beats,
            size: 1.0 / (size as f32),
        }
    }

    pub fn beats(self) -> u16{
        self.beats
    }

    pub fn size(self) -> f32{
        self.size
    }

    pub fn total(self) -> f32{
        self.beats as f32 * self.size
    }
}

pub enum Clef{
    GClef,
    CClef(usize),
    FClef,
}

#[derive(Clone)]
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
            gen[i] = apply_accidental_global(gen[i], acc[i]);
        }
        Self{
            base: NamedNote::G(4).to_note(),
            generator: gen,
        }
    }

    pub fn pitch_on_level(&self, level: usize) -> Note{
        let len = self.generator.len();
        let index = level % len;
        let stride = (level / len) as Note;
        self.base + self.generator[index] + (self.generator[len - 1] * stride)
    }
}
#[derive(Copy,Clone)]
pub enum NoteEffect{
    PinchHarmonic,
}

pub type BarNote = (Note,f32,Vec<NoteEffect>);

pub struct Bar{
    pub notes: Vec<BarNote>,
    pub key: Key,
    pub tempo: f32,
    pub time_sig: TimeSig,
    time_left: f32,
}

impl Bar{
    pub fn new(note_zero: Note, key: Key, tempo: f32, time_sig: TimeSig) -> Self{
        Self{
            notes: Vec::new(),
            key,
            tempo,
            time_sig,
            time_left: time_sig.total(),
        }
    }

    pub fn new_std(tempo: f32, time_sig: TimeSig) -> Self{
        Self{
            notes: Vec::new(),
            key: Key::new(NamedNote::G(4).to_note(), Key::std_generator()),
            tempo,
            time_sig,
            time_left: time_sig.total(),
        }
    }

    pub fn new_from_accidentals(clef: Clef, acc: Vec<Accidental>, tempo: f32, time_sig: TimeSig) -> Self{
        Self{
            notes: Vec::new(),
            key: Key::from_accidentals(acc),
            tempo,
            time_sig,
            time_left: time_sig.total(),
        }
    }

    pub fn add_note(&mut self, (note,duration,effects): BarNote) -> Option<BarNote>{
        let diff = self.time_left - duration;
        if diff > 0.0{ // fits inside this bar still
            self.notes.push((note,duration,effects));
            self.time_left = diff;
            Option::None
        }else{
            self.notes.push((note,self.time_left,effects.clone()));
            self.time_left = 0.0;
            Option::Some((CARRY_ON,-diff,effects))
        }
    }

    pub fn has_left(&self) -> bool{
        self.time_left > 0.0001
    }

    pub fn clone_into_new(&self) -> Self{
        Self{
            notes: Vec::new(),
            key: self.key.clone(),
            tempo: self.tempo,
            time_sig: self.time_sig,
            time_left: self.time_sig.total(),
        }
    }
}
