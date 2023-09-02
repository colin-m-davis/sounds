pub trait Sound {
    fn sample(&self, time: f32) -> f32;
}
