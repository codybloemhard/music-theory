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

