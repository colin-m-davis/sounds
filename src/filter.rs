pub trait Filter {
    fn apply(&self, input: f32) -> f32;
}

pub struct LowPassFilter {
    // parameters
}

impl Filter for LowPassFilter {
    fn apply(&self, input: f32) -> f32 {
        // apply low-pass filtering
        unimplemented!()
    }
}

pub struct HighPassFilter {
    // parameters
}

impl Filter for HighPassFilter {
    fn apply(&self, input: f32) -> f32 {
        // apply high-pass filtering
        unimplemented!()
    }
}
