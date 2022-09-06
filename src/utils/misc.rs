use itertools::*;

macro_rules! impl_op{
    ($assigntrait: ty, $implementee: ty, $outtype: ty, $funcname: ident, $innerfunc: ident,
     $wrapping: path
    ) => {
        impl $assigntrait for $implementee{
            type Output = $outtype;

            fn $funcname(self, other: Self) -> Self::Output{
                $wrapping(self.0.$innerfunc(other.0))
            }
        }
    }
}
pub(crate) use impl_op;

macro_rules! impl_op_assign{
    ($assigntrait:ty, $implementee:ty, $funcname:ident, $innerfunc:ident) => {
        impl $assigntrait for $implementee{
            fn $funcname(&mut self, other: Self) {
                *self = self.$innerfunc(other);
            }
        }
    }
}
pub(crate) use impl_op_assign;

/// Check if a slice is sorted.
/// Somehow this is not stable in std...
///
/// Example:
/// ```
/// use music_theory::utils::misc::*;
/// assert!(!is_sorted(&[6, 7, 6, 9]));
/// assert!(is_sorted(&[3, 4, 4, 5]));
/// ```
pub fn is_sorted<T: PartialOrd + Copy>(v: &[T]) -> bool{
    let mut last = v[0];
    for x in v{
        if last > *x { return false; }
        last = *x;
    }
    true
}

/// Generate subsets of a slice.
/// `max_len` is the maximum length of the subsets generated.
/// No subsets larger than `max_len` will be generated.
/// It's a way to reduce output size.
/// The output size of this grows really fast with the input size so be aware:
/// 2, 5, 16, 65, 326, 1957, 13700, 109601, 986410, 9864101, 108505112.
///
/// Example:
/// ```
/// use music_theory::utils::misc::*;
/// assert_eq!(
///     sub_vecs(&[0, 1], None),
///     vec![
///         vec![],
///         vec![0],
///         vec![1],
///         vec![0, 1],
///         vec![1, 0],
///     ]
/// );
/// ```
pub fn sub_vecs<'a, T>(arr: &'a[T], max_len: Option<usize>) -> Vec<Vec<T>>
    where
        &'a[T]: IntoIterator,
        T: Copy
{
    arr.iter().copied()
        .powerset()
        .filter(|ps| ps.len() <= max_len.unwrap_or(usize::MAX))
        .flat_map(|ps| { let l = ps.len(); ps.into_iter().permutations(l).collect::<Vec<_>>() })
        .collect::<Vec<_>>()
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

    #[test]
    fn test_sub_vecs(){
        assert_eq!(
            sub_vecs(&[0, 1], None),
            vec![
                vec![],
                vec![0],
                vec![1],
                vec![0, 1],
                vec![1, 0],
            ]
        );
        assert_eq!(
            sub_vecs(&[0, 1, 2], None),
            vec![
                vec![],
                vec![0],
                vec![1],
                vec![2],
                vec![0, 1],
                vec![1, 0],
                vec![0, 2],
                vec![2, 0],
                vec![1, 2],
                vec![2, 1],
                vec![0, 1, 2],
                vec![0, 2, 1],
                vec![1, 0, 2],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![2, 1, 0],
            ]
        );
    }
}
