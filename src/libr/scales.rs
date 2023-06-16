use crate::theory::{
    traits::{ ModeIteratorSpawner, VecWrapper, Wrapper },
    Steps, Mode, Interval, Scale, AsScaleTry, ToScaleTry, Note,
};

use std::fmt::Write;

/// Scale Object.
/// Contains information on a scale family.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScaleObj{
    /// Steps describe the base scale in the family.
    pub steps: Steps,
    /// Family name of this scale family.
    pub fam_name: String,
    /// Names of the modes of the scale family.
    pub modes: Vec<String>,
}

/// Mode Object.
/// Contains information on a mode.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModeObj{
    /// Steps describe the scale.
    pub steps: Steps,
    /// Family name of the scale family it comes from.
    pub fam_name: String,
    /// Name of this mode.
    pub mode_name: String,
    /// The mode number this is in the scale family it comes from.
    pub mode_nr: usize,
}

impl ScaleObj{
    /// Get a clone of the steps.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// use music_theory::libr::scales::*;
    /// let major = ionian::obj();
    /// let steps = major.clone_steps();
    /// assert_eq!(steps.len(), 7);
    /// ```
    pub fn clone_steps(&self) -> Steps{
        self.steps.clone()
    }

    /// Get a clone of the family name.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// use music_theory::libr::scales::*;
    /// let major = ionian::obj();
    /// assert_eq!(&major.family_name(), "Ionian");
    /// ```
    pub fn family_name(&self) -> String{
        self.fam_name.clone()
    }

    /// Get the name of the Nth mode of this scale family.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// use music_theory::libr::scales::*;
    /// let major = ionian::obj();
    /// assert_eq!(&major.get_mode_name(2), "Phrygian");
    /// ```
    pub fn get_mode_name(&self, mode: Mode) -> String{
        let m = mode % self.steps.len();
        let name = self.modes[m].clone();
        if name.is_empty(){
            String::from("")
        } else {
            name
        }
    }

    /// Get all the modes as a vector of [ModeObj][ModeObj].
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// use music_theory::libr::scales::*;
    /// let modes = ionian::obj().get_modes();
    /// assert_eq!(&modes[6].mode_name, "Locrian");
    /// ```
    pub fn get_modes(self) -> Vec<ModeObj>{
        let fname = self.family_name();
        let mut res = Vec::new();
        for (i, mode) in self.clone_steps().mode_iter().enumerate(){
            res.push(
                ModeObj{
                    steps: mode,
                    fam_name: fname.clone(),
                    mode_name: self.get_mode_name(i),
                    mode_nr: i,
                }
            );
        }
        res
    }
}

impl std::fmt::Display for ModeObj{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}, {}ᵉ mode of {}", self.mode_name, self.mode_nr + 1, self.fam_name)
    }
}

/// Get all the scale objects in the library in a vector.
///
/// Example:
/// ```
/// use music_theory::theory::*;
/// use music_theory::libr::scales::*;
/// let objs = get_all_scale_objs();
/// assert_eq!(objs.len(), 10); // there are ten scale families in the library currently.
/// ```
pub fn get_all_scale_objs() -> Vec<ScaleObj>{
    vec![
        ionian::obj(),
        harmonic_minor::obj(), harmonic_major::obj(),
        melodic_minor::obj(),
        byzantine::obj(), hungarian_major::obj(),
        neapolitan_minor::obj(), neapolitan_major::obj(),
        enigmatic_major::obj(), enigmatic_minor::obj()
    ]
}

macro_rules! def_scale{
    ($modname:ident, $name:expr, $steps:expr, $( $mode:expr ),* ) => {
        /// A module named after a scale family with functions to get it's data.
        pub mod $modname{
            use crate::theory::{ Steps };
            use super::*;

            /// Get the steps of this scale family.
            pub fn steps() -> Steps{
                Steps($steps)
            }

            /// Get the [ScaleObj][ScaleObj] of this scale family.
            pub fn obj() -> ScaleObj{
                let modes = vec![
                    $(
                        String::from($mode),
                    )*
                ];
                ScaleObj{
                    steps: steps(),
                    fam_name: String::from($name),
                    modes,
                }
            }
        }
    }
}

const SEMI: Interval = Interval::SEMI;
const WHOLE: Interval = Interval::WHOLE;
const MIN3: Interval = Interval::MIN3;

def_scale!(ionian, "Ionian",
    vec![WHOLE, WHOLE, SEMI, WHOLE, WHOLE, WHOLE, SEMI],
    "Ionian", "Dorian", "Phrygian", "Lydian", "Mixolidian", "Aeolian", "Locrian"
);

