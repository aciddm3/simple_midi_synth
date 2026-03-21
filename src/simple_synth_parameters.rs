use nih_plug::{params::*, prelude::*};

use parking_lot::RwLock;
#[derive(Params)]
pub struct SimpleSynthParams {
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "transpose"]
    pub transpose: IntParam,
    #[persist = "file_path"]
    pub file_path: RwLock<Option<String>>,
}

impl Default for SimpleSynthParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                -10.0,
                FloatRange::Linear {
                    min: -20.0,
                    max: 20.0,
                },
            )
            .with_unit("dB"),
            transpose: IntParam::new("Transpose", 0, IntRange::Linear { min: -48, max: 48 })
                .with_unit("st"),
            file_path: RwLock::new(Some("".to_string())),
        }
    }
}
