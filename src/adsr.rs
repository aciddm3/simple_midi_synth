#[derive(Debug, Clone, Copy)]
pub struct Adsr {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub current_val: f32,
    pub is_gated: bool,
    pub phase: ADSRPhase,
}

#[derive(Debug, Clone, Copy)]
pub enum ADSRPhase {
    Attack,
    Decay,
    Sustain,
    Release,
    Idle,
}

impl Adsr {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
            current_val: 0.0,
            is_gated: false,
            phase: ADSRPhase::Idle,
        }
    }

    pub fn gate_on(&mut self) {
        self.is_gated = true;
    }

    pub fn gate_off(&mut self) {
        self.is_gated = false;
    }

    pub fn set_gate(&mut self, gate_val: bool) {
        self.is_gated = gate_val;
    }

    pub fn reset(&mut self) {
        self.current_val = 0.0;
        self.phase = ADSRPhase::Idle;
    }

    pub fn step(&mut self, step: f32) {
        if self.is_gated {
            match self.phase {
                ADSRPhase::Idle => {
                    self.phase = ADSRPhase::Attack;
                }
                ADSRPhase::Attack => {
                    if self.attack < std::f32::EPSILON {
                        self.current_val = 1.0;
                    } else {
                        self.current_val += step / self.attack;
                    }

                    if self.current_val >= 1.0 {
                        self.phase = ADSRPhase::Decay;
                    }
                }
                ADSRPhase::Decay => {
                    let der = self.decay * (1.0 - self.sustain);
                    if der < std::f32::EPSILON {
                        self.current_val = self.sustain;
                    } else {
                        self.current_val -= step / der;
                    }

                    if self.current_val <= self.sustain {
                        self.phase = ADSRPhase::Sustain;
                        self.current_val = self.sustain;
                    }
                }
                ADSRPhase::Sustain => self.current_val = self.sustain,
                _ => self.phase = ADSRPhase::Idle,
            }
        } else {
            // self.is_gated() == false
            if self.current_val > 0.0 {
                self.phase = ADSRPhase::Release;
                let der = self.release * self.sustain;
                if der < std::f32::EPSILON {
                    self.current_val = 0.0;
                } else {
                    self.current_val -= step / self.release;
                }
            } else {
                self.phase = ADSRPhase::Idle;
                self.current_val = 0.0;
            }
        };
        self.current_val = self.current_val.clamp(0.0, 1.0);
    }

    pub fn get_value(&self) -> f32 {
        self.current_val
    }
}
