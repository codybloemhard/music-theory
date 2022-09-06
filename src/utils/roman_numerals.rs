const RNVALS: [usize; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
const RNNAME: [&str; 13] = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];

/// Convert a [usize][usize] to a string of a roman numeral.
///
/// Example:
/// ```
/// use music_theory::utils::roman_numerals::*;
/// assert_eq!(to_roman_num(2), "II");
/// ```
pub fn to_roman_num(mut dec: usize) -> String{
    let mut res = String::new();
    while dec != 0{
        for (i, rnval) in RNVALS.iter().enumerate(){
            if rnval > &dec {
                continue;
            }
            res.push_str(RNNAME[i]);
            dec -= rnval;
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test(){
        assert_eq!(to_roman_num(0), "");
        assert_eq!(to_roman_num(1), "I");
        assert_eq!(to_roman_num(2), "II");
        assert_eq!(to_roman_num(3), "III");
        assert_eq!(to_roman_num(4), "IV");
        assert_eq!(to_roman_num(5), "V");
        assert_eq!(to_roman_num(37), "XXXVII");
        assert_eq!(to_roman_num(666), "DCLXVI");
        assert_eq!(to_roman_num(1998), "MCMXCVIII");
        assert_eq!(to_roman_num(12345), "MMMMMMMMMMMMCCCXLV");
    }
}
