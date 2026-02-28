use crate::simple_synth_parameters::SimpleSynthParams;
use crate::{adsr::*, oscillator::*};
use nih_plug_egui::EguiState;
use std::sync::Arc;
pub struct ActiveNote {
    pub midi_note: u8,
}

pub struct SimpleSynth {
    pub params: Arc<SimpleSynthParams>,
    pub active_note: Option<ActiveNote>,
    pub sample_rate: f32,
    pub amp_env: Adsr,
    pub osc_sin: WaveformOscillator,
    pub osc_saw: WaveformOscillator,
    pub osc_freq: f32,
    pub editor_state: Arc<EguiState>,
}

impl Default for SimpleSynth {
    fn default() -> Self {
        Self {
            params: Arc::new(SimpleSynthParams::default()),
            active_note: None,
            sample_rate: 44100.0,
            amp_env: Adsr::new(0.5, 0.25, 1.0, 1.0),
            osc_sin: WaveformOscillator::new(
                (-512..=512)
                    .map(|s| (std::f32::consts::PI * s as f32 / 512.0).sin())
                    .collect(),
                0.5,
            ),
            osc_saw: WaveformOscillator::new(vec![-1.0, 1.0], 0.5),
            osc_freq: 440.0,
            editor_state: EguiState::from_size(600, 200),
        }
    }
}

impl ToString for ActiveNote {
    fn to_string(&self) -> String {
        let (note, oct) = (self.midi_note % 12, self.midi_note / 12);
        let note = match note {
            0 => "C",
            1 => "C#",
            2 => "D",
            3 => "D#",
            4 => "E",
            5 => "F",
            6 => "F#",
            7 => "G",
            8 => "G#",
            9 => "A",
            10 => "A#",
            11 => "B",
            _ => "Err",
        }
        .to_string();
        format!("{note}{}", oct as i8 - 1)
    }
}
