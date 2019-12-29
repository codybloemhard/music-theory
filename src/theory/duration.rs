#[derive(Clone,Copy)]
pub enum Duration{
    Double,
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    Triplet,
    Quintuplet,
    Sextuplet,
    Septuplet,
}

impl Duration{
    pub fn to_float(self) -> f32{
        match self{
            Self::Double => 2.0,
            Self::Whole => 1.0, 
            Self::Half => 0.5,
            Self::Quarter => 0.25,
            Self::Eighth => 0.125,
            Self::Sixteenth => 0.0625,
            Self::Triplet => 1.0/3.0,
            Self::Quintuplet => 0.2,
            Self::Sextuplet => 1.0/6.0,
            Self::Septuplet => 1.0/7.0,
        }
    }

    pub fn divide(self, interval: f32) -> f32{
        interval * self.to_float()
    }
}
