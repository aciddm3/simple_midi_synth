use nih_plug::{params::*, prelude::*};
#[derive(Params)]
pub struct SimpleSynthParams {
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "transpose"]
    pub transpose: IntParam,
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
        }
    }
}
