extern crate music_gen;
use music_gen::theory::*;
use music_gen::libr::scales::*;
use music_gen::libr::infos::*;
use music_gen::query::*;

fn main(){
    test();
}

fn test(){
    for named in &ucns_to_named(&[C,CS,E,F,G,GS,AS], 3){
        print!("{}, ", named.to_string());
    }
    println!();
    println!("{}", find_scale(&vec![C,CS,E,F,G,GS,AS].into_scale(0)).unwrap());
    println!("\n");
    for modeobj in find_scale_superseq(&vec![A,B,C,D].into_steps()){
        println!("{}", modeobj);
    }
    print_scales(ChordStyling::Extended);
    let subset = vec![C,E,G,B];
    print!("Scales which are an superset to {{");
    for n in &subset{
        print!("{}, ", n);
    }
    println!("}}");
    for (tonic,modeobj) in find_scale_superset(subset.clone(), true){
        println!("{} {}", tonic, modeobj);
    }
    println!("And the strict chordscales: ");
    for modeobj in find_chordscales(subset.into_steps()){
        println!("{}", modeobj);
    }
    let res = find_scale_from_ionian_relative(Relative(vec![
        RN_NAT, RN_S, RN_NAT, RN_S, RN_NAT, RN_S, RN_NAT,
    ]));
    println!("-------");
    for modeobj in res{
        println!("{}", modeobj);
    }
    println!("-------");
    let subchords = scale_sub_chords(ionian::obj().clone_steps().mode(6).into_scale(A4));
    for sc in subchords{
        let name = sc.as_string(true, ChordStyling::Extended);
        if name.contains('[') { continue; }
        if name.contains('(') { continue; }
        println!("{}: {:?}", name, sc.to_scale().into_ucns());
    }
}

