use crate::libr::scales::{get_all_scale_objs,ionian};
use crate::theory::scale::RelativeTrait;
use crate::theory::note::{ToRelative};
use crate::theory::chord::{strs_scale_chords_roman,ChordStyling};

pub fn print_splitted(strings: &Vec<String>, split: &str, end: &str){
    for s in strings{
        print!("{}{}", s, split);
    }
    print!("{}", end);
}

pub fn print_even(strings: &Vec<String>, spaces: usize, end: &str){
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

pub fn print_even_auto(strings: &Vec<String>, end: &str){
    let mut longest = 0;
    for string in strings{
        let len = string.chars().count();
        if len > longest{
            longest = len;
        }
    }
    print_even(strings, longest + 1, end);
}

pub fn print_even_grid(lines: &Vec<Vec<String>>, spaces: usize, end: &str){
    for line in lines{
        print_even(line, spaces, end);
    }
}

pub fn print_even_grid_auto(lines: &Vec<Vec<String>>, end: &str){
    let mut longest = 0;
    for line in lines{
        for string in line{
            let len =  string.chars().count();
            if len > longest{
                longest = len;
            }
        }
    }
    print_even_grid(lines, longest + 1, end);
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
