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
