use std::{error::Error, fs};

use serde_json::{Value, from_str};

use crate::{adsr::Adsr, oscillator::SineOscillator};

pub fn load_synth(
    path: &String,
    func: std::sync::Arc<Vec<f32>>,
) -> Result<(Vec<SineOscillator>, Vec<Adsr>), Box<dyn Error>> {
    let string = fs::read_to_string(path)?;
    match from_str::<Value>(string.as_str()) {
        Err(e) => {
            Err(Box::new(e))
        }
        Ok(val) => {
            let Some(arr) = val.as_array() else {
                eprintln!("Incorrect JSON");
                return Ok((
                    vec![SineOscillator::new(func.clone(), 0.0); 2],
                    vec![Adsr::new(0.0, 0.0, 1.0, 0.0); 2],
                ));
            };
            let size = arr.len();
            let mut res_amp = Vec::with_capacity(size);
            for val in arr {
                let adsr = Adsr::new(
                    val["attack"].as_f64().unwrap_or(1.0) as f32,
                    val["decay"].as_f64().unwrap_or(1.0) as f32,
                    val["sustain"].as_f64().unwrap_or(1.0) as f32,
                    val["release"].as_f64().unwrap_or(1.0) as f32,
                );
                res_amp.push(adsr);
            }
            Ok((vec![SineOscillator::new(func.clone(), 0.0); size], res_amp))
        }
    }
}
