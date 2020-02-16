use super::note::*;
use super::interval::{PERFECT_OCTAVE};

pub type Scale = Vec<Note>;
pub type Mode = u8;

pub const TONIC: Note = 0;
pub const SUPER_TONIC: Note = 1;
pub const MEDIANT: Note = 2;
pub const SUB_DOMINANT: Note = 3;
pub const DOMINANT: Note = 4;
pub const SUB_MEDIANT: Note = 5;
pub const SUB_TONIC: Note = 6;

pub fn next_mode(mut scale: Scale) -> Scale{
    let len = scale.len();
    if len == 0{
        return scale;
    }
    let head = scale[0];
    for i in 0..len - 1{
        scale[i] = scale[i + 1];
    }
    scale[len - 1] = head;
    scale
}

pub fn mode_of_scale(mut scale: Scale, mut mode: Mode) -> Scale{
    mode = mode % scale.len() as u8;
    for _ in 0..mode{
        scale = next_mode(scale)
    }
    scale
}

pub fn scale_notes(scale: &Scale, mut note: Note) -> Vec<Note>{
    let mut vec = Vec::new();
    vec.push(note);
    for step in scale{
        note += *step as Note;
        vec.push(note);
    }
    vec
}

pub fn notes_of_mode(note: Note, scale: Scale, mode: Mode) -> Vec<Note>{
    let scale = mode_of_scale(scale, mode);
    scale_notes(&scale, note)
}

pub fn notes_to_octave_scale(notes: &Scale) -> Scale{
    let mut res = Vec::new();
    if notes.is_empty(){ return res; }
    let mut last = notes[0];
    let mut sum = 0;
    for note in notes.iter().skip(1){
        let diff = note - last;
        res.push(diff);
        last = *note;
        sum += diff;
    }
    if sum > PERFECT_OCTAVE{
        return Vec::new();
    }
    if sum == PERFECT_OCTAVE{
        return res;
    }
    res.push(PERFECT_OCTAVE - sum);
    res
}

pub fn mode_nr_of_scale(input: &Scale, scale: Scale) -> Option<usize>{
    if input.len() != scale.len() {
        return Option::None;
    }
    let mut mode = scale;
    let len = mode.len();
    for i in 0..=len{
        if &mode == input{
            return Option::Some(i);
        }
        mode = next_mode(mode);
    }
    Option::None
}

pub struct ScaleIterator<'a>{
    scale: &'a Scale,
    current: usize,
    len: usize,
    root: Note,
}

impl<'a> Iterator for ScaleIterator<'a>{
    type Item = Note;
    fn next(&mut self) -> Option<Note>{
        if self.current >= self.len{
            self.current = 0;
        }
        let res = self.root;
        self.root += self.scale[self.current];
        self.current += 1;
        Some(res)
    }
}

pub fn note_iter(root: Note, scale: &Scale) -> ScaleIterator{
    ScaleIterator{
        scale,
        current: 0,
        len: scale.len(),
        root,
    }
}
