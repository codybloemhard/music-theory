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

pub fn print_step_chords(steps: &Steps, root: Note, styling: ChordStyling) -> String{
    let mut string = String::new();
    string.push('\t');
    let triads = format_splitted(&strs_scale_chords(steps, root, 3, styling), ", ", "\n");
    string.push_str(&triads);
    string.push('\t');
    let tetrads = format_splitted(&strs_scale_chords(steps, root, 4, styling), ", ", "\n");
    string.push_str(&tetrads);
    string
}

pub fn notes_analysis(input_string: String, styling: ChordStyling) -> String{
    let mut string = String::new();
    let ucns = input_string.into_ucns();
    let scale = ucns.clone().into_scale(0);
    let root = scale.0[0];
    let steps = ucns.clone().into_steps();
    let ctonic = ucns[0];
    let mut included = HashSet::new();
    string.push_str(&format!("Your notes: {:?}\n", ucns));
    string.push_str(&"----------------------------------------\n".to_string());
    string.push_str(&"\tSubchords:\n");
    let rchord = RootedChord::from_scale(scale.clone());
    rchord
        .clone().into_sub_chords()
        .into_iter().map(|c| (c.as_string(true, ChordStyling::Extended),c))
        .filter(|(s,_)| !s.contains('[') /* && !s.contains('(') */ && !s.is_empty())
        .map(|(mut s,c)| { s.push_str(&format!(": {:?}", c.to_scale().into_ucns())); s })
        .for_each(|s| { string.push_str(&format!("{}\n", s)); });
    string.push_str(&"----------------------------------------\n".to_string());
    string.push_str("\tChordtone Wholetone Scale:\n");
    let ctwts = rchord.to_chordtone_wholetone_scale();
    let mo = find_scale(&ctwts);
    if let Some(m) = mo{
        included.insert((ctonic, m.steps.clone()));
        string.push_str(&format!("{} {}\n", ctonic, m));
    } else {
        string.push_str(&format!("{} unnamed\n", ctonic));
    }
    if !ctwts.is_empty() {
        let ctwts = ctwts.into_steps();
        print_step_chords(&ctwts, root, styling);
    }
    string.push_str(&"\tStrict chordscales:\n");
    for modeobj in find_chordscales(steps){
        if included.contains(&(ctonic, modeobj.steps.clone())) { continue; }
        included.insert((ctonic, modeobj.steps.clone()));
        string.push_str(&format!("{} {}\n", ctonic, modeobj));
        print_step_chords(&modeobj.steps, root, styling);
    }
    string.push_str("\tSupersequences:\n");
    for (tonic,modeobj) in find_scale_superseq(&scale){
        if included.contains(&(tonic, modeobj.steps.clone())) { continue; }
        included.insert((tonic, modeobj.steps.clone()));
        string.push_str(&format!("{} {}\n", tonic, modeobj));
        let tonic = tonic.to_note(0);
        print_step_chords(&modeobj.steps, tonic, styling);
    }
    string.push_str("\tSupersets:\n");
    for (tonic,modeobj) in find_scale_superset(ucns, false){
        if included.contains(&(tonic, modeobj.steps.clone())) { continue; }
        included.insert((tonic, modeobj.steps.clone()));
        string.push_str(&format!("{} {}\n", tonic, modeobj));
        let tonic = tonic.to_note(0);
        print_step_chords(&modeobj.steps, tonic, styling);
    }
    string.push_str(&"----------------------------------------\n".to_string());
    string
}

