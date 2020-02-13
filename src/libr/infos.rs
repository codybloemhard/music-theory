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

// pub fn print_scale_obj_modes(obj: &ScaleObj){
//
// }
