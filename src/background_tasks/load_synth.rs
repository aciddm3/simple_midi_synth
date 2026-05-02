use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::tembroblock::common_envelope::CommonEnvelope;
use aciddm3_atom_func::parser::parse_func;

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

pub fn load_synth<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<(CommonEnvelope, CommonEnvelope, CommonEnvelope)>, Box<dyn Error>> {
    let content = fs::read_to_string(&path)?;

    let raw_presets: Vec<RawPreset> = serde_json::from_str(&content)?;

    let mut result = Vec::with_capacity(raw_presets.len());

    for raw in raw_presets {
        
        let pitch = CommonEnvelope::new(
            parse_func(&raw.pitch_gate_on)?,
            parse_func(&raw.pitch_gate_off)?,
            raw.pitch_slew_limit,
        );

        let phase = CommonEnvelope::new(
            parse_func(&raw.phase_gate_on)?,
            parse_func(&raw.phase_gate_off)?,
            raw.phase_slew_limit,
        );

        let amplitude = CommonEnvelope::new(
            parse_func(&raw.amplitude_gate_on)?,
            parse_func(&raw.amplitude_gate_off)?,
            raw.amplitude_slew_limit,
        );

        result.push((pitch, phase, amplitude));
    }

    Ok(result)
}
