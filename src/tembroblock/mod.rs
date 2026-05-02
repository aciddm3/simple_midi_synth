pub mod common_envelope;
pub mod oscillator;

use common_envelope::*;
use oscillator::*;

#[derive(Debug, Clone)]
pub struct Tembroblock {
    pub oscillator: SineOscillator,
    pub amplitude_envelope: CommonEnvelope,
    pub phase_envelope: CommonEnvelope,
    pub pitch_envelope: CommonEnvelope,
    value: f32,
}

impl Tembroblock {
    #[inline]
    pub fn new(
        oscillator: SineOscillator,
        amplitude_envelope: CommonEnvelope,
        phase_envelope: CommonEnvelope,
        pitch_envelope: CommonEnvelope,
    ) -> Self {
		Self {
			oscillator,
			amplitude_envelope,
			phase_envelope,
			pitch_envelope,
			value : 0.0,
		}
	}

    #[inline]
    pub fn do_dt(&mut self, dt_osc: f32, dt_env: f32) -> f32 {
        self.value = self.amplitude_envelope.do_dt(dt_env)
            * self.oscillator.do_dt(
                dt_osc * 2.0f32.powf(self.pitch_envelope.do_dt(dt_env))
                    + self.phase_envelope.do_dt(dt_env),
            );
        self.value
    }

    #[inline]
    pub fn gate_off(&mut self) {
        self.amplitude_envelope.gate_off();
        self.phase_envelope.gate_off();
        self.pitch_envelope.gate_off();
    }

    #[inline]
    pub fn gate_on(&mut self) {
        self.amplitude_envelope.gate_on();
        self.phase_envelope.gate_on();
        self.pitch_envelope.gate_on();
    }

    #[inline]
    pub fn reset(&mut self) {
        self.gate_off();
        self.oscillator.reset();
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }
}
