use super::note::*;
use super::interval::{PERFECT_OCTAVE};
use crate::theory::interval::to_relative_interval_non_nat;

pub type Mode = u8;

pub const TONIC: Note = 0;
pub const SUPER_TONIC: Note = 1;
pub const MEDIANT: Note = 2;
pub const SUB_DOMINANT: Note = 3;
pub const DOMINANT: Note = 4;
pub const SUB_MEDIANT: Note = 5;
pub const SUB_TONIC: Note = 6;

pub fn next_mode(mut scale: Notes) -> Notes{
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

pub fn mode_of(mut scale: Notes, mut mode: Mode) -> Notes{
    mode = mode % scale.len() as u8;
    for _ in 0..mode{
        scale = next_mode(scale)
    }
    scale
}

pub trait ModeTrait{
    fn next_mode_mut(&mut self);
    fn next_mode(self) -> Self;
    fn mode(self, mode: Mode) -> Self;
}

impl ModeTrait for Scale{
    fn next_mode_mut(&mut self){
        self.0.rotate_left(1);
    }

    fn next_mode(self) -> Self{
        Scale(next_mode(self.0))
    }

    fn mode(self, mode: Mode) -> Self{
        Scale(mode_of(self.0, mode))
    }
}

impl ModeTrait for Steps{
    fn next_mode_mut(&mut self){
        self.0.rotate_left(1);
    }

    fn next_mode(self) -> Self{
        Steps(next_mode(self.0))
    }

    fn mode(self, mode: Mode) -> Self{
        Steps(mode_of(self.0, mode))
    }
}

pub trait StepsTrait{
    fn to_scale(&self, note: Note) -> Scale;
    fn as_scale(self, note: Note) -> Scale;
    fn as_mode(self, note: Note, mode: Mode) -> Scale;
    fn mode_nr_of_this(self, mode: &Self) -> Option<(usize,Self)>
        where Self: std::marker::Sized;
    fn to_relative(&self, reference: &Steps) -> Option<Relative>;
}

impl StepsTrait for Steps{
    fn to_scale(&self, mut note: Note) -> Scale{
        let mut vec = Vec::new();
        vec.push(note);
        for step in &self.0{
            note += *step as Note;
            vec.push(note);
        }
        Scale(vec)
    }

    fn as_scale(self, note: Note) -> Scale{
        self.to_scale(note)
    }

    fn as_mode(self, note: Note, mode: Mode) -> Scale{
        self.mode(mode).as_scale(note)
    }

    fn mode_nr_of_this(mut self, mode: &Steps) -> Option<(usize,Steps)>{
        if mode.len() != self.len() {
            return Option::None;
        }
        let len = self.len();
        for i in 0..=len{
            if self.0 == mode.0{
                return Option::Some((i, self));
            }
            // self.next_mode_mut();
            self = self.next_mode();
        }
        Option::None
    }

    fn to_relative(&self, reference: &Steps) -> Option<Relative>{
        if self.0.len() != reference.0.len() { return None; }
        if self.0.len() == 0 { return None; }
        let mut acc_a = 0;
        let mut acc_b = 0;
        let mut res = Vec::new();
        for i in 0..self.0.len(){
            res.push(acc_a - acc_b);
            acc_a += self.0[i];
            acc_b += reference.0[i];
        }
        Some(Relative(res))
    }
}

pub trait RelativeTrait{
    fn string_ionian_rel(&self) -> String;
}

impl RelativeTrait for Relative{
    fn string_ionian_rel(&self) -> String{
        if self.0.len() != 7{
            String::from("Not a Ionian relative!")
        }else{
            let mut res = String::new();
            for i in 1..=7{
                let prefix = to_relative_interval_non_nat(self.0[i - 1]);
                res.push_str(&prefix);
                res.push_str(&format!("{} ", i));
            }
            res
        }
    }
}

pub fn notes_to_octave_scale(notes: &Notes) -> Notes{
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

pub struct ScaleIterator<'a>{
    scale: &'a Notes,
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

pub fn note_iter(root: Note, scale: &Notes) -> ScaleIterator{
    ScaleIterator{
        scale,
        current: 0,
        len: scale.len(),
        root,
    }
}

pub struct ModeIterator<T: ModeTrait + NoteSequence>{
    scale: T,
    current: usize,
    len: usize,
}
//TODO: return references
impl<T: std::clone::Clone + ModeTrait + NoteSequence>
    Iterator for ModeIterator<T>{
    type Item = T;
    fn next(&mut self) -> Option<T>{
        if self.current >= self.len{
            return Option::None;
        }
        let res = self.scale.clone();
        self.scale.next_mode_mut();
        self.current += 1;
        Option::Some(res)
    }
}

pub trait ModeIteratorSpawner<T: ModeTrait + NoteSequence>{
    fn mode_iter(self) -> ModeIterator<T>;
}

impl<T: ModeTrait + NoteSequence> ModeIteratorSpawner<T> for T{
    fn mode_iter(self) -> ModeIterator<T>{
        let len = self.len();
        ModeIterator{
            scale: self,
            current: 0,
            len: len,
        }
    }
}
