use super::{ Note, PC, Octave, OctaveShift };

// General Traits

pub trait Cyclic{
    fn next(self) -> Self;
    fn prev(self) -> Self;
}

pub trait OctaveShiftable{
    fn with_octave(self, octave: Octave) -> Self;
    fn shift_octave(self, shift: OctaveShift) -> Self;
}

// Conversion Traits

pub trait ToNote{
    fn to_note(self) -> Note;
}

pub trait ToPC{
    fn to_pc(self) -> PC;
}

