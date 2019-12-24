type Note = u16;

#[derive(Clone,Copy)]
pub enum NoteName{
    A, As, B, C, Cs, D, Ds, E, F, Fs, G, Gs,
}

pub const NOTE_NAMES: [NoteName; 12] = [NoteName::A,NoteName::As,NoteName::B,NoteName::C,NoteName::Cs,NoteName::D,NoteName::Ds,NoteName::E,NoteName::F,NoteName::Fs,NoteName::G,NoteName::Gs];

/*
0   1   2   3   4   5   6   7   8   9   10  11  // rank 0
12  13  14  15  16  17  18  19  20  21  22  23  // rank 1
24  25  26  27  28  29  30  31  32  33  34  35  // rank 2
36  37  38  39  40  41  42  43  44  45  46  47  // rank 3
48                                              // A4
*/

// note 48 (12*4) is A4 at 440 hz
pub fn to_pitch(note: Note) -> f32{
    let x = note as i32 - 49;
    (2.0f32).powf(note as f32 / 12.0) * 440.0
}

pub fn to_note_name(note: Note) -> NoteName{
    let inrank = note % 12;
    NOTE_NAMES[inrank as usize]
}

pub fn to_note_rank(note: Note) -> {
    note / 12
}

pub fn to_name_rank(note: Note) -> (NoteName, u16){
    (to_note_name(note), to_note_rank(note))
}

pub fn dodeca_scale_index(name: NoteName) -> u16{
    match name{
        NoteName::A     => 0,
        NoteName::As    => 1,
        NoteName::B     => 2,
        NoteName::C     => 3,
        NoteName::Cs    => 4,
        NoteName::D     => 5,
        NoteName::Ds    => 6,
        NoteName::E     => 7,
        NoteName::F     => 8,
        NoteName::Fs    => 9,
        NoteName::G     => 10,
        NoteName::Gs    => 11,
    }
}

pub fn to_note(name: NoteName, rank: u16) -> Note{
    (rank * 12) + 
}
