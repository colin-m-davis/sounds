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

pub struct TriangleWave {
    pub frequency: f32,
    pub amplitude: f32,
}

impl Sound for TriangleWave {
    fn sample(&self, t: f32) -> f32 {
        let value = (t * self.frequency).fract();
        4.0 * self.amplitude * value.abs() - 1.0
    }
}

pub struct SquareWave {
    pub frequency: f32,
    pub amplitude: f32,
}

impl Sound for SquareWave {
    fn sample(&self, t: f32) -> f32 {
        if (t * self.frequency).fract() < 0.5 {
            self.amplitude
        } else {
            -self.amplitude
        }
    }
}
