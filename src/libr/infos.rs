// use crate::libr::scales::{get_all_scale_objs,ionian,HeptatonicScaleNamer};
// use crate::theory::scale::RelativeTrait;
// use crate::theory::note::{ToRelative};
// use crate::theory::chord::{strs_scale_chords_roman,ChordStyling};

pub trait Intercalatable{
    type InterType;
    type JoinType;
    fn intercalate(self, val: Self::InterType) -> Self::JoinType;
    fn intercalate_with_end(self, val: Self::InterType, end: Self::InterType) -> Self::JoinType;
}

impl Intercalatable for Vec<String>{
    type InterType = String;
    type JoinType = String;

    fn intercalate(self, val: Self::InterType) -> Self::JoinType{
        let mut builder = String::new();
        let mut iter = self.into_iter();
        let first = iter.next();
        if let Some(s) = first{
            builder.push_str(&s);
        }
        for string in iter{
            builder.push_str(&val);
            builder.push_str(&string);
        }
        builder
    }

    fn intercalate_with_end(self, val: Self::InterType, end: Self::InterType) -> Self::JoinType{
        let mut res = self.intercalate(val);
        res.push_str(&end);
        res
    }
}

pub fn format_even(strings: &[String], spaces: usize, end: &str) -> String{
    let mut res = String::new();
    for string in strings{
        let len = string.chars().count();
        if len < spaces{
            res.push_str(string);
            for _ in 0..spaces - len{
                res.push(' ');
            }
        } else {
            for (i, ch) in string.chars().enumerate(){
                if i >= spaces - 1{
                    break;
                }
                res.push(ch);
            }
            res.push('`');
        }
    }
    res.push_str(end);
    res
}

pub fn format_to_grid_auto(strings: &[String], width: usize, padding: usize) -> String{
    let mut res = String::new();
    let mut longest = 0;
    for string in strings{
        let len = string.chars().count();
        longest = longest.max(len);
    }
    let max = width / (longest + padding);
    let mut count = 0;
    let mut line = Vec::new();
    for string in strings{
        if count > max {
            let line_res = format_even(&line, longest + padding, "\n");
            res.push_str(&line_res);
            line.clear();
            count = 0;
        }
        line.push(string.clone());
        count += 1;
    }
    let line_res = format_even(&line, longest + padding, "\n");
    res.push_str(&line_res);
    res
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn intercalate(){

        assert_eq!(&vec!["X".to_string()].intercalate(",".to_string()), "X");
        assert_eq!(
            &vec!["X", "Y", "Z"].iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .intercalate("--".to_string()),
            "X--Y--Z"
        );
        assert_eq!(
            &vec!["X", "Y", "Z"].iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .intercalate_with_end("--".to_string(), "\n".to_string()),
            "X--Y--Z\n"
        );
    }

    #[test]
    fn format_even_test(){
        let v = vec![
            "AAAA".to_string(),
            "BBBBBBBB".to_string(),
            "CCCC".to_string()
        ];
        assert_eq!(&format_even(&v, 6, "\n"), "AAAA  BBBBB`CCCC  \n");
    }

    #[test]
    fn format_to_grid_auto_test(){
        let v = vec![
            "X", "XX", "XXX", "XXXX", "XXXXXX", "XXXXX",
            "Y", "YY", "YYY", "YYYY", "YYYYYY", "YYYYY",
            "Z", "ZZ", "ZZZ", "ZZZZ", "ZZZZZZ", "ZZZZZ"
        ].iter().map(|x| x.to_string()).collect::<Vec<_>>();
        let res = format_to_grid_auto(&v, 20, 2);
        assert_eq!(
            &res,
            "X       XX      XXX     \nXXXX    XXXXXX  XXXXX   \nY       YY      YYY     \nYYYY    YYYYYY  YYYYY   \nZ       ZZ      ZZZ     \nZZZZ    ZZZZZZ  ZZZZZ   \n"
        );
    }
}
