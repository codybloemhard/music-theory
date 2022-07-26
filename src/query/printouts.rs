use super::find_scale_chords;
use crate::theory::{ Steps, ChordStyle };
use crate::utils::to_roman_num;

pub fn strs_scale_chords_roman(steps: &Steps, size: usize, style: ChordStyle) -> Vec<String>{
    let chords = find_scale_chords(steps, size);
    let mut res = Vec::new();
    for (i, chord) in chords.iter().enumerate(){
        res.push(chord.quality(to_roman_num(i + 1), style));
    }
    res
}

// pub fn print_scales(styling: ChordStyling){
//     let namer = HeptatonicScaleNamer::new();
//     let objs = get_all_scale_objs();
//     let empty = String::from("");
//     for sobj in objs{
//         println!("{}", sobj.family_name());
//         for mode in sobj.get_modes(){
//             let temp;
//             let mode_name = if mode.mode_name == empty{
//                 temp = namer.name(&mode.steps);
//                 &temp
//             } else {
//                 &mode.mode_name
//             };
//             println!("{}: {}", mode.mode_nr, mode_name);
//             println!("\t{}", mode.steps.to_relative(&ionian::steps()).unwrap().string_ionian_rel());
//             let c3 = strs_scale_chords_roman(&mode.steps, 3, styling);
//             let c4 = strs_scale_chords_roman(&mode.steps, 4, styling);
//             print!("\t");
//             print_splitted(&c3, ", ", "\n");
//             print!("\t");
//             print_splitted(&c4, ", ", "\n");
//         }
//     }
// }

#[cfg(test)]
mod tests{
    use super::*;
    use crate::theory::*;

    #[test]
    fn test_strs_scale_chords_roman(){
        let style = ChordStyle::Std(MStyle::Long, EStyle::Long);
        let chords = strs_scale_chords_roman(&crate::libr::ionian::steps(), 3, style);
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
}
