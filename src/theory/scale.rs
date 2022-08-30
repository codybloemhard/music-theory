use super::{ Note, _Note, Interval, PCs, Intervals, EnharmonicNote, Chord, RootedChord };
use super::traits::{
    Wrapper, VecWrapper, ModeTrait, AsScaleTry, AsSteps, AddInterval, ToPC, AsPCs,
    AsRelativeIntervals, AsEnharmonicNotes, AsEnharmonicNotesWithStart, Cyclic,
    ToEnharmonicNote, ToNote, ModeIteratorSpawner, AsChord, AsRootedChord
};

pub type Mode = usize;
pub type Notes = Vec<Note>;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scale(pub(crate) Notes);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Steps(pub(crate) Intervals);

impl Scale{
    pub fn as_octave_steps(&self) -> Option<Steps>{
        if self.is_empty(){ return None; }
        let mut res = Vec::new();
        let mut last = self[0];
        let mut sum = Interval::ROOT;
        for note in self.iter().skip(1){
            let diff = *note - last;
            if diff.0 < 0 { return None; }
            if diff.0 == 0 { continue; }
            res.push(diff);
            last = *note;
            sum += diff;
        }
        if sum > Interval::OCTAVE { return None; }
        if sum == Interval::OCTAVE {
            return Some(Steps(res));
        }
        res.push(Interval::OCTAVE - sum);
        Some(Steps(res))
    }
}

impl Steps{
    pub fn mode_nr_of_this(&self, mode: &Steps) -> Option<(usize, Steps)>{
        if mode.len() != self.len() {
            return None;
        }
        let len = self.len();
        let mut copy = self.clone();
        for i in 0..=len{
            if copy.0 == mode.0{
                return Some((i, copy));
            }
            copy.next_mode_mut();
        }
        None
    }
}

ImplVecWrapper!(Scale, Note);
ImplVecWrapper!(Steps, Interval);

impl Wrapper for Scale{
    type Inner = Notes;

    fn wrap(scale: Self::Inner) -> Option<Self>{
        if scale.is_empty(){
            None
        } else {
            Some(Self(scale))
        }
    }

    fn unwrap(self) -> Self::Inner{
        self.0
    }
}

impl Wrapper for Steps{
    type Inner = Intervals;

    fn wrap(steps: Self::Inner) -> Option<Self>{
        if steps.is_empty(){
            None
        } else {
            Some(Self(steps))
        }
    }

    fn unwrap(self) -> Self::Inner{
        self.0
    }
}

impl ModeTrait for Scale{
    fn next_mode_mut(&mut self){
        self.0.rotate_left(1);
    }

    fn next_mode(mut self) -> Self{
        self.next_mode_mut();
        self
    }

    fn mode(mut self, mode: Mode) -> Self{
        let len = self.len();
        self.0.rotate_left(mode % len);
        Scale(self.0)
    }
}

impl ModeTrait for Steps{
    fn next_mode_mut(&mut self){
        self.0.rotate_left(1);
    }

    fn next_mode(mut self) -> Self{
        self.next_mode_mut();
        self
    }

    fn mode(mut self, mode: Mode) -> Self{
        let len = self.len();
        self.0.rotate_left(mode % len);
        Steps(self.0)
    }
}

impl AsSteps for Scale{
    fn as_steps(&self, complete_octave_cycle: bool) -> Steps{
        if self.0.is_empty() { return Steps::default(); }
        let mut last = self[0];
        let mut intervals = Vec::new();
        for note in self.iter().skip(1){
            let diff = *note - last;
            intervals.push(diff);
            last = *note;
        }
        if complete_octave_cycle {
            intervals.push(self[0] - last + Interval::OCTAVE);
        }
        Steps(intervals)
    }
}

impl AsPCs for Scale{
    fn as_pcs(&self) -> PCs{
        let mut res = Vec::new();
        for n in &self.0{
            res.push(n.to_pc());
        }
        res
    }
}

impl AsChord for Scale{
    fn as_chord(&self) -> Chord{
        if self.is_empty() { return Chord(Vec::new()); }
        let root = self[0];
        let mut intervals = vec![];
        for note in self.iter().skip(1){
            let mut diff = note.0 as i32 - root.0 as i32;
            if diff == 0 { continue; }
            if diff < 0 {
                diff = (diff % 12) + 12;
            }
            intervals.push(Note(diff as _Note));
        }
        intervals.sort();
        Chord(intervals)
    }
}

