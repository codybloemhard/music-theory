use crate::libr::scales::{get_all_scale_objs,ionian};
use crate::theory::scale::RelativeTrait;
use crate::theory::note::{ToRelative};
use crate::theory::chord::{strs_scale_chords_roman,ChordStyling};

pub fn print_splitted(strings: &[String], split: &str, end: &str){
    for s in strings{
        print!("{}{}", s, split);
    }
    print!("{}", end);
}

pub fn format_splitted(strings: &[String], split: &str, end: &str) -> String{
    let mut string = String::new();
    for s in strings{
        let x = format!("{}{}", s, split);
        string.push_str(&x);
    }
    string.push_str(&end.to_string());
    string
}

pub fn print_even(strings: &[String], spaces: usize, end: &str){
    for string in strings{
        let len = string.chars().count();
        if len < spaces{
            print!("{}", string);
            for _ in 0..spaces - len{
                print!(" ");
            }
        }else{
            for (i,ch) in string.chars().enumerate(){
                if i >= spaces - 1{
                    break;
                }
                print!("{}", ch);
            }
            print!("`");
        }
    }
    print!("{}", end);
}

pub fn print_to_grid_auto(strings: &[String], width: usize, padding: usize){
    let mut longest = 0;
    for string in strings{
        let len = string.chars().count();
        longest = longest.max(len);
    }
    let max = width / (longest + padding);
    let mut count = 0;
    let mut line = Vec::new();
    for string in strings{
        if count > max {
            print_even(&line, longest + padding, "\n");
            line.clear();
            count = 0;
        }
        line.push(string.clone());
        count += 1;
    }
}

pub fn print_scales(styling: ChordStyling){
    let objs = get_all_scale_objs();
    for sobj in objs{
        println!("{}", sobj.family_name());
        for mode in sobj.get_modes(){
            println!("{}: {}", mode.mode_nr, mode.mode_name);
            println!("\t{}", mode.steps.to_relative(&ionian::steps()).unwrap().string_ionian_rel());
            let c3 = strs_scale_chords_roman(&mode.steps, 3, styling);
            let c4 = strs_scale_chords_roman(&mode.steps, 4, styling);
            print!("\t");
            print_splitted(&c3, ", ", "\n");
            print!("\t");
            print_splitted(&c4, ", ", "\n");
        }
    }
}
