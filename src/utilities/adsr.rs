//! An ADSR envelope with adjustable times and ratios.
//! Based on the Bela platform's implementation of EarLevel Engineering's ADSR - http://www.earlevel.com/main/2013/06/01/envelope-generators/

fn calculate_coefficient(rate: f32, ratio: f32) -> f32 {
    let log = -((1.0 + ratio) / ratio).log(std::f32::consts::E) / rate;        
    log.exp()
}

pub struct ADSR {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,    
    attack_coeff: f32,
    decay_coeff: f32,
    release_coeff: f32,
    attack_target_ratio: f32,
    dr_target_ratio: f32,    
    attack_base: f32,
    decay_base: f32,
    release_base: f32,
    state: ADSRState,
    sample_rate: f32,
    output: f32
}

#[derive(PartialEq)]
enum ADSRState {
    Idle, Attack, Decay, Sustain, Release
}

impl Default for ADSR {
    fn default() -> ADSR {
        let attack_coeff = calculate_coefficient(0.2, 0.3);
        let decay_coeff = calculate_coefficient(0.2, 0.0001);
        let release_coeff = calculate_coefficient(0.5, 0.0001);
        ADSR {
            attack: 0.2,
            decay: 0.1,
            sustain: 0.8,
            release: 0.5,                        
            attack_coeff,
            decay_coeff,
            release_coeff,
            attack_target_ratio: 0.3,
            dr_target_ratio: 0.0001,
            attack_base: (1.0 + 0.3) * (1.0 - attack_coeff),
            decay_base: (1.0 - 0.0001) * (1.0 - decay_coeff),
            release_base: -0.0001 * (1.0 - release_coeff),
            state: ADSRState::Idle,
            sample_rate: 44100.0,
            output: 0.0
        }
    }
}

impl ADSR {

    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32, sr: f32) -> ADSR {
        let attack_coeff = calculate_coefficient(0.2, 0.3);
        let decay_coeff = calculate_coefficient(0.2, 0.0001);
        let release_coeff = calculate_coefficient(0.5, 0.0001);
        ADSR {
            attack: attack * sr,
            decay: decay * sr,
            sustain,
            release: release * sr,
            attack_coeff,
            decay_coeff,
            release_coeff,
            attack_target_ratio: 0.3,
            dr_target_ratio: 0.0001,
            attack_base: (1.0 + 0.3) * (1.0 - attack_coeff),
            decay_base: (1.0 - 0.0001) * (1.0 - decay_coeff),
            release_base: -0.0001 * (1.0 - release_coeff),
            state: ADSRState::Idle,
            sample_rate: sr,
            output: 0.0
        }
    }

    /// Calling start_note will set the ADSR into attack mode, whether it was idle or in another state
    pub fn start_note(&mut self) {
        self.state = ADSRState::Attack;
    }

    /// Calling end_note will tell the ADSR to start releasing from whatever state it is currently in - unless it is idle, when it will do nothing.
    pub fn end_note(&mut self) {         
        if self.state != ADSRState::Idle {
            self.state = ADSRState::Release;
        }
    }

    /// Calling reset will set the ADSR to idle and its output to 0, if you need it to stop doing anything.
    pub fn reset(&mut self) {
        self.state = ADSRState::Idle;
        self.output = 0.0;
    }

    /// Call this to get the current level (between 0 and 1) of the ADSR - separate to process() so process can be called on every sample, whereas this can be called only when needed.
    pub fn get_output(&self) -> f32 {
        self.output
    }

    /// You may set the attack, decay, sustain, and release params together, or also with attack and decay/release ratios.
    pub fn set_params(&mut self, attack: f32, decay: f32, sustain: f32, release: f32) {
        self.calculate_attack(attack * self.sample_rate);
        self.calculate_decay(decay * self.sample_rate);           
        self.calculate_sustain(sustain);     
        self.calculate_release(release * self.sample_rate);        
    }

    pub fn set_params_with_ratio(&mut self, attack: f32, decay: f32, sustain: f32, release: f32, attack_ratio: f32, dr_ratio: f32) {
        self.calculate_attack(attack * self.sample_rate);
        self.calculate_decay(decay * self.sample_rate);           
        self.calculate_sustain(sustain);     
        self.calculate_release(release * self.sample_rate); 
        self.set_attack_target_ratio(attack_ratio);
        self.set_dr_target_ratio(dr_ratio);
    }

    /// Important to call this whenever our sample rate changes!
    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
    }

    /// This function should be called on every sample that the ADSR is being used.
    pub fn process(&mut self) {
        match self.state {
            ADSRState::Idle => (),
            ADSRState::Attack => {
                self.output = self.attack_base + self.output * self.attack_coeff;
                if self.output >= 1.0 {
                    self.output = 1.0;
                    self.state = ADSRState::Decay;
                }
            },
            ADSRState::Decay => {
                self.output = self.decay_base + self.output * self.decay_coeff;
                if self.output <= self.sustain {
                    self.output = self.sustain;
                    self.state = ADSRState::Sustain;
                }
            },
            ADSRState::Sustain => (),
            ADSRState::Release => {
                self.output = self.release_base + self.output * self.release_coeff;
                if self.output <= 0.0 {
                    self.output = 0.0;
                    self.state = ADSRState::Idle;
                }
            }
        }        
    }
    
    fn calculate_attack(&mut self, attack: f32) {
        self.attack = attack;
        self.attack_coeff = calculate_coefficient(self.attack, self.attack_target_ratio);
        self.attack_base = (1.0 + self.attack_target_ratio) * (1.0 - self.attack_coeff);
    }

    fn calculate_decay(&mut self, decay: f32) {
        self.decay = decay;
        self.decay_coeff = calculate_coefficient(self.decay, self.dr_target_ratio);
        self.decay_base = (self.sustain - self.dr_target_ratio) * (1.0 - self.decay_coeff);
    }

    fn calculate_sustain(&mut self, sustain: f32) {
        self.sustain = sustain;
        self.decay_base = (self.sustain - self.dr_target_ratio) * (1.0 - self.decay_coeff);
    }

    fn calculate_release(&mut self, release: f32) {
        self.release = release;
        self.release_coeff = calculate_coefficient(self.release, self.dr_target_ratio);
        self.release_base = -self.dr_target_ratio * (1.0 - self.release_coeff);
    }    

    fn set_attack_target_ratio(&mut self, mut ratio: f32) {
        if ratio < 0.000000001 {
            ratio = 0.000000001;
        }
        self.attack_target_ratio = ratio;
        self.attack_base = (1.0 + self.attack_target_ratio) * (1.0 - self.attack_coeff);
    }

    fn set_dr_target_ratio(&mut self, mut ratio: f32) {
        if ratio < 0.000000001 {
            ratio = 0.000000001;
        }
        self.dr_target_ratio = ratio;
        self.decay_base = (self.sustain - self.dr_target_ratio) * (1.0 - self.decay_coeff);
        self.release_base = -self.dr_target_ratio * (1.0 - self.release_coeff);
    }
}