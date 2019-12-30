pub fn hit_lin(slope: f32) -> impl Fn(f32,f32) -> f32{
    move |_, t| (1.0 - (t*slope)).max(0.0)
}

pub fn hit_quad(factor: f32, power: f32) -> impl Fn(f32,f32) -> f32{
    move |_, t| (1.0 - (t*factor).powf(power)).max(0.0)
}

pub fn hit_quad_seconds(seconds: f32, power: f32) -> impl Fn(f32,f32) -> f32{
    move |_, t| (1.0 - (t*(1.0/seconds)).powf(power)).max(0.0)
}

pub fn hit_lin_quad(slope: f32, factor: f32, power: f32) -> impl Fn(f32,f32) -> f32{
    move |_, t| (slope*t).min(1.0 - ((t-(1.0/slope))*factor).powf(power)).max(0.0)
}

pub fn hit_lin_quot(slope: f32, dividend: f32) -> impl Fn(f32,f32) -> f32{
    move |_, t| (slope*t).min(dividend / (t + dividend - (1.0 / slope))).max(0.0)
}

pub fn hit_lin_quot_quad(slope: f32, dividend: f32, die_second: f32, sharpness: u16) -> impl Fn(f32,f32) -> f32{
    move |_, t| (slope*t).min(dividend / (t + dividend - (1.0 / slope))).min(1.0 - (t*(1.0/die_second)).powf(sharpness as f32)).max(0.0)
}
