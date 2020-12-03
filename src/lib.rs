pub mod constants;
pub mod mathh;
pub mod theory;
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

pub fn notes_analysis(string: String){
    let ucns = string.into_ucns();
    let scale = ucns.clone().into_scale(0);
    let root = scale.0[0];
    let steps = ucns.clone().into_steps();
    let mut included = HashSet::new();
    println!("Your notes: {:?}", ucns);
    println!("----------------------------------------");
    println!("\tSubchords:");
    RootedChord::from_scale(scale.clone()).into_sub_chords()
        .into_iter().map(|c| (c.as_string(true, ChordStyling::Extended),c))
        .filter(|(s,_)| !s.contains('[') && !s.contains('(') && !s.is_empty())
        .map(|(mut s,c)| { s.push_str(&format!(": {:?}", c.to_scale().into_ucns())); s })
        .for_each(|s| { println!("{}", s); });
    println!("----------------------------------------");
    println!("\tStrict chordscales;");
    for modeobj in find_chordscales(steps){
        if included.contains(&modeobj.steps) { continue; }
        included.insert(modeobj.steps.clone());
        println!("{} {}", ucns[0], modeobj);
        print!("\t");
        print_splitted(&strs_scale_chords(&modeobj.steps, root, 3, ChordStyling::Extended), ", ", "\n");
        print!("\t");
        print_splitted(&strs_scale_chords(&modeobj.steps, root, 4, ChordStyling::Extended), ", ", "\n");
        // let subchords = scale_sub_chords(modeobj.steps.clone().into_scale(root))
        //     .into_iter().map(|c| (c.as_string(true, ChordStyling::Extended),c))
        //     .filter(|(s,_)| !s.contains('[') && !s.contains('(') && !s.is_empty())
        //     .map(|(mut s,c)| { s.push_str(&format!(": {:?}", c.to_scale().into_ucns())); s })
        //     .collect::<Vec<_>>();
        // print_to_grid_auto(&subchords, 80, 3);
    }
    println!("\tSupersequences:");
    for (tonic,modeobj) in find_scale_superseq(&scale){
        if included.contains(&modeobj.steps) { continue; }
        included.insert(modeobj.steps.clone());
        println!("{} {}", tonic, modeobj);
        let tonic = tonic.to_note(0);
        print!("\t");
        print_splitted(&strs_scale_chords(&modeobj.steps, tonic, 3, ChordStyling::Extended), ", ", "\n");
        print!("\t");
        print_splitted(&strs_scale_chords(&modeobj.steps, tonic, 4, ChordStyling::Extended), ", ", "\n");
    }
    println!("\tSupersets:");
    for (tonic,modeobj) in find_scale_superset(ucns, false){
        if included.contains(&modeobj.steps) { continue; }
        included.insert(modeobj.steps.clone());
        println!("{} {}", tonic, modeobj);
        let tonic = tonic.to_note(0);
        print!("\t");
        print_splitted(&strs_scale_chords(&modeobj.steps, tonic, 3, ChordStyling::Extended), ", ", "\n");
        print!("\t");
        print_splitted(&strs_scale_chords(&modeobj.steps, tonic, 4, ChordStyling::Extended), ", ", "\n");
    }
    println!("----------------------------------------");
}

