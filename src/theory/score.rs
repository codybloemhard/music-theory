pub type Score = Vec<Staff>;
pub type Staff = Vec<Bar>;

pub type TimedNote = (Note, f32);
pub type TimeSig = (u16,u16);

pub enum Clef{
    GClef,
    CClef(usize),
    FClef,
}

pub struct Bar{
    notes: Vec<TimedNote>,
    scale: Vec<Note>,
    tempo: f32,
    time_sig: TimeSig,
}

impl Bar{
    pub new(note_zero: Note, scale: Vec<Note>, tempo: f32, time_sig: TimeSig) -> Self{
        Self{
            notes: Vec::new(),
            scale,
            tempo,
            time_sig,
        }
    }

    /* pub new(clef: Clef, key: Vec<Accidental>, tempo: f32, time_sig: TimeSig) -> Self{
        
    } */
}
