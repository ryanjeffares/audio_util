//! Trait for an audio utility that will generate its own samples.

pub trait AudioSource {
    // Generate a single sample - generally this would be called on every sample.
    fn get_next_sample(&mut self) -> f32;

    /// Set the current sample rate - it is very important to do this whenever our sample rate changes.
    fn set_sample_rate(&mut self, sr: f32);
}