// Could be used for hexatonics etc?
// fn _into_enharmonic_notes_with_start_subheptatonic(scale: Scale, start: Option<EnharmonicNote>) -> Vec<EnharmonicNote>{
//     let mut set = vec![0, 0, 0, 0, 0, 0, 0];
//     let mut res = Vec::new();
//     let skip = if let Some(en) = start{
//         set[en.letter as usize] = 1;
//         res.push(en);
//         1
//     } else {
//         0
//     };
//     for (i, note) in scale.0.into_iter().enumerate().skip(skip){
//         if i >= 7 { return Vec::new(); } // Impossible: no more letters.
//         let en = note.to_enharmonic_note().unwrap();
//         let en = if set[en.letter as usize] == 1{
//             let mut nen = en;
//             loop {
//                 nen = nen.next();
//                 if set[nen.letter as usize] == 0 { break nen; }
//             }
//         } else {
//             en
//         };
//         set[en.letter as usize] = 1;
//         res.push(en);
//     }
//     res
// }

fn as_enharmonic_notes_with_start_heptatonic(scale: &Scale, start: Option<EnharmonicNote>)
    -> Vec<EnharmonicNote>
{
    let mut res = Vec::new();
    if scale.is_empty() { return res; }
    let (skip, mut target_letter) = if let Some(en) = start{
        if en.to_note() != scale[0] { return res; }
        res.push(en);
        (1, en.next().letter)
    } else {
        (0, scale[0].to_enharmonic_note().letter)
    };
    for note in scale.iter().skip(skip){
        let en = note.to_enharmonic_note();
        let new_en = if en.letter == target_letter {
            en
        } else {
            en.spelled_as(target_letter)
        };
        res.push(new_en);
        target_letter = target_letter.next();
    }
    res
}

impl AsEnharmonicNotes for Scale{
    fn as_enharmonic_notes(&self) -> Vec<EnharmonicNote>{
        as_enharmonic_notes_with_start_heptatonic(self, None)
    }
}

impl AsEnharmonicNotesWithStart for Scale{
    fn as_enharmonic_notes_with_start(&self, start: Option<EnharmonicNote>) -> Vec<EnharmonicNote>{
        as_enharmonic_notes_with_start_heptatonic(self, start)
    }
}

impl AsRootedChord for Scale{
    fn as_rooted_chord(&self) -> RootedChord{
        if self.is_empty() { RootedChord{ root: Note::ZERO, chord: Chord(vec![]) } }
        else if self.len() == 1 { RootedChord{ root: self[0], chord: Chord(vec![]) } }
        else { RootedChord::from_chord(self[0], self.as_chord()) }
    }
}

impl AsScaleTry for Steps{
    fn as_scale_try(&self, mut note: Note) -> Option<Scale>{
        let mut vec = vec![note];
        if self.is_empty(){
            return Some(Scale(vec));
        }
        for step in self.iter().take(self.len() - 1){
            note = note.add_interval(*step)?;
            vec.push(note);
        }
        Some(Scale(vec))
    }
}

impl AsRelativeIntervals for Steps{
    fn as_relative_intervals(&self, reference: &Self) -> Option<Intervals>{
        if self.0.len() != reference.0.len() { return None; }
        if self.0.is_empty() { return None; }
        let mut acc_a = Interval(0);
        let mut acc_b = Interval(0);
        let mut res = Vec::new();
        for i in 0..self.0.len(){
            let diff = acc_a - acc_b;
            res.push(diff);
            acc_a += self[i];
            acc_b += reference[i];
        }
        Some(res)
    }
}

impl AsChord for Steps{
    fn as_chord(&self) -> Chord{
        if self.is_empty() { return Chord(Vec::new()); }
        let mut intervals = vec![];
        let mut acc = 0;
        for step in self.iter(){
            acc += step.0;
            if acc == 0 { continue; }
            intervals.push(Note((((acc % 12) + 12) % 12) as _Note));
        }
        let chord = Chord(intervals);
        chord.normalized()
    }
}

