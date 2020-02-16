use crate::theory::scale::{Scale,notes_to_octave_scale, mode_nr_of_scale};

pub fn find_scale(scale: &Scale) -> String{
    let steps = notes_to_octave_scale(scale);
    let scales = crate::libr::scales::get_all_scale_objs();
    for sc in scales{
        if let Some(mode) = mode_nr_of_scale(&steps, sc.steps.clone()){
            println!("{}", mode);
            return sc.get_mode_name(mode as u8);
        }
    }
    String::from("unnamed")
}
