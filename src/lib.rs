//! Rust library for music theory objects and queries.
//! The main types are:
//! - [Note][crate::theory::note::Note]
//! - [PC][crate::theory::pc::PC]
//! - [Interval][crate::theory::interval::Interval]
//! - [NamedInterval][crate::theory::interval::NamedInterval]
//! - [NamedOctaveInterval][crate::theory::interval::NamedOctaveInterval]
//! - [Letter][crate::theory::enharmonic_note::Letter]
//! - [EnharmonicNote][crate::theory::enharmonic_note::EnharmonicNote]
//! - [Scale][crate::theory::scale::Scale]
//! - [Steps][crate::theory::scale::Steps]
//! - [Chord][crate::theory::chord::Chord]
//! - [RootedChord][crate::theory::chord::RootedChord]
//! - [RelativeChord][crate::theory::chord::RelativeChord]

/// Music theory core functionality.
pub mod theory;
#[macro_use]
pub mod utils;
pub mod libr;
/// Queries such as searches.
pub mod query;

use theory::*;
use libr::*;
use query::*;

use std::collections::{ HashSet, HashMap };
use std::mem;
use std::fmt::Write;

use vec_string::*;

/// return (header, content)
#[cfg(not(tarpaulin))]
pub fn notes_analysis(input_string: String, style: ChordStyle) -> Vec<(String, String)>{
    // Remove duplicate notes
    let (ens, pcs, pcs_to_ens) = {
        let ens = input_string.to_enharmonic_notes();
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
        let start = pcs_to_ens.get(&tonic.to_pc()).copied();
        scale.to_enharmonic_notes_with_start(start).into_iter()
            .map(|e| e.to_string()).collect::<Vec<String>>().intercalate(", ".to_string())
    };

    let map_pc_to_en = |pc: PC| {
        if let Some(x) = pcs_to_ens.get(&pc){
            *x
        } else {
            pc.to_note().to_enharmonic_note()
        }
    };

    let namer = HeptatonicScaleNamer::new();
    let mode_format = |en: EnharmonicNote, mo: ModeObj, spelled_out: String|{
        let mode_name = if mo.mode_name == *""{
            namer.name(&mo.steps).unwrap_or_else(|| String::from("unnamed"))
        } else {
            mo.mode_name
        };
        format!("{} {}, {}ᵉ mode of {}: {}\n", en, mode_name, mo.mode_nr + 1, mo.fam_name, spelled_out)
    };

    let mut res = Vec::new();
    if pcs.is_empty() { return res; }
    let scale = pcs.clone().to_scale_try(Note::ZERO).expect("PCs to scale try with 0 should work!");
    let root = scale.0[0];
    let steps = scale.as_steps(true);
    let ctonic = pcs[0];
    let rchord = scale.as_rooted_chord();
    let mut included = HashSet::new();
    let ens_string = ens.into_iter()
        .map(|en| { let mut string = en.to_string(); string.push_str(", "); string })
        .collect::<String>();

    let mut string = format!("Your input: {}\n", ens_string);
    let _ = writeln!(string, "Numbered pitchclasses: {:?}",
        pcs.iter().map(|pc| *pc as u32).collect::<Vec<_>>());
    let _ = writeln!(string, "Named pitchclasses: {:?}", pcs.vec_string());
    res.push(("Input".to_string(), mem::take(&mut string)));

    if scale.len() == 7{ // we have an heptatonic scale on our hands
        let mo = find_scale(&scale);
        string = if let Some(mo) = mo{
            let temp;
            let mode_name = if mo.mode_name == *""{
                temp = namer.name(&mo.steps);
                temp.unwrap_or_else(|| "unnamed".to_string())
            } else {
                mo.mode_name
            };
            format!("{}, {}ᵉ mode of {}\n", mode_name, mo.mode_nr + 1, mo.fam_name)
        } else {
            format!("{}\n", namer.name(&steps).unwrap_or_else(|| "unnamed".to_string()))
        };
        string.push_str(
            &steps.as_relative_intervals(&ionian::steps()).unwrap()
            .as_ionian_relative_string_try(true).unwrap()
        );
        string.push('\n');
        string.push_str(&step_chords_string(&steps, root, style));
        string.push('\n');
        res.push(("Heptatonic Scale".to_string(), mem::take(&mut string)));
    }

    let inversions = {
        let mut inversions = rchord.as_all_inversions();
        inversions.pop();
        inversions
    };
    inversions
        .into_iter().map(|c| (c.as_string(style), c))
        .filter(|(s, _)| !s.contains('[') && !s.is_empty())
        .map(|(mut s, c)| { let _ = write!(s, ": {}", c.to_scale().to_pcs().vec_string()); s })
        .for_each(|s| { let _ = writeln!(string, "{}", s); });
    res.push(("Inversions".to_string(), mem::take(&mut string)));

    rchord
        .as_subs(Some(7))
        .into_iter().map(|c| (c.as_string(style), c))
        .filter(|(s, _)| !s.contains('[') /* && !s.contains('(') */ && !s.is_empty())
        .for_each(|(s, c)| { let _ = writeln!(string, "{}: {:?}", s, c.to_scale().to_pcs()); });
    res.push(("Sub Chords".to_string(), mem::take(&mut string)));

    if let Some(ctwts) = rchord.as_chordtone_wholetone_scale(){
        let mo = find_scale(&ctwts);
        if let Some(m) = mo{
            included.insert((ctonic, m.steps.clone()));
            let spelled_out = spell_out(m.steps.as_scale_try(root).unwrap());
            let _ = write!(string, "{}", mode_format(map_pc_to_en(ctonic), m, spelled_out));
        }

        if !ctwts.is_empty() {
            let ctwts = ctwts.to_steps(true);
            let _ = write!(string, "{}", step_chords_string(&ctwts, root, style));
        }
        res.push(("Chordtone Wholetone Scale".to_string(), mem::take(&mut string)));
    }

    for modeobj in find_chordscales(&pcs){
        if included.contains(&(ctonic, modeobj.steps.clone())) { continue; }
        included.insert((ctonic, modeobj.steps.clone()));
        let spelled_out = spell_out(modeobj.steps.as_scale_try(root).unwrap());
        string.push_str(&mode_format(map_pc_to_en(ctonic), modeobj, spelled_out));
    }
    res.push(("Strict Chordscales".to_string(), mem::take(&mut string)));

    for (tonic, modeobj) in find_scale_superstring(&scale){
        if included.contains(&(tonic, modeobj.steps.clone())) { continue; }
        included.insert((tonic, modeobj.steps.clone()));
        let spelled_out = spell_out(modeobj.steps.as_scale_try(tonic.to_note()).unwrap());
        string.push_str(&mode_format(map_pc_to_en(tonic), modeobj, spelled_out));
    }
    res.push(("Super Strings".to_string(), mem::take(&mut string)));

    for (tonic, modeobj) in find_scale_superset(&pcs, false){
        if included.contains(&(tonic, modeobj.steps.clone())) { continue; }
        included.insert((tonic, modeobj.steps.clone()));
        let spelled_out = spell_out(modeobj.steps.as_scale_try(tonic.to_note()).unwrap());
        string.push_str(&mode_format(map_pc_to_en(tonic), modeobj, spelled_out));
    }
    res.push(("Super Sets".to_string(), mem::take(&mut string)));
    res
}