pub struct ScaleIterator<'a>{
    scale: &'a [Interval],
    current: usize,
    len: usize,
    root: Note,
}

impl<'a> Iterator for ScaleIterator<'a>{
    type Item = Note;

    fn next(&mut self) -> Option<Note>{
        if self.current >= self.len{
            self.current = 0;
        }
        let res = self.root;
        self.root = self.root.add_interval(self.scale[self.current])?;
        self.current += 1;
        Some(res)
    }
}

pub fn scale_iter(root: Note, scale: &[Interval]) -> ScaleIterator{
    ScaleIterator{
        scale,
        current: 0,
        len: scale.len(),
        root,
    }
}

pub struct ModeIterator<T: ModeTrait + VecWrapper>{
    wrapper: T,
    current: usize,
    len: usize,
}

// TODO: return references?
impl<T: Clone + ModeTrait + VecWrapper> Iterator for ModeIterator<T>{
    type Item = T;

    fn next(&mut self) -> Option<T>{
        if self.current >= self.len{
            return Option::None;
        }
        let res = self.wrapper.clone();
        self.wrapper.next_mode_mut();
        self.current += 1;
        Option::Some(res)
    }
}

impl<T: ModeTrait + VecWrapper> ModeIteratorSpawner<T> for T{
    fn mode_iter(self) -> ModeIterator<T>{
        let len = self.len();
        ModeIterator{
            wrapper: self,
            current: 0,
            len,
        }
    }
}

#[cfg(test)]
mod tests{
    use super::super::*;

    #[test]
    fn scale_wrap(){
        assert_eq!(Scale::wrap(vec![]), None);
        assert_eq!(Scale::wrap(vec![Note(0), Note(1)]), Some(Scale(vec![Note(0), Note(1)])));
    }

    #[test]
    fn scale_unwrap(){
        assert_eq!(Scale(vec![Note(0), Note(1)]).unwrap(), vec![Note(0), Note(1)]);
    }

    #[test]
    fn steps_wrap(){
        assert_eq!(Steps::wrap(vec![]), None);
        assert_eq!(
            Steps::wrap(vec![Interval(0), Interval(1)]), Some(Steps(vec![Interval(0), Interval(1)]))
        );
    }

    #[test]
    fn steps_unwrap(){
        assert_eq!(Steps(vec![Interval(2), Interval(1)]).unwrap(), vec![Interval(2), Interval(1)]);
    }

    #[test]
    fn scale_len(){
        assert_eq!(Scale(vec![Note(0), Note(1)]).len(), 2);
    }

    #[test]
    fn scale_is_empty(){
        assert!(!Scale(vec![Note(0)]).is_empty());
    }

