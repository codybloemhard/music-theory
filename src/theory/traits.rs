use super::{ Note, PC };

// Convertion Traits

pub trait ToNote{
    fn to_note(self) -> Note;
}

pub trait ToPC{
    fn to_pc(self) -> PC;
}

