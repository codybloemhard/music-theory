use super::{ Note, Interval, PCs, _OCTAVE, _SEMI };
use super::traits::{ Wrapper, VecWrapper, ModeTrait, AsScaleTry, AsSteps, AddInterval, ToPC, AsPCs };

pub type Mode = usize;
pub type Notes = Vec<Note>;
pub type Intervals = Vec<Interval>;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scale(pub(crate) Notes);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Steps(pub(crate) Intervals);

impl Steps{
    fn mode_nr_of_this(&self, mode: &Steps) -> Option<usize>{
        if mode.len() != self.len() {
            return None;
        }
        let len = self.len();
        let mut copy = self.clone();
        for i in 0..=len{
            if copy.0 == mode.0{
                return Some(i);
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
        let mut last = self.0[0];
        let mut intervals = Vec::new();
        for note in self.iter().skip(1){
            let diff = *note - last;
            intervals.push(diff);
            last = *note;
        }
        if complete_octave_cycle {
            intervals.push(self.0[0] - last + Interval::OCTAVE);
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

// impl ToChord for Scale{
//     fn to_chord(&self) -> Chord{
//         if self.0.is_empty() { return Chord(Vec::new()); }
//         let root = self.0[0];
//         let mut intervals = vec![];
//         for note in self.0.iter().skip(1){
//             let diff = note - root;
//             intervals.push(diff);
//         }
//         Chord(intervals)
//     }
// }

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

// impl ToRelative for Steps{
//     fn to_relative(&self, reference: &Steps) -> Option<Relative>{
//         if self.0.len() != reference.0.len() { return None; }
//         if self.0.is_empty() { return None; }
//         let mut acc_a = 0;
//         let mut acc_b = 0;
//         let mut res = Vec::new();
//         for i in 0..self.0.len(){
//             let diff = (acc_a - acc_b) / _SEMI;
//             if diff.abs() > 255 { return None; }
//             let rn = match diff.cmp(&0){
//                 Ordering::Greater => { RelativeNote::Sharp(diff.unsigned_abs()) },
//                 Ordering::Less => { RelativeNote::Flat(diff.unsigned_abs()) },
//                 Ordering::Equal => { RelativeNote::Natural },
//             };
//             res.push(rn);
//             acc_a += self.0[i];
//             acc_b += reference.0[i];
//         }
//         Some(Relative(res))
//     }
// }

// pub trait RelativeTrait{
//     fn string_ionian_rel(&self) -> String;
// }
//
// impl RelativeTrait for Relative{
//     fn string_ionian_rel(&self) -> String{
//         if self.0.len() != 7{
//             String::from("Not a Ionian relative!")
//         }else{
//             let mut res = String::new();
//             for i in 1..=7{
//                 let prefix = self.0[i - 1].to_string();
//                 res.push_str(&prefix);
//                 res.push_str(&format!("{} ", i));
//             }
//             res
//         }
//     }
// }
//
// pub fn notes_to_octave_scale(scale: &Scale) -> Notes{
//     let mut res = Vec::new();
//     if scale.0.is_empty(){ return res; }
//     let mut last = scale.0[0];
//     let mut sum = 0;
//     for note in scale.0.iter().skip(1){
//         let diff = note - last;
//         res.push(diff);
//         last = *note;
//         sum += diff;
//     }
//     if sum > _OCTAVE{
//         return Vec::new();
//     }
//     if sum == _OCTAVE{
//         return res;
//     }
//     res.push(_OCTAVE - sum);
//     res
// }
//
// pub struct ScaleIterator<'a>{
//     scale: &'a [Note],
//     current: usize,
//     len: usize,
//     root: Note,
// }
//
// impl<'a> Iterator for ScaleIterator<'a>{
//     type Item = Note;
//     fn next(&mut self) -> Option<Note>{
//         if self.current >= self.len{
//             self.current = 0;
//         }
//         let res = self.root;
//         self.root += self.scale[self.current];
//         self.current += 1;
//         Some(res)
//     }
// }
//
// pub fn note_iter(root: Note, scale: &[Note]) -> ScaleIterator{
//     ScaleIterator{
//         scale,
//         current: 0,
//         len: scale.len(),
//         root,
//     }
// }
//
// pub struct ModeIterator<T: ModeTrait + NoteSequence>{
//     scale: T,
//     current: usize,
//     len: usize,
// }
//
// //TODO: return references
// impl<T: std::clone::Clone + ModeTrait + NoteSequence>
//     Iterator for ModeIterator<T>{
//     type Item = T;
//     fn next(&mut self) -> Option<T>{
//         if self.current >= self.len{
//             return Option::None;
//         }
//         let res = self.scale.clone();
//         self.scale.next_mode_mut();
//         self.current += 1;
//         Option::Some(res)
//     }
// }
//
// pub trait ModeIteratorSpawner<T: ModeTrait + NoteSequence>{
//     fn mode_iter(self) -> ModeIterator<T>;
// }
//
// impl<T: ModeTrait + NoteSequence> ModeIteratorSpawner<T> for T{
//     fn mode_iter(self) -> ModeIterator<T>{
//         let len = self.len();
//         ModeIterator{
//             scale: self,
//             current: 0,
//             len,
//         }
//     }
// }

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
        assert_eq!(Scale(vec![Note(0)]).is_empty(), false);
    }

    #[test]
    fn scale_iter(){
        let scale = Scale(vec![Note(0), Note(1), Note(2)]);
        let mut iter = scale.iter();
        assert_eq!(iter.next(), Some(&Note(0)));
        assert_eq!(iter.next(), Some(&Note(1)));
        assert_eq!(iter.next(), Some(&Note(2)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn steps_len(){
        assert_eq!(Steps(vec![Interval(1), Interval(2)]).len(), 2);
    }

    #[test]
    fn steps_is_empty(){
        assert_eq!(Steps(vec![Interval(1)]).is_empty(), false);
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
        assert_eq!(scale.clone().mode(1), scale.clone().next_mode());
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
        assert_eq!(steps.clone().mode(1), steps.clone().next_mode());
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
    fn mode_nr_of_this(){
        let major = Steps(vec![Interval(2), Interval(2), Interval(1), Interval(2),
            Interval(2), Interval(2), Interval(1)]);
        let minor = major.clone().mode(5);
        assert_eq!(major.mode_nr_of_this(&minor), Some(5));
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
}
