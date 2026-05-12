use aciddm3_atom_func::func::{EnvFunction, EnvFunctionArguments};

mod slew_limiter;
use slew_limiter::SlewLimiter;

#[derive(Debug, Clone)]
pub struct CommonEnvelope {
    value: f32,
    prev_phase_last_value: f32,
    slew_limiter: SlewLimiter,

    gate_on_func: EnvFunction,
    gate_off_func: EnvFunction,

    phase: EnvPhase,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EnvPhase {
    GateOn(f32),
    GateOff(f32),
}

impl Default for CommonEnvelope {
    fn default() -> Self {
        Self {
            value: 0.0,
            prev_phase_last_value : 0.0,
            slew_limiter: SlewLimiter::new(f32::MIN, f32::MAX),
            gate_on_func: EnvFunction::Constant(1.0),
            gate_off_func: EnvFunction::Constant(0.0),
            phase: EnvPhase::GateOff(0.0),
        }
    }
}

impl CommonEnvelope {
    pub fn new(gate_on_func: EnvFunction, gate_off_func: EnvFunction, slew_limition: f32) -> Self {
        Self {
            value: 0.0,
            prev_phase_last_value : 0.0,
            slew_limiter: SlewLimiter::new(-slew_limition.abs(), slew_limition.abs()),
            gate_on_func,
            gate_off_func,
            phase: EnvPhase::GateOff(0.0),
        }
    }
    #[inline]
    pub fn gate_on(&mut self) {
        self.prev_phase_last_value = self.value;
        self.phase = EnvPhase::GateOn(0.0);
    }
    #[inline]
    pub fn gate_off(&mut self) {
        self.prev_phase_last_value = self.value;
        self.phase = EnvPhase::GateOff(0.0);
    }

    #[inline]
    pub fn do_dt(&mut self, dt: f32, other_args: EnvFunctionArguments) -> f32 {
        self.value = self.slew_limiter.process(match self.phase {
            EnvPhase::GateOff(t) => {
                self.phase = EnvPhase::GateOff(t + dt);
                self.gate_off_func.eval(EnvFunctionArguments {
                    t,
                    prev_val: self.prev_phase_last_value,
                    ..other_args
                })
            }
            EnvPhase::GateOn(t) => {
                self.phase = EnvPhase::GateOn(t + dt);
                self.gate_on_func.eval(EnvFunctionArguments {
                    t,
                    prev_val: self.prev_phase_last_value,
                    ..other_args
                })
            }
        });
        self.value
    }
}
