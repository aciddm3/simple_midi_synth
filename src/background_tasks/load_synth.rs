use aciddm3_atom_func::parser::error::ParseError;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::tembroblock::common_envelope::CommonEnvelope;
use aciddm3_atom_func::parser::parse_func;

const SIMPLE_DEPTH: usize = 256;

#[derive(Deserialize)]
struct RawPreset {
    pitch_gate_on: String,
    pitch_gate_off: String,
    pitch_slew_limit: f32,

    phase_gate_on: String,
    phase_gate_off: String,
    phase_slew_limit: f32,

    amplitude_gate_on: String,
    amplitude_gate_off: String,
    amplitude_slew_limit: f32,
}

fn load_raw_preset(
    raw: RawPreset,
) -> Result<(CommonEnvelope, CommonEnvelope, CommonEnvelope), ParseError> {
    let pitch = CommonEnvelope::new(
        {
            let mut res = parse_func(&raw.pitch_gate_on)?;
            res.simplify::<SIMPLE_DEPTH>();
            res
        },
        {
            let mut res = parse_func(&raw.pitch_gate_off)?;
            res.simplify::<SIMPLE_DEPTH>();
            res
        },
        raw.pitch_slew_limit,
    );

    let phase = CommonEnvelope::new(
        {
            let mut res = parse_func(&raw.phase_gate_on)?;
            res.simplify::<SIMPLE_DEPTH>();
            res
        },
        {
            let mut res = parse_func(&raw.phase_gate_off)?;
            res.simplify::<SIMPLE_DEPTH>();
            res
        },
        raw.phase_slew_limit,
    );

    let amplitude = CommonEnvelope::new(
        {
            let mut res = parse_func(&raw.amplitude_gate_on)?;
            res.simplify::<SIMPLE_DEPTH>();
            res
        },
        {
            let mut res = parse_func(&raw.amplitude_gate_off)?;
            res.simplify::<SIMPLE_DEPTH>();
            res
        },
        raw.amplitude_slew_limit,
    );
    Ok((pitch, phase, amplitude))
}

pub fn load_synth<P: AsRef<Path>>(
    path: P,
    error_msg: &mut String,
) -> Result<Vec<(CommonEnvelope, CommonEnvelope, CommonEnvelope)>, Box<dyn Error>> {
    let content = fs::read_to_string(&path)?;

    let raw_presets: Vec<RawPreset> = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            *error_msg = e.to_string();
            return Err(Box::new(e));
        }
    };

    let mut result = Vec::with_capacity(raw_presets.len());

    for (index, raw) in raw_presets.into_iter().enumerate() {
        match load_raw_preset(raw) {
            Ok(preset) => result.push(preset),
            Err(e) => {
                *error_msg = format!("in tb № {index} {e}");
                return Err(Box::new(e));
            }
        }
    }

    *error_msg = "✔".to_string();
    Ok(result)
}
