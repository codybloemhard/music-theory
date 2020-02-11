pub fn print_even(strings: &Vec<String>, spaces: usize, end: &str){
    for string in strings{
        let len = string.len();
        if len <= spaces{
            print!("{}", string);
            for _ in 0..spaces - len{
                print!(" ");
            }
        }else{
            for (i,ch) in string.chars().enumerate(){
                if i >= spaces{
                    break;
                }
                println!("{}", ch);
            }
        }
    }
    print!("{}", end);
}

// pub fn print_scale_obj_modes(obj: &ScaleObj){
//
// }