def_scale!(harmonic_minor, "Harmonic Minor",
    vec![WHOLE, SEMI, WHOLE, WHOLE, SEMI, MIN3, SEMI],
    "Harmonic Minor", "", "", "", "Phrygian Dominant", "", "Superlocrian"
);

def_scale!(harmonic_major, "Harmonic Major",
    vec![WHOLE, WHOLE, SEMI, WHOLE, SEMI, MIN3, SEMI],
    "Harmonic Major" ,"", "Super Phrygian", "Lydian Diminished", "", "" ,""
);

def_scale!(byzantine, "Double Harmonic Major",
    vec![SEMI, MIN3, SEMI, WHOLE, SEMI, MIN3, SEMI],
    "Byzantine", "", "Ultra Phrygian", "Hungarian Minor", "Oriental", "", ""
);

def_scale!(hungarian_major, "Hungarian Major",
    vec![MIN3, SEMI, WHOLE, SEMI, WHOLE, SEMI, WHOLE],
    "Hungarian Major", "", "", "", "", "", ""
);

def_scale!(neapolitan_minor, "Neapolitan Minor",
    vec![SEMI, WHOLE, WHOLE, WHOLE, SEMI, MIN3, SEMI],
    "Neapolitan Minor", "", "Mixolydian Augmented", "Lydian Minor", "", "", ""
);

def_scale!(neapolitan_major, "Neapolitan Major",
    vec![SEMI, WHOLE, WHOLE, WHOLE, WHOLE, WHOLE, SEMI],
    "Neapolitan Major", "", "", "", "", "", ""
);

def_scale!(melodic_minor, "Melodic Minor",
    vec![WHOLE, SEMI, WHOLE, WHOLE, WHOLE, WHOLE, SEMI],
    "Melodic Minor", "", "Lydian Augmented", "Lydian Dominant", "Melodic Major", "", "Altered Scale"
);

def_scale!(enigmatic_major, "Enigmatic Major",
    vec![SEMI, MIN3, WHOLE, WHOLE, WHOLE, SEMI, SEMI], "Enigmatic Major",
    "Enigmatic Major", "", "", "", "", "", ""
);

def_scale!(enigmatic_minor, "Enigmatic Minor",
    vec![SEMI, WHOLE, MIN3, SEMI, MIN3, SEMI, SEMI],
    "Enigmatic Minor", "", "", "", "", "", ""
);

/// A struct that has methods used for naming scales that are seven (7) notes in length.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeptatonicScaleNamer{
    basis: Vec<(Scale, String)>,
}

impl HeptatonicScaleNamer{
    /// Generate a new `HeptatonicScaleName`.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// use music_theory::libr::scales::*;
    /// let namer = HeptatonicScaleNamer::new();
    /// assert_eq!(namer.name(&ionian::steps()), Some(String::from("Ionian")));
    /// ```
    pub fn new() -> Self{
        let scales = get_all_scale_objs();
        let mut basis = Vec::new();
        for sc in scales{
            for (i, mode) in sc.steps.clone().mode_iter().enumerate(){
                let mode_name = sc.get_mode_name(i);
                if mode_name.is_empty() {
                    continue;
                }
                if let Some(scale) = mode.to_scale_try(Note::new(0)){
                    basis.push((scale, mode_name));
                }
            }
        }
        Self{ basis }
    }

    /// Given the input steps, try to return a good name for this scale/mode.
    ///
    /// Example:
    /// ```
    /// use music_theory::theory::*;
    /// use music_theory::libr::scales::*;
    /// let namer = HeptatonicScaleNamer::new();
    /// assert_eq!(
    ///     namer.name(
    ///         &vec![PC::C, PC::Cs, PC::E, PC::F, PC::G, PC::A, PC::B]
    ///         .to_scale_try(Note::ZERO).unwrap()
    ///         .to_steps(true)
    ///     ),
    ///     Some(String::from("Ionian ♭2"))
    /// );
    /// ```
    pub fn name(&self, steps: &Steps) -> Option<String>{
        let nameless = steps.as_scale_try(Note(0))?;
        if nameless.len() != 7 { return None; }
        let mut dif_positions = 8;
        let mut dif_units = 9999;
        let ionian = ionian::obj().steps.to_scale_try(Note(0))?;
        let mut base_scale = ionian.clone();
        let mut base_name = "Ionian".to_string();
        for (scale, name) in &self.basis{
            let mut dp = 0;
            let mut du = 0;
            for i in 0..7{
                let d = nameless[i] - scale[i];
                if d == Interval::ROOT { continue; }
                dp += 1;
                du += d.abs().unwrap();
            }
            if dp > dif_positions { continue; }
            if du >= dif_units { continue; }
            dif_positions = dp;
            dif_units = du;
            base_scale = scale.clone();
            base_name = name.to_string();
        }
        for i in 0..7{
            let d = nameless[i] - base_scale[i];
            if d == Interval::ROOT { continue; }
            let d = nameless[i] - ionian[i];
            let _ = write!(base_name, " {}{}", d, i + 1);
        }
        Some(base_name)
    }
}

