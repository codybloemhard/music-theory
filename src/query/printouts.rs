use super::{ find_scale_chords, find_rooted_scale_chords };
use crate::theory::{ Note, Steps, ChordStyle, AsRelativeIntervals, ToIonianRelativeStringTry };
use crate::utils::{ to_roman_num, Intercalatable  };
use crate::libr::{ ionian, HeptatonicScaleNamer, get_all_scale_objs };

use std::fmt::Write;

/// Outputs the chords of the scale degrees with roman numerals as base string.
///
/// Example:
/// ```
/// use music_theory::{ theory::*, query::*, libr::* };
/// let style = ChordStyle::Std(MStyle::Long, EStyle::Long);
/// let chords = scale_chords_roman_printout(&ionian::steps(), 3, style);
/// assert_eq!(&chords[0], "Imaj");
/// assert_eq!(&chords[1], "IImin");
/// ```
pub fn scale_chords_roman_printout(steps: &Steps, size: usize, style: ChordStyle) -> Vec<String>{
    let chords = find_scale_chords(steps, size);
    let mut res = Vec::new();
    for (i, chord) in chords.iter().enumerate(){
        res.push(chord.quality(to_roman_num(i + 1), style));
    }
    res
}

/// Outputs the chords of the scale degrees with the chord root as base string.
///
/// Example:
/// ```
/// use music_theory::{ theory::*, query::*, libr::* };
/// let style = ChordStyle::Std(MStyle::Long, EStyle::Long);
/// let chords = scale_chords_root_printout(&ionian::steps(), Note::C1, 3, style);
/// assert_eq!(&chords[0], "Cmaj");
/// assert_eq!(&chords[6], "Bdim");
/// ```
pub fn scale_chords_root_printout(steps: &Steps, root: Note, size: usize, style: ChordStyle)
    -> Vec<String>
{
    let chords = find_rooted_scale_chords(steps, root, size);
    let mut res = Vec::new();
    for chord in chords{
        res.push(chord.as_string(style));
    }
    res
}

/// Outputs all triads and tetrads build upon scale degrees of the scale.
///
/// Example:
/// ```
/// use music_theory::{ theory::*, query::*, libr::* };
/// let style = ChordStyle::Std(MStyle::Long, EStyle::Long);
/// let po = step_chords_string(&ionian::steps(), Note::new(3), style);
/// assert_eq!(
///     po, concat!(
///         "Cmaj, Dmin, Emin, Fmaj, Gmaj, Amin, Bdim\n",
///         "Cmaj7, Dmin7, Emin7, Fmaj7, G7, Amin7, Bø\n"
///     )
/// );
/// ```
pub fn step_chords_string(steps: &Steps, root: Note, style: ChordStyle) -> String{
    let mut string = String::new();
    let triads = scale_chords_root_printout(steps, root, 3, style)
        .intercalate_with_end(", ".to_string(), "\n".to_string());
    string.push_str(&triads);
    let tetrads = scale_chords_root_printout(steps, root, 4, style)
        .intercalate_with_end(", ".to_string(), "\n".to_string());
    string.push_str(&tetrads);
    string
}

