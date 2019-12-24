
pub fn hit_lin(slope: f32) -> impl Fn(f32,f32) -> f32{
    move |_, t| (1.0 - (t*slope)).max(0.0)
}

pub fn hit_quad(factor: f32, power: f32) -> impl Fn(f32,f32) -> f32{
    move |_, t| (1.0 - (t*factor).powf(power)).max(0.0)
}

pub fn hit_lin_quad(slope: f32, factor: f32, power: f32) -> impl Fn(f32,f32) -> f32{
    move |_, t| (10.0*t).min(1.0 - ((t-(1.0/slope))*factor).powf(power)).max(0.0)
}
