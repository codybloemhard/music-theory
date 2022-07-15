
macro_rules! ImplAssign{ ($assigntrait:ty, $implementee:ty, $funcname:ident, $innerfunc:ident) => {
        impl $assigntrait for $implementee{
            fn $funcname(&mut self, other: Self){
                *self = self.$innerfunc(other);
            }
        }
    }
}

pub(crate) use ImplAssign;
