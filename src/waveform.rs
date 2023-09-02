use std::f32::consts::PI;
use super::sound::Sound;

pub struct SineWave {
    pub amplitude: f32,
    pub frequency: f32,
}

impl Sound for SineWave {
    fn sample(&self, time: f32) -> f32 {
        self.amplitude * (2.0 * PI * self.frequency * time).sin()
    }
}

pub struct SawtoothWave {
    pub amplitude: f32,
    pub frequency: f32,
}

impl Sound for SawtoothWave {
    fn sample(&self, time: f32) -> f32 {
        self.amplitude * (2.0 * (time * self.frequency).fract() - 1.0)
    }
}
