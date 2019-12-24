use std::f32::consts::PI;
use std::i16;
use hound;

pub mod tones;
pub mod constants;
pub mod mathh;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn yeet(){
    println!("YEET");
}

pub fn write_test_tone(){
    let spec = hound::WavSpec{
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for t in (0..44100).map(|x| x as f32 / 44100.0){
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let ampl = i16::MAX as f32;
        writer.write_sample((sample * ampl) as i16).unwrap();
    }
}

pub fn write_test_tone_stereo(){
    let spec = hound::WavSpec{
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for t in (0..44100).map(|x| x as f32 / 44100.0){
        let sa = (t * 220.0 * 2.0 * PI).sin();
        let sb = (t * 440.0 * 2.0 * PI).sin();
        let ampl = i16::MAX as f32;
        writer.write_sample((sa * ampl) as i16).unwrap();
        writer.write_sample((sb * ampl) as i16).unwrap();
    }
}
