use super::note::*;

pub const SEMI: u8 = 1;
pub const WHOLE: u8 = 2;

pub type Scale = Vec<u8>;
pub type Mode = u8;

pub const IONIAN: Mode = 0;
pub const DORIAN: Mode = 1;
pub const PHRYGIAN: Mode = 2;
pub const LYDIAN: Mode = 3;
pub const MIXOLYDIAN: Mode = 4;
pub const AEOLIAN: Mode = 5;
pub const LOCRIAN: Mode = 6;

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

pub fn mode_of_scale(mut scale: Scale, mut mode: Mode) -> Scale{
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

pub fn ionian_mode(note: Note, mode: Mode) -> Vec<Note>{
    let scale = mode_of_scale(ionian_scale_steps(), PHRYGIAN);
    scale_notes(&scale, note)
}

pub fn print_notes(notes: &Vec<Note>){
    for i in 0..notes.len()-1{
        print!("{},\t", to_note_name(notes[i]));
    }
    print!("{}\n", to_note_name(notes[notes.len()-1]));
}
