
macro_rules! ImplAssign{ ($assigntrait:ty, $implementee:ty, $funcname:ident, $innerfunc:ident) => {
        impl $assigntrait for $implementee{
            fn $funcname(&mut self, other: Self){
                *self = self.$innerfunc(other);
            }
        }
    }
}

pub(crate) use ImplAssign;

pub fn as_lowercase(input: &str) -> String{
    let mut lowercase = String::new();
    for c in input.chars(){
        for l in c.to_lowercase(){
            lowercase.push(l);
        }
    }
    lowercase
}
