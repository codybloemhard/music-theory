pub mod theory;
#[macro_use]
pub mod utils;
pub mod libr;
pub mod query;

// use theory::*;
// use libr::*;
// use query::*;
//
// use std::collections::{ HashSet, HashMap };
// use std::mem;
//
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
//
// pub fn step_chords_string(steps: &Steps, root: Note, styling: ChordStyling) -> String{
//     let mut string = String::new();
//     let triads = format_splitted(&strs_scale_chords(steps, root, 3, styling), ", ", "\n");
//     string.push_str(&triads);
//     let tetrads = format_splitted(&strs_scale_chords(steps, root, 4, styling), ", ", "\n");
//     string.push_str(&tetrads);
//     string
// }
//
// // return (header,content)
// pub fn notes_analysis(input_string: String, styling: ChordStyling) -> Vec<(String, String)>{
//     // Remove duplicate notes
//     let (ens, pcs, pcs_to_ens) = {
//         let ens = input_string.into_enharmonic_notes();
//         let mut hs = HashSet::new();
//         let mut map = HashMap::new();
//         let mut pcs = Vec::new();
//         let mut new_ens = Vec::new();
//         for en in ens{
//             let pc = en.to_pc();
//             if !hs.contains(&pc){
//                 hs.insert(pc);
//                 pcs.push(pc);
//                 new_ens.push(en);
//                 map.insert(pc, en);
//             }
//         }
//         (new_ens, pcs, map)
//     };
//
//     let spell_out = |scale: Scale| {
//         if scale.is_empty() { return String::new(); }
//         let tonic = scale.0[0];
//         let start = pcs_to_ens.get(&tonic.to_pc()).copied();
//         scale.into_enharmonic_notes_with_start(start).into_iter().map(|e| e.to_string_name()).collect::<Vec<String>>().intercalate(", ".to_string())
//     };
//
//     let map_pc_to_en = |pc: PC| {
//         if let Some(x) = pcs_to_ens.get(&pc){
//             *x
//         } else {
//             pc.to_note().to_enharmonic_note().expect("should not be able to return None")
//         }
//     };
//
//     let namer = HeptatonicScaleNamer::new();
//     let mode_format = |en: EnharmonicNote, mo: ModeObj, spelled_out: String|{
//         let temp;
//         let mode_name = if mo.mode_name == *""{
//             temp = namer.name(&mo.steps);
//             &temp
//         } else {
//             &mo.mode_name
//         };
//         format!("{} {}, {}ᵉ mode of {}: {}\n", en, mode_name, mo.mode_nr + 1, mo.fam_name, spelled_out)
//     };
//
//     let mut res = Vec::new();
//     if pcs.is_empty() { return res; }
//     let scale = pcs.clone().into_scale(0);
//     let steps = scale.to_steps();
//     let root = scale.0[0];
//     let ctonic = pcs[0];
//     let rchord = RootedChord::from_scale(scale.clone());
//     let mut included = HashSet::new();
//     let ens_string = ens.into_iter().map(|en| { let mut string = en.to_string_name(); string.push_str(", "); string }).collect::<String>();
//     let mut string = format!("Your input: {}\n", ens_string);
//     string.push_str(&format!("Numbered pitchclasses: {:?}\n", pcs.iter().map(|pc| pc.0).collect::<Vec<_>>()));
//     string.push_str(&format!("Named pitchclasses: {:?}\n", pcs));
//     res.push(("Input".to_string(), mem::take(&mut string)));
//
//     if scale.len() == 7{ // we have an heptatonic scale on our hands
//         let mo = find_scale(&scale);
//         string = if let Some(mo) = mo{
//             let temp;
//             let mode_name = if mo.mode_name == *""{
//                 temp = namer.name(&mo.steps);
//                 &temp
//             } else {
//                 &mo.mode_name
//             };
//             format!("{}, {}ᵉ mode of {}\n", mode_name, mo.mode_nr + 1, mo.fam_name)
//         } else {
//             format!("{}\n", namer.name(&steps))
//         };
//         string.push_str(&steps.to_relative(&ionian::steps()).unwrap().string_ionian_rel());
//         string.push('\n');
//         string.push_str(&step_chords_string(&steps, root, styling));
//         string.push('\n');
//         res.push(("Heptatonic Scale".to_string(), mem::take(&mut string)));
//     }
//
//     let inversions = {
//         let mut inversions = rchord.all_inversions();
//         inversions.pop();
//         inversions
//     };
//     inversions
//         .into_iter().map(|c| (c.as_string(true, styling),c))
//         .filter(|(s,_)| !s.contains('[') && !s.is_empty())
//         .map(|(mut s,c)| { s.push_str(&format!(": {:?}", c.to_scale().into_pcs())); s })
//         .for_each(|s| { string.push_str(&format!("{}\n", s)); });
//     res.push(("Inversions".to_string(), mem::take(&mut string)));
//
//     rchord
//         .clone().into_subseq_chords()
//         .into_iter().map(|c| (c.as_string(true, styling),c))
//         .filter(|(s,_)| !s.contains('[') /* && !s.contains('(') */ && !s.is_empty())
//         .map(|(mut s,c)| { s.push_str(&format!(": {:?}", c.to_scale().into_pcs())); s })
//         .for_each(|s| { string.push_str(&format!("{}\n", s)); });
//     res.push(("Sub Chords".to_string(), mem::take(&mut string)));
//
//     let ctwts = rchord.to_chordtone_wholetone_scale();
//     let mo = find_scale(&ctwts);
//     if let Some(m) = mo{
//         included.insert((ctonic, m.steps.clone()));
//         let spelled_out = spell_out(m.steps.to_scale(root));
//         string.push_str(&mode_format(map_pc_to_en(ctonic), m, spelled_out));
//     }
//     // if !ctwts.is_empty() {
//     //     let ctwts = ctwts.into_steps();
//     //     string.push_str(&print_step_chords(&ctwts, root, styling));
//     // }
//     res.push(("Chordtone Wholetone Scale".to_string(), mem::take(&mut string)));
//
//     for modeobj in find_chordscales(&pcs){
//         if included.contains(&(ctonic, modeobj.steps.clone())) { continue; }
//         included.insert((ctonic, modeobj.steps.clone()));
//         let spelled_out = spell_out(modeobj.steps.to_scale(root));
//         string.push_str(&mode_format(map_pc_to_en(ctonic), modeobj, spelled_out));
//     }
//     res.push(("Strict Chordscales".to_string(), mem::take(&mut string)));
//
//     for (tonic,modeobj) in find_scale_superstring(&scale){
//         if included.contains(&(tonic, modeobj.steps.clone())) { continue; }
//         included.insert((tonic, modeobj.steps.clone()));
//         let spelled_out = spell_out(modeobj.steps.to_scale(tonic.to_note()));
//         string.push_str(&mode_format(map_pc_to_en(tonic), modeobj, spelled_out));
//     }
//     res.push(("Super Strings".to_string(), mem::take(&mut string)));
//
//     for (tonic,modeobj) in find_scale_superset(pcs, false){
//         if included.contains(&(tonic, modeobj.steps.clone())) { continue; }
//         included.insert((tonic, modeobj.steps.clone()));
//         let spelled_out = spell_out(modeobj.steps.to_scale(tonic.to_note()));
//         string.push_str(&mode_format(map_pc_to_en(tonic), modeobj, spelled_out));
//     }
//     res.push(("Super Sets".to_string(), mem::take(&mut string)));
//     res
// }
//
