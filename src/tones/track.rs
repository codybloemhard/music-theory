use std::i16;

pub struct Track{
    samples: Vec<f32>,
    sample_rate: usize,
    channels: usize,
}

impl Track{
    pub fn new(sample_rate: usize, channels: usize) -> Self{
        Track{
            samples: Vec::new(),
            sample_rate,
            channels,
        }
    }

    pub fn len(&self) -> usize{
        self.samples.len() / self.channels
    }

    pub fn is_empty(&self) -> bool{
        self.samples.is_empty()
    }

    pub fn sample_rate(&self) -> usize{
        self.sample_rate
    }

    pub fn enlongate(&mut self, extra: usize){
        self.samples.resize(self.len() + (extra * self.channels), 0.0);
    }

    pub fn set_sample(&mut self, sample: f32, pos: usize, channel: usize){
        self.samples[pos*self.channels + channel] = sample;
    }

    pub fn add_sample(&mut self, sample: f32, pos: usize, channel: usize){
        self.samples[pos*self.channels + channel] += sample;
    }

    pub fn normalize_per_channel(&mut self){
        let len = self.samples.len() / self.channels;
        for ch in 0..self.channels{
            let mut max = 0.0;
            for i in 0..len{
                let s = self.samples[i * self.channels + ch].abs();
                if s > max{
                    max = s;
                }
            }
            for i in 0..len{
                self.samples[i * self.channels + ch] /= max;
            }
        }
    }

    pub fn normalize(&mut self, vol: f32){
        let len = self.samples.len();
        let mut max = 0.0;
        for sam in 0..len{
            let s = self.samples[sam].abs();
            if s > max{
                max = s;
            }
        }
        for sam in 0..len{
            self.samples[sam] /= max;
            self.samples[sam] *= vol;
        }
    }

    pub fn trim_end(&mut self, epsilon: f32){
        let mut collecting = false;
        let mut i = 0;
        for (j,sam) in self.samples.iter().enumerate(){
            if collecting{
                if sam.abs() > epsilon{
                    collecting = false;
                }
            }else if sam.abs() <= epsilon{
                i = j;
                collecting = true;
            }
        }
        if collecting{
            self.samples.truncate(i);
        }
    }

    pub fn render(&self, filepath: &str){
        let spec = hound::WavSpec{
            channels: self.channels as u16,
            sample_rate: self.sample_rate as u32,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(filepath, spec).expect("Error: src::track::Track::render creating writer failed.");
        for s in &self.samples{
            writer.write_sample((s * ((i16::MAX - 1) as f32)) as i16).expect("Error: src::track::Track::render write_sample failed.");
        }
    }
}
