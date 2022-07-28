
macro_rules! ImplAssign{ ($assigntrait:ty, $implementee:ty, $funcname:ident, $innerfunc:ident) => {
        impl $assigntrait for $implementee{
            fn $funcname(&mut self, other: Self){
                *self = self.$innerfunc(other);
            }
        }
    }
}

pub(crate) use ImplAssign;

pub fn is_sorted<T: PartialOrd + Copy>(v: &[T]) -> bool{
    let mut last = v[0];
    for x in v{
        if last > *x { return false; }
        last = *x;
    }
    true
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_is_sorted(){
        assert!(is_sorted(&[0]));
        assert!(is_sorted(&[0, 1]));
        assert!(is_sorted(&[1, 1]));
        assert!(!is_sorted(&[3, 1]));
        assert!(!is_sorted(&[3, 4, 3, 5]));
    }
}
