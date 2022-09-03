#[macro_use]
pub mod traits;
/// A single musical note.
/// This is one of the core values used in computations.
/// It does not take enharmonic spelling into account.
pub mod note;
/// PC stands for Pitch Class.
pub mod pc;
/// Scales are lists of notes. Variants are `Scale` and `Steps`.
pub mod scale;
/// Chords are stacks of notes. Variants are `Chord`, `RootedChord` and `RelativeChord`.
pub mod chord;
/// An interval is a distance between notes.
pub mod interval;
/// An enharmonic note is a note that takes into account enharmonic spelling.
pub mod enharmonic_note;

pub use traits::*;
pub use note::*;
pub use pc::*;
pub use scale::*;
pub use chord::*;
pub use interval::*;
pub use enharmonic_note::*;
