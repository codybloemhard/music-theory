pub mod constants;
pub mod mathh;
pub mod theory;
#[macro_use]
pub mod utils;
pub mod libr;
pub mod query;

use std::collections::HashSet;
use std::mem;
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

// return (header,content)
pub fn notes_analysis(input_string: String, styling: ChordStyling) -> Vec<(String, String)>{
    let ucns = {
        let ucns = input_string.into_ucns();
        let mut hm = HashSet::new();
        let mut res = Vec::new();
        for ucn in ucns{
            if !hm.contains(&ucn){
                hm.insert(ucn);
                res.push(ucn);
            }
        }
        res
    };
    let mut res = Vec::new();
    if ucns.is_empty() { return res; }
    let mut string = String::new();
    let scale = ucns.clone().into_scale(0);
    let root = scale.0[0];
    let steps = ucns.clone().into_steps();
    let ctonic = ucns[0];
    let rchord = RootedChord::from_scale(scale.clone());
    let mut included = HashSet::new();
    res.push(("Input".to_string(), format!("Your notes: {:?}\n", ucns)));
    let inversions = {
        let mut inversions = rchord.all_inversions();
        inversions.pop();
        inversions
    };
    inversions
        .into_iter().map(|c| (c.as_string(true, styling),c))
        .filter(|(s,_)| !s.contains('[') && !s.is_empty())
        .map(|(mut s,c)| { s.push_str(&format!(": {:?}", c.to_scale().into_ucns())); s })
        .for_each(|s| { string.push_str(&format!("{}\n", s)); });
    res.push(("Inversions".to_string(), mem::replace(&mut string, String::new())));
    rchord
        .clone().into_subseq_chords()
        .into_iter().map(|c| (c.as_string(true, styling),c))
        .filter(|(s,_)| !s.contains('[') /* && !s.contains('(') */ && !s.is_empty())
        .map(|(mut s,c)| { s.push_str(&format!(": {:?}", c.to_scale().into_ucns())); s })
        .for_each(|s| { string.push_str(&format!("{}\n", s)); });
    res.push(("SubChords".to_string(), mem::replace(&mut string, String::new())));
    let ctwts = rchord.to_chordtone_wholetone_scale();
    let mo = find_scale(&ctwts);
    if let Some(m) = mo{
        included.insert((ctonic, m.steps.clone()));
        let spelled_out = m.steps.to_scale(root);
        string.push_str(&format!("{} {}: {:?}\n", ctonic, m, spelled_out.0));
    }
    // if !ctwts.is_empty() {
    //     let ctwts = ctwts.into_steps();
    //     string.push_str(&print_step_chords(&ctwts, root, styling));
    // }
    res.push(("Chordtone Wholetone Scale".to_string(), mem::replace(&mut string, String::new())));
    for modeobj in find_chordscales(steps){
        if included.contains(&(ctonic, modeobj.steps.clone())) { continue; }
        included.insert((ctonic, modeobj.steps.clone()));
        string.push_str(&format!("{} {}\n", ctonic, modeobj));
        // string.push_str(&print_step_chords(&modeobj.steps, root, styling));
    }
    res.push(("Strict Chordscale".to_string(), mem::replace(&mut string, String::new())));
    for (tonic,modeobj) in find_scale_superseq(&scale){
        if included.contains(&(tonic, modeobj.steps.clone())) { continue; }
        included.insert((tonic, modeobj.steps.clone()));
        string.push_str(&format!("{} {}\n", tonic, modeobj));
        // let tonic = tonic.to_note(0);
        // string.push_str(&print_step_chords(&modeobj.steps, tonic, styling));
    }
    res.push(("Supersequences".to_string(), mem::replace(&mut string, String::new())));
    for (tonic,modeobj) in find_scale_superset(ucns, false){
        if included.contains(&(tonic, modeobj.steps.clone())) { continue; }
        included.insert((tonic, modeobj.steps.clone()));
        string.push_str(&format!("{} {}\n", tonic, modeobj));
        // let tonic = tonic.to_note(0);
        // string.push_str(&print_step_chords(&modeobj.steps, tonic, styling));
    }
    res.push(("Supersets".to_string(), mem::replace(&mut string, String::new())));
    res
}

