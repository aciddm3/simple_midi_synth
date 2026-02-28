use nih_plug::{params::*, prelude::*};
#[derive(Params)]
pub struct SimpleSynthParams {
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "transpose"]
    pub transpose: IntParam,
    #[id = "amp_env_attack"]
    pub amp_adsr_attack: FloatParam,
    #[id = "amp_env_decay"]
    pub amp_adsr_decay: FloatParam,
    #[id = "amp_env_sustain"]
    pub amp_adsr_sustain: FloatParam,
    #[id = "amp_env_release"]
    pub amp_adsr_release: FloatParam,
    #[id = "osc_xfader"]
    pub osc_xfade: FloatParam,
}

impl Default for SimpleSynthParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                -10.0, // Дефолтное значение: -10 dB
                FloatRange::Linear {
                    min: -20.0,
                    max: 20.0,
                },
            )
            .with_unit("dB"),
            transpose: IntParam::new("Transpose", 0, IntRange::Linear { min: -48, max: 48 })
                .with_unit("st"),
            amp_adsr_attack: FloatParam::new(
                "Attack",
                0.5,
                FloatRange::Linear { min: 0.0, max: 8.0 },
            )
            .with_unit("sec"),
            amp_adsr_decay: FloatParam::new(
                "Decay",
                0.25,
                FloatRange::Linear { min: 0.0, max: 8.0 },
            )
            .with_unit("sec"),
            amp_adsr_sustain: FloatParam::new(
                "Sustain",
                100.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            )
            .with_unit("%"),
            amp_adsr_release: FloatParam::new(
                "Release",
                1.0,
                FloatRange::Linear { min: 0.0, max: 8.0 },
            )
            .with_unit("sec"),
            osc_xfade: FloatParam::new("Osc xfade", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
        }
    }
}
