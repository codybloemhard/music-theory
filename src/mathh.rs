/*
error[E0658]: use of unstable library feature 'clamp'
  --> src/tones/tone_pure.rs:17:43
   |
17 |     ((t * hz * 2.0 * PI).sin() * 10000.0).clamp(-1.0, 1.0) * vol
   |                                           ^^^^^
uNsTaBlEfEaTuRe
*/
pub trait Clampable{
    fn cclamp(&self, l: Self, u: Self) -> Self;
}

impl Clampable for f32{
    fn cclamp(&self, lower: Self, upper: Self) -> Self{
        if self < &lower { return lower; }
        if self > &upper { return upper; }
        *self
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_clamp0(){
        assert_eq!((3.0f32.cclamp(2.0,4.0) - 3.0).abs() < 0.001, true);
    }
    #[test]
    fn test_clamp1(){
        assert_eq!((3.0f32.cclamp(3.0,4.0) - 3.0).abs() < 0.001, true);
    }
    #[test]
    fn test_clamp2(){
        assert_eq!((3.0f32.cclamp(2.0,3.0) - 3.0).abs() < 0.001, true);
    }
    #[test]
    fn test_clamp3(){
        assert_eq!((2.0f32.cclamp(3.0,4.0) - 3.0).abs() < 0.001, true);
    }
    #[test]
    fn test_clamp4(){
        assert_eq!((4.0.cclamp(2.0,3.0) - 3.0).abs() < 0.001, true);
    }
}
