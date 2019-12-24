use super::note::*;

pub const SEMI: u8 = 1;
pub const WHOLE: u8 = 2;

pub type Scale = Vec<u8>;

pub const IONIAN: u8 = 0;
pub const DORIAN: u8 = 1;
pub const PHRYGIAN: u8 = 2;
pub const LYDIAN: u8 = 3;
pub const MIXOLYDIAN: u8 = 4;
pub const AEOLIAN: u8 = 5;
pub const LOCRIAN: u8 = 6;

pub fn ionian_scale_steps() -> Scale{
    vec![WHOLE,WHOLE,SEMI,WHOLE,WHOLE,WHOLE,SEMI]
}

pub fn next_mode(mut scale: Scale) -> Scale{
    let len = scale.len();
    if len == 0{
        panic!("panic: Scales cannot have 0 steps!");
    }
    let head = scale[0];
    for i in 0..len - 1{
        scale[i] = scale[i + 1];
    }
    scale[len - 1] = head;
    scale
}

pub fn mode_of_scale(mut scale: Scale, mut mode: u8) -> Scale{
    mode = mode % 7;
    for i in 0..mode{
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