/// Outputs a list of scales and modes family name, name, mode number, Ionian relative string and
/// scale degree triads and tetrads.
///
/// Example:
/// ```
/// use music_theory::{ theory::*, query::* };
/// let style = ChordStyle::Std(MStyle::Long, EStyle::Long);
/// let snc = scales_and_chords_printout(style);
/// let lines = snc.lines().collect::<Vec<_>>();
/// assert_eq!(lines[3], "\tImaj, IImin, IIImin, IVmaj, Vmaj, VImin, VIIdim");
/// ```
/// Part of the output:
/// ```plain
/// Ionian
/// 0: Ionian
///     1 2 3 4 5 6 7
///     IΔ, II-, III-, IVΔ, VΔ, VI-, VII°
///     IΔ7, II-7, III-7, IVΔ7, V7, VI-7, VIIø
/// 1: Dorian
///     1 2 ♭3 4 5 6 ♭7
///     I-, II-, IIIΔ, IVΔ, V-, VI°, VIIΔ
///     I-7, II-7, IIIΔ7, IV7, V-7, VIø, VIIΔ7
/// 2: Phrygian
/// ...
/// ```
pub fn scales_and_chords_printout(style: ChordStyle) -> String{
    let namer = HeptatonicScaleNamer::new();
    let objs = get_all_scale_objs();
    let empty = String::from("");
    let mut res = String::new();
    for sobj in objs{
        let _ = writeln!(res, "{}", sobj.family_name());
        for mode in sobj.get_modes(){
            let mode_name = if mode.mode_name == empty{
                if let Some(n) = namer.name(&mode.steps) { n } else { continue; }
            } else {
                mode.mode_name
            };
            let relstring = if let Some(ints) = mode.steps.as_relative_intervals(&ionian::steps()){
                if let Some(rel) = ints.to_ionian_relative_string_try(true){rel} else { continue; }
            } else { continue; };
            let _ = writeln!(res, "{}: {}", mode.mode_nr, mode_name);
            let _ = writeln!(res, "\t{}", relstring);
            let c3 = scale_chords_roman_printout(&mode.steps, 3, style);
            let c4 = scale_chords_roman_printout(&mode.steps, 4, style);
            let _ = write!(res, "\t");
            let _ = write!(res, "{}", c3.intercalate_with_end(", ".to_string(), "\n".to_string()));
            let _ = write!(res, "\t");
            let _ = write!(res, "{}", c4.intercalate_with_end(", ".to_string(), "\n".to_string()));
        }
    }
    res
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::theory::*;

    #[test]
    fn test_scale_chords_roman_printout(){
        let style = ChordStyle::Std(MStyle::Long, EStyle::Long);
        let chords = scale_chords_roman_printout(&ionian::steps(), 3, style);
        assert_eq!(
            chords,
            vec![
                String::from("Imaj"),
                String::from("IImin"),
                String::from("IIImin"),
                String::from("IVmaj"),
                String::from("Vmaj"),
                String::from("VImin"),
                String::from("VIIdim"),
            ]
        );
    }

    #[test]
    fn test_scale_chords_root_printout(){
        let style = ChordStyle::Std(MStyle::Long, EStyle::Long);
        let chords = scale_chords_root_printout(&ionian::steps(), Note::C1, 3, style);
        assert_eq!(
            chords,
            vec![
                String::from("Cmaj"),
                String::from("Dmin"),
                String::from("Emin"),
                String::from("Fmaj"),
                String::from("Gmaj"),
                String::from("Amin"),
                String::from("Bdim"),
            ]
        );
    }

    #[test]
    fn test_step_chords_string(){
        let style = ChordStyle::Std(MStyle::Long, EStyle::Long);
        let po = step_chords_string(&ionian::steps(), Note(3), style);
        assert_eq!(
            po, concat!(
                "Cmaj, Dmin, Emin, Fmaj, Gmaj, Amin, Bdim\n",
                "Cmaj7, Dmin7, Emin7, Fmaj7, G7, Amin7, Bø\n"
            )
        );
    }

    #[test]
    fn test_scales_and_chords_printout(){
        let style = ChordStyle::Std(MStyle::Long, EStyle::Long);
        let snc = scales_and_chords_printout(style);
        let lines = snc.lines().collect::<Vec<_>>();
        assert_eq!(lines[3], "\tImaj, IImin, IIImin, IVmaj, Vmaj, VImin, VIIdim");
        assert_eq!(lines[36], "\tIdim, IIaug, IIImin, IVmaj, Vmaj, VIdim, VIImin");
        assert_eq!(lines[58], "Harmonic Major");
        assert_eq!(lines[287], "\t1 ♭2 ♭♭3 ♭4 5 ♭6 7");
    }
}
