extern crate music_theory;
use music_theory::{ theory::*, query::* };

// jazzbøt
#[cfg(not(tarpaulin))]
fn main(){
    let args = lapp::parse_args("
        -c, --chord (default '') comma seperated vector of notes, interpreted as chord
        -t, --test testing output
        --chordstyling (default 'std') can be std, extended, spelled
    ");
    let chord = args.get_string("chord");
    let test = args.get_bool("test");
    let style = match args.get_string("chordstyling").as_ref(){
        "extended" => ChordStyle::Extra(MStyle::Symbol, EStyle::Symbol),
        "spelled" => ChordStyle::Spelled,
        _ => ChordStyle::Std(MStyle::Symbol, EStyle::Symbol),
    };
    if test { dotest(); }
    if !chord.is_empty() {
        let res = music_theory::notes_analysis(chord, style);
        for (header, content) in res{
            println!("\t{}", header);
            println!("{}", content);
        }
    }
}

#[cfg(not(tarpaulin))]
fn dotest(){
    let style = ChordStyle::Extra(MStyle::Symbol, EStyle::Symbol);
    print!("{}", scales_and_chords_printout(style));
}
