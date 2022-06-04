use super::{ Note, PC, Octave, OctaveShift };

// General Traits

// You always know next, prev and it goes round n round
pub trait Cyclic{
    fn next(self) -> Self;
    fn prev(self) -> Self;
}

// Possible to generate next, prev, might fail
pub trait GeneratablePartialOrder where Self: std::marker::Sized{
    fn next(self) -> Option<Self>;
    fn prev(self) -> Option<Self>;
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

