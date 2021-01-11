pub mod constants;
pub mod mathh;
pub mod theory;
#[macro_use]
pub mod utils;
pub mod libr;
pub mod query;

use std::collections::HashSet;
use theory::*;
use libr::infos::*;
use query::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn print_step_chords(steps: &Steps, root: Note, styling: ChordStyling){
    print!("\t");
    print_splitted(&strs_scale_chords(steps, root, 3, styling), ", ", "\n");
    print!("\t");
    print_splitted(&strs_scale_chords(steps, root, 4, styling), ", ", "\n");
}

pub fn notes_analysis(string: String, styling: ChordStyling){
    let ucns = string.into_ucns();
    let scale = ucns.clone().into_scale(0);
    let root = scale.0[0];
    let steps = ucns.clone().into_steps();
    let ctonic = ucns[0];
    let mut included = HashSet::new();
    println!("Your notes: {:?}", ucns);
    println!("----------------------------------------");
    println!("\tSubchords:");
    let rchord = RootedChord::from_scale(scale.clone());
    rchord
        .clone().into_sub_chords()
        .into_iter().map(|c| (c.as_string(true, ChordStyling::Extended),c))
        .filter(|(s,_)| !s.contains('[') /* && !s.contains('(') */ && !s.is_empty())
        .map(|(mut s,c)| { s.push_str(&format!(": {:?}", c.to_scale().into_ucns())); s })
        .for_each(|s| { println!("{}", s); });
    println!("----------------------------------------");
    println!("\tChordtone Wholetone Scale:");
    let ctwts = rchord.to_chordtone_wholetone_scale();
    let mo = find_scale(&ctwts);
    if let Some(m) = mo{
        included.insert((ctonic, m.steps.clone()));
        println!("{} {}", ctonic, m);
    } else {
        println!("{} unnamed", ctonic);
    }
    if !ctwts.is_empty() {
        let ctwts = ctwts.into_steps();
        print_step_chords(&ctwts, root, styling);
    }
    println!("\tStrict chordscales:");
    for modeobj in find_chordscales(steps){
        if included.contains(&(ctonic, modeobj.steps.clone())) { continue; }
        included.insert((ctonic, modeobj.steps.clone()));
        println!("{} {}", ctonic, modeobj);
        print_step_chords(&modeobj.steps, root, styling);
        // let subchords = scale_sub_chords(modeobj.steps.clone().into_scale(root))
        //     .into_iter().map(|c| (c.as_string(true, ChordStyling::Extended),c))
        //     .filter(|(s,_)| !s.contains('[') && !s.contains('(') && !s.is_empty())
        //     .map(|(mut s,c)| { s.push_str(&format!(": {:?}", c.to_scale().into_ucns())); s })
        //     .collect::<Vec<_>>();
        // print_to_grid_auto(&subchords, 80, 3);
    }
    println!("\tSupersequences:");
    for (tonic,modeobj) in find_scale_superseq(&scale){
        if included.contains(&(tonic, modeobj.steps.clone())) { continue; }
        included.insert((tonic, modeobj.steps.clone()));
        println!("{} {}", tonic, modeobj);
        let tonic = tonic.to_note(0);
        print_step_chords(&modeobj.steps, tonic, styling);
    }
    println!("\tSupersets:");
    for (tonic,modeobj) in find_scale_superset(ucns, false){
        if included.contains(&(tonic, modeobj.steps.clone())) { continue; }
        included.insert((tonic, modeobj.steps.clone()));
        println!("{} {}", tonic, modeobj);
        let tonic = tonic.to_note(0);
        print_step_chords(&modeobj.steps, tonic, styling);
    }
    println!("----------------------------------------");
}

