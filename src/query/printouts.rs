use super::find_scale_chords;
use crate::theory::{ Steps, ChordStyle, AsRelativeIntervals, ToIonianRelativeStringTry };
use crate::utils::to_roman_num;
use crate::libr::{ ionian, HeptatonicScaleNamer, get_all_scale_objs, Intercalatable };

use std::fmt::Write;

pub fn scale_chords_roman_printout(steps: &Steps, size: usize, style: ChordStyle) -> Vec<String>{
    let chords = find_scale_chords(steps, size);
    let mut res = Vec::new();
    for (i, chord) in chords.iter().enumerate(){
        res.push(chord.quality(to_roman_num(i + 1), style));
    }
    res
}

pub fn scale_chords_root_printout(steps: &Steps, root: String, size: usize, style: ChordStyle)
    -> Vec<String>
{
    let chords = find_scale_chords(steps, size);
    let mut res = Vec::new();
    for chord in chords{
        res.push(chord.quality(root.clone(), style));
    }
    res
}

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
        let chords = scale_chords_roman_printout(&crate::libr::ionian::steps(), 3, style);
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
