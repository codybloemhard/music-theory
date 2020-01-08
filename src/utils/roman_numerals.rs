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

impl Iterator for RomanNumeralIter{
    type Item = String;
    fn next(&mut self) -> Option<String>{
        let res = to_roman_num(self.current);
        self.current += 1;
        Some(res)
    }
}
