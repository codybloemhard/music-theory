use super::note::*;
use super::interval::*;
use super::scale::*;

type Chord = Vec<Note>;

pub fn major_chord(base: Note) -> Chord{
    vec![base, base + MAJOR_THIRD, base + PERFECT_FIFTH]
}

pub fn minor_chord(base: Note) -> Chord{
    vec![base, base + MINOR_THIRD, base + PERFECT_FIFTH]
}

pub fn chord_from_scale(base: Note, scale: &Scale, degrees: &Vec<usize>) -> Chord{
    let slen = scale.len();
    let mut chord = Vec::new();
    let mut i = 1;
    let mut note = base;
    let mut index = 0;
    loop{
        if index >= degrees.len(){
            break;
        }
        let d = degrees[index];
        if i == d{
            chord.push(note);
            index += 1;
        }
        note += scale[(i - 1) % slen];
        i += 1;
    }
    chord
}
