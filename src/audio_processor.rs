//! Trait for an audio utility that process samples, either single samples or blocks of samples.

pub trait AudioProcessor {
    /// Process a single sample, and return the output.    
    fn process_sample(&mut self, sample: f32) -> f32;

    /// Set the current sample rate - it is very important to do this whenever our sample rate changes.
    fn set_sample_rate(&mut self, sr: f32);
}