impl Default for HeptatonicScaleNamer{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use super::super::super::theory::*;

    #[test]
    fn clone_steps(){
        assert_eq!(
            ScaleObj{
                steps: Steps(vec![Interval(123456)]),
                fam_name: String::from("test"),
                modes: vec![]
            }.clone_steps(),
            Steps(vec![Interval(123456)])
        );
    }

    #[test]
    fn family_name(){
        assert_eq!(
            ScaleObj{
                steps: Steps(vec![Interval(123456)]),
                fam_name: String::from("test"),
                modes: vec![]
            }.family_name(),
            String::from("test")
        );
    }

    #[test]
    fn get_mode_name(){
        let so = ScaleObj{
            steps: Steps(vec![Interval(0), Interval(1), Interval(2)]),
            fam_name: String::from("test"),
            modes: vec![String::from("Uhh"), String::from(""), String::from("Ahh")]
        };
        assert_eq!(&so.get_mode_name(0), "Uhh");
        assert_eq!(&so.get_mode_name(1), "");
        assert_eq!(&so.get_mode_name(2), "Ahh");
        assert_eq!(&so.get_mode_name(3), "Uhh");
    }

    #[test]
    fn get_modes(){
        let so = ScaleObj{
            steps: Steps(vec![Interval(0), Interval(1), Interval(2)]),
            fam_name: String::from("test"),
            modes: vec![String::from("Uhh"), String::from(""), String::from("Ahh")]
        };
        let mut iter = so.get_modes().into_iter();
        assert_eq!(
            iter.next(),
            Some(ModeObj{
                steps: Steps(vec![Interval(0), Interval(1), Interval(2)]),
                fam_name: String::from("test"),
                mode_name: String::from("Uhh"),
                mode_nr: 0,
            })
        );
        assert_eq!(
            iter.next(),
            Some(ModeObj{
                steps: Steps(vec![Interval(1), Interval(2), Interval(0)]),
                fam_name: String::from("test"),
                mode_name: String::from(""),
                mode_nr: 1,
            })
        );
        assert_eq!(
            iter.next(),
            Some(ModeObj{
                steps: Steps(vec![Interval(2), Interval(0), Interval(1)]),
                fam_name: String::from("test"),
                mode_name: String::from("Ahh"),
                mode_nr: 2,
            })
        );
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn mode_obj_to_string(){
        assert_eq!(
            &ModeObj{
                steps: Steps(vec![Interval(2), Interval(0), Interval(1)]),
                fam_name: String::from("test"),
                mode_name: String::from("Ahh"),
                mode_nr: 2,
            }.to_string(),
            "Ahh, 3ᵉ mode of test"
        );
    }

    #[test]
    fn test_get_all_scale_objs(){
        let objs = get_all_scale_objs();
        assert_eq!(objs.len(), 10);
    }

    #[test]
    fn steps(){
        assert_eq!(
            ionian::steps(),
            Steps(vec![WHOLE, WHOLE, SEMI, WHOLE, WHOLE, WHOLE, SEMI]),
        );
    }

    #[test]
    fn obj(){
        assert_eq!(&ionian::obj().modes[5], "Aeolian");
        assert_eq!(&neapolitan_minor::obj().modes[2], "Mixolydian Augmented");
    }

    #[test]
    fn heptatonic_scale_namer_new(){
        let namer = HeptatonicScaleNamer::new();
        assert_eq!(namer.basis.len(), 30);
        assert_eq!(namer.basis[0].1, String::from("Ionian"));
    }

    #[test]
    fn heptatonic_scale_namer_name(){
        let namer = HeptatonicScaleNamer::new();
        assert_eq!(namer.name(&ionian::steps()), Some(String::from("Ionian")));
        let objs = get_all_scale_objs();
        for obj in objs{
            let mut steps = obj.steps;
            for mode in obj.modes.clone(){
                if !mode.is_empty(){
                    assert_eq!(namer.name(&steps), Some(mode));
                }
                steps.next_mode_mut();
            }
        }
        assert_eq!(namer.name(
            &vec![PC::C, PC::Cs, PC::E, PC::F, PC::G, PC::A, PC::B]
                .to_scale_try(Note(0)).unwrap()
                .to_steps(true)
        ), Some(String::from("Ionian ♭2")));
    }

    #[test]
    fn heptotonic_scale_namer_default(){
        assert_eq!(HeptatonicScaleNamer::new(), HeptatonicScaleNamer::default());
    }
}
