//! Create an oscillator that can be used as a sound sorce or an LFO.
//! This is a basic oscillator with set a small variety of shapes.

use crate::{audio_source, consts::{self, TWO_PI}};


pub struct Oscillator {
    osc_type: OscillatorType,
    frequency: f32,
    phase: f32,
    output: f32,
    pulsewidth: f32,    // only used if oscillator type is Pulse
    sample_rate: f32
}

pub enum OscillatorType {
    Sin, Saw, Square, Pulse, Triangle
}

impl Oscillator {
    fn new(osc_type: OscillatorType, freq: f32, pw: f32, sr: f32) -> Self {
        Oscillator {
            osc_type: osc_type,
            frequency: freq,
            phase: 0.0,
            output: 0.0,
            pulsewidth: pw,
            sample_rate: sr
        }
    }        

    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }

    pub fn set_pulsewidth(&mut self, pw: f32) {
        self.pulsewidth = pw;
    }

    // The given pitch mod will be the fraction of the current frequency that is added 
    // e.g, giving 1.0 would double the current frequency, 0.5 would give a perfect fifth above, -1.0 would give nothing
    pub fn get_next_sample_with_pitch_mod(&mut self, pitch_mod: f32) -> f32 {
        if self.frequency <= 0.0 {
            0.0
        }
        else {
            match self.osc_type {
                OscillatorType::Sin => {
                    self.output = f32::sin(self.phase * consts::TWO_PI);
                    if self.phase >= 1.0 {
                        self.phase -= 1.0;
                    }
                    self.phase += 1.0 / (self.sample_rate / (self.frequency + (self.frequency * pitch_mod)));
                    self.output
                }
                OscillatorType::Saw => {
                    self.output = self.phase;
                    if self.phase >= 1.0 {
                        self.phase -= 2.0;
                    }
                    self.phase += (1.0 / (self.sample_rate / (self.frequency + (self.frequency * pitch_mod)))) * 2.0;                                
                    self.output
                }
                OscillatorType::Square => {
                    if self.phase >= 1.0 {
                        self.phase -= 1.0;
                    }
                    self.phase += 1.0 / (self.sample_rate / (self.frequency + (self.frequency * pitch_mod)));
                    if self.phase < 0.5 { 
                        1.0
                    } else { 
                        -1.0
                    }
                }
                OscillatorType::Triangle => {
                    if self.phase >= 1.0 {
                        self.phase -= 1.0;
                    }
                    self.phase += 1.0 / (self.sample_rate / (self.frequency + (self.frequency * pitch_mod)));
                    if self.phase <= 0.5 {
                        self.output = (self.phase - 0.25) * 4.0;
                    }
                    else {
                        self.output = ((1.0 - self.phase) - 0.25) * 4.0;
                    }
                    self.output
                }
                OscillatorType::Pulse => {
                    if self.phase >= 1.0 {
                        self.phase -= 1.0;
                    }
                    if self.pulsewidth >= 1.0 { self.pulsewidth = 1.0; }
                    if self.pulsewidth <= 0.0 { self.pulsewidth = 0.0; }
                    self.phase += 1.0 / (self.sample_rate / (self.frequency + (self.frequency * pitch_mod)));
                    if self.phase < self.pulsewidth { 
                        1.0
                    } else { 
                        -1.0
                    }
                }
            }
        }   
    }
}

impl Default for Oscillator {
    fn default() -> Self {
        Oscillator {
            osc_type: OscillatorType::Sin,
            frequency: consts::A4_FREQUENCY,
            phase: 0.0,
            output: 0.0,
            pulsewidth: 0.5,
            sample_rate: 44100.0  
        }
    }
}

impl audio_source::AudioSource for Oscillator {
    fn get_next_sample(&mut self) -> f32 {
        if self.frequency <= 0.0 {
            0.0
        }
        else {
            match self.osc_type {
                OscillatorType::Sin => {
                    self.output = f32::sin(self.phase * TWO_PI);
                    if self.phase >= 1.0 {
                        self.phase -= 1.0;
                    }
                    self.phase += 1.0 / (self.sample_rate / self.frequency);
                    self.output
                }
                OscillatorType::Saw => {
                    self.output = self.phase;
                    if self.phase >= 1.0 {
                        self.phase -= 2.0;
                    }
                    self.phase += (1.0 / (self.sample_rate / self.frequency)) * 2.0;                                
                    self.output
                }
                OscillatorType::Square => {
                    if self.phase >= 1.0 {
                        self.phase -= 1.0;
                    }
                    self.phase += 1.0 / (self.sample_rate / self.frequency);
                    if self.phase < 0.5 { 
                        1.0
                    } else { 
                        -1.0
                    }
                }
                OscillatorType::Triangle => {
                    if self.phase >= 1.0 {
                        self.phase -= 1.0;
                    }
                    self.phase += 1.0 / (self.sample_rate / self.frequency);
                    if self.phase <= 0.5 {
                        self.output = (self.phase - 0.25) * 4.0;
                    }
                    else {
                        self.output = ((1.0 - self.phase) - 0.25) * 4.0;
                    }
                    self.output
                }
                OscillatorType::Pulse => {
                    if self.phase >= 1.0 {
                        self.phase -= 1.0;
                    }
                    if self.pulsewidth >= 1.0 { self.pulsewidth = 1.0; }
                    if self.pulsewidth <= 0.0 { self.pulsewidth = 0.0; }
                    self.phase += 1.0 / (self.sample_rate / self.frequency);
                    if self.phase < self.pulsewidth { 
                        1.0
                    } else { 
                        -1.0
                    }
                }
            }
        }        
    }

    fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
    }
}