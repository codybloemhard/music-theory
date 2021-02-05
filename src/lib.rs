pub mod theory;
#[macro_use]
pub mod utils;
pub mod libr;
pub mod query;

use std::collections::{ HashSet, HashMap };
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
    // Remove duplicate notes
    let (ens, pcs, pcs_to_ens) = {
        let ens = input_string.into_enharmonic_notes();
        let mut hs = HashSet::new();
        let mut map = HashMap::new();
        let mut pcs = Vec::new();
        let mut new_ens = Vec::new();
        for en in ens{
            let pc = en.to_pc();
            if !hs.contains(&pc){
                hs.insert(pc);
                pcs.push(pc);
                new_ens.push(en);
                map.insert(pc, en);
            }
        }
        (new_ens, pcs, map)
    };

    let spell_out = |scale: Scale| {
        if scale.is_empty() { return String::new(); }
        let tonic = scale.0[0];
        let start = if let Some(x) = pcs_to_ens.get(&tonic.to_pc()){
            Some(*x)
        } else {
            None
        };
        scale.into_enharmonic_notes_with_start(start).into_iter().map(|e| e.to_string_name()).collect::<Vec<String>>().intercalate(", ".to_string())
    };

    let map_pc_to_en = |pc: PC| {
        if let Some(x) = pcs_to_ens.get(&pc){
            *x
        } else {
            pc.to_note(0).to_enharmonic_note().expect("should not be able to return None")
        }
    };

    let mut res = Vec::new();
    if pcs.is_empty() { return res; }
    let scale = pcs.clone().into_scale(0);
    let root = scale.0[0];
    let ctonic = pcs[0];
    let rchord = RootedChord::from_scale(scale.clone());
    let mut included = HashSet::new();
    let ens_string = ens.into_iter().map(|en| { let mut string = en.to_string_name(); string.push_str(", "); string }).collect::<String>();
    let mut string = format!("Your input: {}\nYour pitchclasses: {:?}\n", ens_string, pcs);
    res.push(("Input".to_string(), mem::replace(&mut string, String::new())));
    let inversions = {
        let mut inversions = rchord.all_inversions();
        inversions.pop();
        inversions
    };
    inversions
        .into_iter().map(|c| (c.as_string(true, styling),c))
        .filter(|(s,_)| !s.contains('[') && !s.is_empty())
        .map(|(mut s,c)| { s.push_str(&format!(": {:?}", c.to_scale().into_pcs())); s })
        .for_each(|s| { string.push_str(&format!("{}\n", s)); });
    res.push(("Inversions".to_string(), mem::replace(&mut string, String::new())));
    rchord
        .clone().into_subseq_chords()
        .into_iter().map(|c| (c.as_string(true, styling),c))
        .filter(|(s,_)| !s.contains('[') /* && !s.contains('(') */ && !s.is_empty())
        .map(|(mut s,c)| { s.push_str(&format!(": {:?}", c.to_scale().into_pcs())); s })
        .for_each(|s| { string.push_str(&format!("{}\n", s)); });
    res.push(("SubChords".to_string(), mem::replace(&mut string, String::new())));
    // let namer = HeptatonicScaleNamer::new(); TODO
    let ctwts = rchord.to_chordtone_wholetone_scale();
    let mo = find_scale(&ctwts);
    if let Some(m) = mo{
        included.insert((ctonic, m.steps.clone()));
        let spelled_out = spell_out(m.steps.to_scale(root));
        string.push_str(&format!("{} {}: {}\n", map_pc_to_en(ctonic), m, spelled_out));
    }
    // if !ctwts.is_empty() {
    //     let ctwts = ctwts.into_steps();
    //     string.push_str(&print_step_chords(&ctwts, root, styling));
    // }
    res.push(("Chordtone Wholetone Scale".to_string(), mem::replace(&mut string, String::new())));
    for modeobj in find_chordscales(&pcs){
        if included.contains(&(ctonic, modeobj.steps.clone())) { continue; }
        included.insert((ctonic, modeobj.steps.clone()));
        let spelled_out = spell_out(modeobj.steps.to_scale(root));
        string.push_str(&format!("{} {}: {}\n", map_pc_to_en(ctonic), modeobj, spelled_out));
        // string.push_str(&print_step_chords(&modeobj.steps, root, styling));
    }
    res.push(("Strict Chordscales".to_string(), mem::replace(&mut string, String::new())));
    for (tonic,modeobj) in find_scale_superseq(&scale){
        if included.contains(&(tonic, modeobj.steps.clone())) { continue; }
        included.insert((tonic, modeobj.steps.clone()));
        let spelled_out = spell_out(modeobj.steps.to_scale(tonic.to_note(0)));
        string.push_str(&format!("{} {}: {}\n", map_pc_to_en(tonic), modeobj, spelled_out));
        // let tonic = tonic.to_note(0);
        // string.push_str(&print_step_chords(&modeobj.steps, tonic, styling));
    }
    res.push(("Supersequences".to_string(), mem::replace(&mut string, String::new())));
    for (tonic,modeobj) in find_scale_superset(pcs, false){
        if included.contains(&(tonic, modeobj.steps.clone())) { continue; }
        included.insert((tonic, modeobj.steps.clone()));
        let spelled_out = spell_out(modeobj.steps.to_scale(tonic.to_note(0)));
        string.push_str(&format!("{} {}: {}\n", map_pc_to_en(tonic), modeobj, spelled_out));
        // let tonic = tonic.to_note(0);
        // string.push_str(&print_step_chords(&modeobj.steps, tonic, styling));
    }
    res.push(("Supersets".to_string(), mem::replace(&mut string, String::new())));
    res
}