    #[test]
    fn test_scale_iter(){
        let scale = Scale(vec![Note(0), Note(1), Note(2)]);
        let mut iter = scale.iter();
        assert_eq!(iter.next(), Some(&Note(0)));
        assert_eq!(iter.next(), Some(&Note(1)));
        assert_eq!(iter.next(), Some(&Note(2)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn scale_contains(){
        let scale = Scale(vec![Note(0), Note(1), Note(2)]);
        assert!(scale.contains(&Note(0)));
        assert!(scale.contains(&Note(1)));
        assert!(scale.contains(&Note(2)));
        assert!(!scale.contains(&Note(3)));
    }

    #[test]
    fn scale_contains_all(){
        let scale = Scale(vec![Note(0), Note(1), Note(2)]);
        assert!(scale.contains_all(&[Note(0)]));
        assert!(scale.contains_all(&[Note(0), Note(1)]));
        assert!(scale.contains_all(&[Note(0), Note(1), Note(2)]));
        assert!(!scale.contains_all(&[Note(0), Note(1), Note(2), Note(3)]));
    }

    #[test]
    fn scale_contains_any(){
        let scale = Scale(vec![Note(0), Note(1), Note(2)]);
        assert!(scale.contains_any(&[Note(0)]));
        assert!(scale.contains_any(&[Note(1)]));
        assert!(scale.contains_any(&[Note(2)]));
        assert!(scale.contains_any(&[Note(3245), Note(2)]));
        assert!(!scale.contains_any(&[Note(3)]));
    }

    #[test]
    fn scale_as_subs(){
        let (c, d, e) = (Note::C1, Note::D1, Note::E1);
        let scale = Scale(vec![c, d, e]);
        assert_eq!(
            scale.as_subs(None),
            vec![
                Scale(vec![]),
                Scale(vec![c]),
                Scale(vec![d]),
                Scale(vec![e]),
                Scale(vec![c, d]),
                Scale(vec![d, c]),
                Scale(vec![c, e]),
                Scale(vec![e, c]),
                Scale(vec![d, e]),
                Scale(vec![e, d]),
                Scale(vec![c, d, e]),
                Scale(vec![c, e, d]),
                Scale(vec![d, c, e]),
                Scale(vec![d, e, c]),
                Scale(vec![e, c, d]),
                Scale(vec![e, d, c]),
            ]
        );
    }

    #[test]
    fn scale_indexing(){
        let (c, d, e) = (Note::C1, Note::D1, Note::E1);
        let mut scale = Scale(vec![c, d, e]);
        assert_eq!(scale[0], c);
        assert_eq!(scale[1], d);
        assert_eq!(scale[2], e);
        scale[0] = d;
        assert_eq!(scale[0], d);
    }

    #[test]
    fn steps_len(){
        assert_eq!(Steps(vec![Interval(1), Interval(2)]).len(), 2);
    }

    #[test]
    fn steps_is_empty(){
        assert!(!Steps(vec![Interval(1)]).is_empty());
    }

    #[test]
    fn steps_iter(){
        let steps = Steps(vec![Interval(1), Interval(2), Interval(3)]);
        let mut iter = steps.iter();
        assert_eq!(iter.next(), Some(&Interval(1)));
        assert_eq!(iter.next(), Some(&Interval(2)));
        assert_eq!(iter.next(), Some(&Interval(3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn scale_next_mode_mut(){
        let mut scale = Scale(vec![Note(0), Note(1), Note(2)]);
        scale.next_mode_mut();
        assert_eq!(scale, Scale(vec![Note(1), Note(2), Note(0)]));
        scale.next_mode_mut();
        assert_eq!(scale, Scale(vec![Note(2), Note(0), Note(1)]));
        scale.next_mode_mut();
        assert_eq!(scale, Scale(vec![Note(0), Note(1), Note(2)]));
        let clone = scale.clone();
        scale.next_mode_mut();
        assert_eq!(scale, clone.next_mode());
    }

    #[test]
    fn scale_next_mode(){
        let mut scale = Scale(vec![Note(0), Note(1), Note(2)]);
        scale = scale.next_mode();
        assert_eq!(scale, Scale(vec![Note(1), Note(2), Note(0)]));
        scale = scale.next_mode();
        assert_eq!(scale, Scale(vec![Note(2), Note(0), Note(1)]));
        scale = scale.next_mode();
        assert_eq!(scale, Scale(vec![Note(0), Note(1), Note(2)]));
        let mut clone = scale.clone();
        clone.next_mode_mut();
        assert_eq!(scale.next_mode(), clone);
    }

    #[test]
    fn scale_mode(){
        let scale = Scale(vec![Note(0), Note(1), Note(2)]);
        assert_eq!(scale.clone().mode(0), scale);
        assert_eq!(scale.clone().mode(1), Scale(vec![Note(1), Note(2), Note(0)]));
        assert_eq!(scale.clone().mode(2), Scale(vec![Note(2), Note(0), Note(1)]));
        assert_eq!(scale.clone().mode(3), scale);
        assert_eq!(scale.clone().mode(1), scale.next_mode());
    }

    #[test]
    fn steps_next_mode_mut(){
        let mut steps = Steps(vec![Interval(1), Interval(2), Interval(3)]);
        steps.next_mode_mut();
        assert_eq!(steps, Steps(vec![Interval(2), Interval(3), Interval(1)]));
        steps.next_mode_mut();
        assert_eq!(steps, Steps(vec![Interval(3), Interval(1), Interval(2)]));
        steps.next_mode_mut();
        assert_eq!(steps, Steps(vec![Interval(1), Interval(2), Interval(3)]));
        let clone = steps.clone();
        steps.next_mode_mut();
        assert_eq!(steps, clone.next_mode());
    }

    #[test]
    fn steps_next_mode(){
        let mut steps = Steps(vec![Interval(1), Interval(2), Interval(3)]);
        steps = steps.next_mode();
        assert_eq!(steps, Steps(vec![Interval(2), Interval(3), Interval(1)]));
        steps = steps.next_mode();
        assert_eq!(steps, Steps(vec![Interval(3), Interval(1), Interval(2)]));
        steps = steps.next_mode();
        assert_eq!(steps, Steps(vec![Interval(1), Interval(2), Interval(3)]));
        let mut clone = steps.clone();
        clone.next_mode_mut();
        assert_eq!(steps.next_mode(), clone);
    }

    #[test]
    fn steps_mode(){
        let steps = Steps(vec![Interval(1), Interval(2), Interval(3)]);
        assert_eq!(steps.clone().mode(0), steps);
        assert_eq!(steps.clone().mode(1), Steps(vec![Interval(2), Interval(3), Interval(1)]));
        assert_eq!(steps.clone().mode(2), Steps(vec![Interval(3), Interval(1), Interval(2)]));
        assert_eq!(steps.clone().mode(3), steps);
        assert_eq!(steps.clone().mode(1), steps.next_mode());
    }

    #[test]
    fn steps_as_scale(){
        assert_eq!(
            Steps(vec![]).as_scale_try(Note(123)),
            Some(Scale(vec![Note(123)]))
        );
        assert_eq!( // C Major
            Steps(vec![Interval(2), Interval(2), Interval(1), Interval(2),
                        Interval(2), Interval(2), Interval(1)])
                .as_scale_try(PC::C.to_note()).unwrap().iter().map(|n| n.to_pc()).collect::<Vec<_>>(),
            vec![PC::C, PC::D, PC::E, PC::F, PC::G, PC::A, PC::B]
        );
        assert_eq!( // A Minor
            Steps(vec![Interval(2), Interval(2), Interval(1), Interval(2),
                        Interval(2), Interval(2), Interval(1)]).mode(5)
                .to_scale_try(PC::A.to_note()).unwrap().iter().map(|n| n.to_pc()).collect::<Vec<_>>(),
            vec![PC::A, PC::B, PC::C, PC::D, PC::E, PC::F, PC::G]
        );
    }

    #[test]
    fn scale_as_steps(){
        assert_eq!( // C Major
            Scale(vec![Note::C1, Note::D1, Note::E1, Note::F1, Note::G1, Note::A2, Note::B2])
                .as_steps(true),
            Steps(vec![Interval(2), Interval(2), Interval(1), Interval(2),
                        Interval(2), Interval(2), Interval(1)])
        );
        assert_eq!(
            Scale(vec![Note::A1, Note::B1, Note::C1, Note::D1, Note::E1, Note::F1, Note::G1])
                .as_steps(true),
            Steps(vec![Interval(2), Interval(2), Interval(1), Interval(2),
                        Interval(2), Interval(2), Interval(1)]).mode(5)
        );
        assert_eq!(
            Scale(vec![Note::A1, Note::B1, Note::A1, Note::B1]).to_steps(true),
            Steps(vec![Interval(2), Interval(-2), Interval(2), Interval(10)])
        );
        assert_eq!(
            Scale(vec![Note::A1, Note::B1, Note::A1, Note::B1]).to_steps(false),
            Steps(vec![Interval(2), Interval(-2), Interval(2)])
        );
    }

    #[test]
    fn scale_as_pcs(){
        assert_eq!(
            Scale(vec![Note::C1, Note::E1, Note::G1, Note::C2]).as_pcs(),
            vec![PC::C, PC::E, PC::G, PC::C]
        );
        assert_eq!(
            Scale(vec![Note::A1, Note::C1, Note::D1, Note::F1, Note::A2]).to_pcs(),
            vec![PC::A, PC::C, PC::D, PC::F, PC::A]
        );
    }

    #[test]
    fn scale_as_chord(){
        assert_eq!(Scale(vec![]).to_chord(), Chord(vec![]));
        assert_eq!(
            Scale(vec![Note::C1, Note::E1, Note::G1]).as_chord(),
            Chord::new(MAJOR)
        );
        assert_eq!(
            Scale(vec![Note::C1, Note::B1, Note::E1, Note::G1]).as_chord(),
            Chord::new(MAJOR_SEVENTH_CHORD)
        );
        assert_eq!(
            Scale(vec![Note::C1, Note::E1, Note::G1, Note::B2]).as_chord(),
            Chord::new(MAJOR_SEVENTH_CHORD)
        );
    }

    #[test]
    fn mode_nr_of_this(){
        let major = Steps(vec![Interval(2), Interval(2), Interval(1), Interval(2),
            Interval(2), Interval(2), Interval(1)]);
        let minor = major.clone().mode(5);
        assert_eq!(
            major.mode_nr_of_this(&minor),
            Some((5, Steps(vec![
                Interval(2), Interval(1), Interval(2), Interval(2),
                Interval(1), Interval(2), Interval(2)
            ])))
        );
        assert_eq!(
            Steps(vec![Interval(1), Interval(1)]).mode_nr_of_this(&Steps(vec![Interval(1)])),
            None
        );
        assert_eq!(
            Steps(vec![Interval(1), Interval(2)])
                .mode_nr_of_this(&Steps(vec![Interval(2), Interval(2)])),
            None
        );
    }

    #[test]
    fn steps_as_relative_intervals(){
        let a = Steps(vec![Interval(2), Interval(2), Interval(1), Interval(2),
            Interval(2), Interval(2), Interval(1)]);
        let b = Steps(vec![Interval(3), Interval(2), Interval(1), Interval(1),
            Interval(2), Interval(2), Interval(1)]);
        assert_eq!(
            a.to_relative_intervals(&b),
            Some(vec![Interval(0), Interval(-1), Interval(-1), Interval(-1),
                        Interval(0), Interval(0), Interval(0)])
        );

        let scalea = vec![PC::C, PC::D, PC::E, PC::F, PC::G, PC::A, PC::B]
            .to_scale_try(Note(1)).unwrap().to_steps(true);
        let scaleb = vec![PC::A, PC::B, PC::C, PC::D, PC::E, PC::F, PC::G]
            .to_scale_try(Note(1)).unwrap().to_steps(true);
        assert_eq!(
            scaleb.to_relative_intervals(&scalea),
            Some(vec![Interval(0), Interval(0), Interval(-1), Interval(0),
                    Interval(0), Interval(-1), Interval(-1)])
        );
    }

    #[test]
    fn steps_as_chord(){
        assert_eq!(Steps(vec![]).to_chord(), Chord(vec![]));
        assert_eq!(
            Steps(vec![
                Interval(4), Interval(3), Interval(4)
            ]).to_chord(),
            Chord::new(MAJOR_SEVENTH_CHORD)
        );
        assert_eq!(
            Steps(vec![
                Interval(0), Interval(4), Interval(3), Interval(4)
            ]).to_chord(),
            Chord::new(MAJOR_SEVENTH_CHORD)
        );
    }

    #[test]
    fn scale_as_enharmonic_notes(){
        assert_eq!(Scale(vec![]).to_enharmonic_notes(), vec![]);
        assert_eq!(
            Scale(vec![
                  Note(0), Note(1), Note(2), Note(3)
            ]).to_enharmonic_notes(),
            vec![
                EnharmonicNote{ letter: Letter::A, accidental: Interval(0) },
                EnharmonicNote{ letter: Letter::B, accidental: Interval(-1) },
                EnharmonicNote{ letter: Letter::C, accidental: Interval(-1) },
                EnharmonicNote{ letter: Letter::D, accidental: Interval(-2) },
            ]
        );
    }

    #[test]
    fn scale_as_enharmonic_notes_with_start(){
        assert_eq!(
            Scale(vec![]).to_enharmonic_notes_with_start(
                Some(EnharmonicNote{ letter: Letter::A, accidental: Interval(0) })
            ),
            vec![]
        );
        let scale = Scale(vec![Note(0), Note(1), Note(2), Note(3)]);
        assert_eq!(
            scale.clone().to_enharmonic_notes(),
            scale.to_enharmonic_notes_with_start(
                Some(EnharmonicNote{ letter: Letter::A, accidental: Interval(0) })
            )
        );
        assert_eq!(
            Scale(vec![
                  Note(10), Note(11), Note(12), Note(13)
            ]).to_enharmonic_notes_with_start(
                Some(EnharmonicNote { letter: Letter::G, accidental: Interval(0) })
            ),
            vec![
                EnharmonicNote { letter: Letter::G, accidental: Interval(0) },
                EnharmonicNote { letter: Letter::A, accidental: Interval(-1) },
                EnharmonicNote { letter: Letter::B, accidental: Interval(-2) },
                EnharmonicNote { letter: Letter::C, accidental: Interval(-2) }
            ]
        );
        assert_eq!(
            Scale(vec![
                  Note(10), Note(11), Note(12), Note(13)
            ]).to_enharmonic_notes_with_start(
                Some(EnharmonicNote { letter: Letter::F, accidental: Interval(2) })
            ),
            vec![
                EnharmonicNote { letter: Letter::F, accidental: Interval(2) },
                EnharmonicNote { letter: Letter::G, accidental: Interval(1) },
                EnharmonicNote { letter: Letter::A, accidental: Interval(0) },
                EnharmonicNote { letter: Letter::B, accidental: Interval(-1) }
            ]
        );
        assert_eq!(
            Scale(vec![
                  Note(10), Note(11), Note(12), Note(13)
            ]).to_enharmonic_notes_with_start(
                Some(EnharmonicNote { letter: Letter::F, accidental: Interval(0) })
            ),
            vec![]
        );
    }

    #[test]
    fn mode_iterator(){
        let mut iter = Steps(vec![Interval(1), Interval(2), Interval(3)]).mode_iter();
        assert_eq!(iter.next(), Some(Steps(vec![Interval(1), Interval(2), Interval(3)])));
        assert_eq!(iter.next(), Some(Steps(vec![Interval(2), Interval(3), Interval(1)])));
        assert_eq!(iter.next(), Some(Steps(vec![Interval(3), Interval(1), Interval(2)])));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn scale_iterator(){
        let major = crate::libr::ionian::steps();
        let mut iter = scale_iter(Note::C1, &major.0);
        assert_eq!(iter.next(), Some(Note::C1));
        assert_eq!(iter.next(), Some(Note::D1));
        assert_eq!(iter.next(), Some(Note::E1));
        assert_eq!(iter.next(), Some(Note::F1));
        assert_eq!(iter.next(), Some(Note::G1));
        assert_eq!(iter.next(), Some(Note::A2));
        assert_eq!(iter.next(), Some(Note::B2));
        assert_eq!(iter.next(), Some(Note::C2));
        assert_eq!(iter.next(), Some(Note::D2));
    }

    #[test]
    fn scale_as_octave_steps(){
        assert_eq!( // C Major
            Scale(vec![Note::C1, Note::D1, Note::E1, Note::F1, Note::G1, Note::A2, Note::B2])
                .as_octave_steps(),
            Some(Steps(vec![Interval(2), Interval(2), Interval(1), Interval(2),
                        Interval(2), Interval(2), Interval(1)]))
        );
        assert_eq!(
            Scale(vec![Note::A1, Note::B1, Note::C1, Note::D1, Note::E1, Note::F1, Note::G1])
                .as_octave_steps(),
            Some(Steps(vec![Interval(2), Interval(2), Interval(1), Interval(2),
                        Interval(2), Interval(2), Interval(1)]).mode(5))
        );
        assert_eq!(
            Scale(vec![Note::A1, Note::B1, Note::A1, Note::B1]).as_octave_steps(),
            None
        );
        assert_eq!(
            Scale(vec![Note::A1, Note::B1, Note::A1, Note::B1]).as_octave_steps(),
            None
        );
        assert_eq!( // C Major
            Scale(vec![Note::C1, Note::D1, Note::E1, Note::F1, Note::G1, Note::A2, Note::B2, Note::C2])
                .as_octave_steps(),
            Some(Steps(vec![Interval(2), Interval(2), Interval(1), Interval(2),
                        Interval(2), Interval(2), Interval(1)]))
        );
    }
}
