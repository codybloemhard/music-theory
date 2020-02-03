use crate::theory::note::Note;

pub fn map<T,F>(inp: &[T], f: &F) -> Vec<T>
    where
        F: Fn(&T) -> T,
{
    let mut res = Vec::new();
    for x in inp{
        res.push(f(x));
    }
    res
}

pub fn remove_items<T>(vec: &mut Vec<T>, item: &T)
    where
        T: std::cmp::PartialEq
{
    let mut indicis = Vec::new();
    for (i,x) in vec.iter().enumerate(){
        if x == item{
            indicis.push(i);
        }
    }
    for ind in indicis{
        vec.remove(ind);
    }
}

pub fn both_differences<T>(a: &Vec<T>, b: &Vec<T>) -> (Vec<T>,Vec<T>)
    where
        T: std::cmp::PartialEq + std::marker::Copy
{
    let mut notina = Vec::new();
    let mut notinb = Vec::new();
    for x in a{
        if !b.contains(x){
            notinb.push(*x);
        }
    }
    for x in b{
        if !a.contains(x){
            notina.push(*x);
        }
    }
    (notina, notinb)
}

pub fn add_note(notes: &mut Vec<Note>, note: Note){
    for x in notes{
        *x += note;
    }
}
