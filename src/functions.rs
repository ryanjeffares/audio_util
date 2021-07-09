use crate::consts;

pub fn mtof(note: u8) -> f32 {
    ((f32::from(note - consts::A4_PITCH)) / 12.0).exp2() * consts::A4_FREQUENCY
}

pub fn ftom(freq: f32) -> u8 {
    consts::A4_PITCH + (12.0 * (freq / consts::A4_FREQUENCY).log2()) as u8
}