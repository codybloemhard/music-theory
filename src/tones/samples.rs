use crate::mathh::{Clampable};
use std::f32::consts::PI;

pub fn sine_sample(t: f32, hz: f32) -> f32{
    (t * hz * 2.0 * PI).sin()
}

pub fn square_sample(t: f32, hz: f32) -> f32{
    ((t * hz * 2.0 * PI).sin() * 10000.0).cclamp(-1.0, 1.0)
}

pub fn topflat_sine_sample(t: f32, hz: f32) -> f32{
    (t * hz * 2.0 * PI).sin().min(0.0) * 2.0 + 1.0
}

pub fn botflat_sine_sample(t: f32, hz: f32) -> f32{
    (t * hz * 2.0 * PI).sin().max(0.0) * 2.0 - 1.0
}

pub fn triangle_sample(t: f32, hz: f32) -> f32{
    4.0 * ((t * hz) - ((t * hz) + 0.5).floor()).abs() - 1.0
}
