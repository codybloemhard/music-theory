use super::{ Note, PC };

// General Traits

pub trait Cyclic{
    fn next(self) -> Self;
    fn prev(self) -> Self;
}

// Conversion Traits

pub trait ToNote{
    fn to_note(self) -> Note;
}

pub trait ToPC{
    fn to_pc(self) -> PC;
}

