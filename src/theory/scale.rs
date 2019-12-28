use super::note::*;

pub type Scale = Vec<Note>;
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
/* 
Old Greek Dorian mode.
A 7 note scale in a octave of 2 four-note segments separated by a whole tone.
quarter,quarter,major third,whole,quarter,quarter,major third.
1/4 + 1/4 + 2 + 1 + 1/4 + 1/4 + 2 = 6 whole tones = 12 semitones = 1 octave
https://en.wikipedia.org/wiki/Dorian_mode
*/
pub fn greek_dorian_enharmonic() -> Scale{
    vec![QUAD,QUAD,MAJOR_THIRD,WHOLE,QUAD,QUAD,MAJOR_THIRD]
}

pub fn greek_dorian_chromatic() -> Scale{
    vec![SEMI,SEMI,MINOR_THIRD,WHOLE,SEMI,SEMI,MINOR_THIRD]
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

pub fn ionian_mode(note: Note, mode: Mode) -> Vec<Note>{
    let scale = mode_of_scale(ionian_scale_steps(), mode);
    scale_notes(&scale, note)
}

pub fn notes_of_mode(note: Note, scale: Scale, mode: Mode) -> Vec<Note>{
    let scale = mode_of_scale(scale, mode);
    scale_notes(&scale, note)
}

pub fn print_notes(notes: &Vec<Note>){
    for i in 0..notes.len()-1{
        print!("{},\t", NamedNote::from_note(notes[i]));
    }
    print!("{}\n", NamedNote::from_note(notes[notes.len()-1]));
}