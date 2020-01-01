use super::note::*;
use crate::tones::tones::*;
use crate::tones::track::*;
use super::interval::CARRY_ON;

pub struct Score{
    pub bars: Vec<Vec<Bar>>,
}

impl Score{
    pub fn new() -> Self{
        Self{
            bars: Vec::new(),
        }
    }

    pub fn new_staff(&mut self){
        self.bars.push(Vec::new());
    }

    pub fn new_bar(&mut self, staff: usize, bar: Bar){
        if staff >= self.bars.len(){
            return;
        }
        self.bars[staff].push(bar);
    }

    pub fn add_note(&mut self, note: BarNote, parralel: bool, staff: usize){
        if staff >= self.bars.len(){ return; } // need some logging
        if self.bars[staff].is_empty() { return; }
        if !parralel{
            self.append_if_needed(staff);
        }
        let mut residue = self.append_note(staff, note, parralel);
        while let Some(rnote) = residue{
            self.append_if_needed(staff);
            residue = self.append_note(staff, rnote, parralel);
        }
    }

    pub fn staff_head_unsafe(&self, staff: usize) -> usize{
        self.bars[staff].len() - 1
    }

    pub fn need_new(&self, staff: usize) -> bool{
        !self.bars[staff][self.staff_head_unsafe(staff)].has_left()
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

    pub fn append_note(&mut self, staff: usize, note: BarNote, parralel: bool) -> Option<BarNote>{
        let index = self.staff_head_unsafe(staff);
        self.bars[staff][index].add_note(note, !parralel)
    }

    pub fn render_to_track_stereo<F0,F1,F2,F3>(&self, staff: usize, track: &mut Track, runout: f32, volume: f32, pan: f32, samplef: &F0, volf: &F1, hzf: &F2, passf: &F3)
        where
            F0: Fn(f32,f32) -> (f32,f32),
            F1: Fn(f32,f32) -> f32,
            F2: Fn(f32,f32) -> f32,
            F3: Fn(f32,f32) -> f32,
    {
        if self.bars.len() <= staff { return; }
        let sr = track.sample_rate();
        let rout = ((sr as f32) * runout) as usize;
        let mut time = 0;
        for bar in &self.bars[staff]{
            let whole_note_time = (60.0 / bar.tempo) * 4.0; //tempo is quarter notes per minute
            for chord in &bar.notes{
                if chord.is_empty() { continue; }
                for (note,_,_) in chord{
                    if note < &0 { continue; } //rest or continued note in score
                    let hz = to_pitch(*note);
                    tone_to_track_stereo(track, time, rout, volume, pan, hz, samplef, volf, hzf, passf);
                }
                time += (chord[0].1 * whole_note_time * (sr as f32)) as usize;
            }
        }
    }

    pub fn as_string(&self, staff: usize) -> String{
        let mut builder = String::new();
        if self.bars.len() <= staff { return builder; }
        for bar in &self.bars[staff]{
            let string = bar.as_string();
            builder.push_str(&string);
            builder.push_str("\n");
        }
        builder
    }
}

#[derive(Clone,Copy)]
pub struct TimeSig{
    pub beats: u16,
    pub size: f32,
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
    CClef,
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

    pub fn std_key() -> Self{
        Self{
            base: NamedNote::G(4).to_note(),
            generator: Self::std_generator(),
        }
    }

    pub fn from_accidentals(clef: Clef, acc: Vec<Accidental>) -> Self{
        let mut gen = Self::std_generator();
        let x = std::cmp::min(gen.len(), acc.len());
        for i in 0..x{
            gen[i] = apply_accidental_global(gen[i], acc[i]);
        }
        let bnote = match clef {
            Clef::GClef => NamedNote::G(4).to_note(),
            Clef::FClef => NamedNote::F(3).to_note(),
            Clef::CClef => NamedNote::C(4).to_note(),
        };
        Self{
            base: bnote,
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

pub fn barnote(note: Note, d: f32) -> BarNote{
    (note,d, Vec::new())
}

/*
Tempo is quarter notes per minute.
Why?
We start on 5/4 for example and set a nice tempo.
We want an extra eighth note in the next bar.
Notate [5*2 + 1]/[4 * 2] = 11/8.
But the 8th is now the beat, if we want the quarter note to be the same lenght,
we need to do tempo =/ 2;
And i don't feel like doing that. 
*/

pub struct Bar{
    pub notes: Vec<Vec<BarNote>>,
    pub key: Key,
    pub tempo: f32,
    pub time_sig: TimeSig,
    time_left: f32,
}

impl Bar{
    pub fn new(key: Key, tempo: f32, time_sig: TimeSig) -> Self{
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
            key: Key::from_accidentals(clef, acc),
            tempo,
            time_sig,
            time_left: time_sig.total(),
        }
    }

    pub fn add_note(&mut self, (note,duration,effects): BarNote, increase_time: bool) -> Option<BarNote>{
        if increase_time{
            let diff = self.time_left - duration;
            if diff > -0.001{ // fits inside this bar still
                self.notes.push(vec![(note,duration,effects)]);
                self.time_left = diff;
                Option::None
            }else{
                self.notes.push(vec![(note,self.time_left,effects.clone())]);
                self.time_left = 0.0;
                Option::Some((CARRY_ON,-diff,effects))
            }
        }else{ // play notes in parralel, can go outside bar without carryover
            self.create_chord_if_none();
            let last = self.notes.len() - 1;
            self.notes[last].push((note,duration,effects));
            Option::None
        }
    }

    pub fn create_chord_if_none(&mut self){
        if self.notes.is_empty(){
            self.notes.push(Vec::new());
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

    pub fn as_string(&self) -> String{
        let mut string = format!("||{}", self.time_sig.beats);
        let size = (1.0 / self.time_sig.size).round() as i32;
        //println!("{}", size.to_string());
        string.push_str("/");
        string.push_str(&size.to_string());
        string.push_str("||");
        for chord in &self.notes{
            for (note,dur,_) in chord{
                let named = NamedNote::from_note(*note);
                string.push_str(&named.as_string());
                string.push_str("\\");
                string.push_str(&format!("{}", dur));
                string.push_str(", ");
            }
            for _ in 0..2 { string.pop(); }
            string.push_str(" | ")
        }
        for _ in 0..3 { string.pop(); }
        string.push_str("||");
        string
    }
}
