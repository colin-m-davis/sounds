pub trait Filter {
    fn apply(&mut self, input: f32) -> f32;
}

pub struct LowPassFilter {
    alpha: f32,
    last_output: f32,
}

impl LowPassFilter {
    pub fn new(alpha: f32) -> Self {
        Self { alpha, last_output: 0.0 }
    }
}

impl Filter for LowPassFilter {
    fn apply(&mut self, input: f32) -> f32 {
        let output = (1.0 - self.alpha) * input + self.alpha * self.last_output;
        self.last_output = output;
        output
    }
}

pub struct HighPassFilter {
    alpha: f32,
    last_output: f32,
    last_input: f32,
}

impl HighPassFilter {
    pub fn new(alpha: f32) -> Self {
        Self { alpha, last_output: 0.0, last_input: 0.0 }
    }
}

impl Filter for HighPassFilter {
    fn apply(&mut self, input: f32) -> f32 {
        let output = self.alpha * (self.last_output + input - self.last_input);
        self.last_output = output;
        self.last_input = input;
        output
    }
}
