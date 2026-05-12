use nih_plug::{params::*, prelude::*};

use parking_lot::RwLock;
#[derive(Params)]
pub struct SimpleSynthParams {
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "transpose"]
    pub transpose: IntParam,
    #[id = "parameter1"]
    pub p1 : FloatParam,
    #[id = "parameter2"]
    pub p2 : FloatParam,
    #[id = "parameter3"]
    pub p3 : FloatParam,
    #[id = "parameter4"]
    pub p4 : FloatParam,
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
            p1 : FloatParam::new("Param1", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
            p2 : FloatParam::new("Param2", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
            p3 : FloatParam::new("Param3", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
            p4 : FloatParam::new("Param4", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
        }
    }
}
