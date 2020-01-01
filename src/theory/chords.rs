use super::note::*;
use super::intervals::*;
use super::scale::*;

type Chord = Vec<Note>;

pub fn major_chord(base: Note) -> Chord{
    vec![base, base + MAJOR_THIRD, base + PERFECT_FIFTH]
}

pub fn minor_chord(base: Note) -> Chord{
    vec![base, base + MINOR_THIRD, base + PERFECT_FIFTH]
}

pub fn chord_std_from_notes(base: Note, scale: &Scale, max_size: usize) -> Chord{
    let mut chord = vec![base];
    for (i,note) in scale.iter().enumerate(){
        if i % 2 == 0 { continue; }
        if chord.len() >= max_size { break; }
        chord.push(*note);
    }
    chord
}

pub fn chord_std_from_scale(base: Note, scale: &Scale, size: usize) -> Chord{
    let scale_len = scale.len();
    let mut chord = Vec::new();
    let mut i = 0;
    let mut note = base;
    loop{
        if i % 2 == 1 { 
            i += 1;
            continue;
        }
        if chord.len() >= size { break; }
        chord.push(note);
        note += scale[i % scale_len];
        i += 1;
    }
    chord
}
