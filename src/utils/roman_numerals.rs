pub const RNVALS: [usize; 13] = [1000,900,500,400,100,90,50,40,10,9,5,4,1];
pub const RNNAME: [&'static str; 13] = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];

pub fn to_roman_num(mut dec: usize) -> String{
    let mut res = String::new();
    while dec != 0{
        for (i,rnval) in RNVALS.iter().enumerate(){
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

pub struct RomanNumeralIter{
    current: usize,
}

impl RomanNumeralIter{
    pub fn new() -> Self{
        Self{
            current: 0,
        }
    }
}
impl Default for RomanNumeralIter {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for RomanNumeralIter{
    type Item = String;
    fn next(&mut self) -> Option<String>{
        let res = to_roman_num(self.current);
        self.current += 1;
        Some(res)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_roman0(){
        assert_eq!(to_roman_num(0), "");
    }
    #[test]
    fn test_roman1(){
        assert_eq!(to_roman_num(1), "I")
    }
    #[test]
    fn test_roman2(){
        assert_eq!(to_roman_num(2), "II")
    }
    #[test]
    fn test_roman3(){
        assert_eq!(to_roman_num(3), "III")
    }
    #[test]
    fn test_roman4(){
        assert_eq!(to_roman_num(4), "IV")
    }
    #[test]
    fn test_roman5(){
        assert_eq!(to_roman_num(5), "V")
    }
    #[test]
    fn test_roman6(){
        assert_eq!(to_roman_num(37), "XXXVII")
    }
    #[test]
    fn test_roman7(){
        assert_eq!(to_roman_num(666), "DCLXVI")
    }
    #[test]
    fn test_roman8(){
        assert_eq!(to_roman_num(1998), "MCMXCVIII")
    }
    #[test]
    fn test_roman9(){
        assert_eq!(to_roman_num(12345), "MMMMMMMMMMMMCCCXLV")
    }
}
