use crate::simple_synth_parameters::SimpleSynthParams;
use crate::tembroblock::Tembroblock;
use crate::tembroblock::common_envelope::CommonEnvelope;
use crate::tembroblock::oscillator::SineOscillator;

use nih_plug_egui::EguiState;
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Debug, Default, Clone, Copy)]
pub struct ActiveNote {
    pub midi_note: u8,
    pub velocity_normalized : f32,
}

pub struct SimpleSynth {
    pub sine_table: Arc<Vec<f32>>,
    pub params: Arc<SimpleSynthParams>,
    pub active_note: Option<ActiveNote>,
    pub tembroblocks: Arc<RwLock<Vec<Tembroblock>>>,
    pub sample_rate: f32,
    pub dt : f32,
    pub master_freq: f32,
    pub editor_state: Arc<EguiState>,
    pub err_msg: Arc<RwLock<String>>,
}

impl Default for SimpleSynth {
    fn default() -> Self {
        let sine_table: Arc<Vec<f32>> = Arc::new(
            (0..=1024)
                .map(|s| (std::f32::consts::PI * s as f32 / 512.0).sin())
                .collect(),
        );
        Self {
            sine_table: sine_table.clone(),
            params: Arc::new(SimpleSynthParams::default()),
            active_note: None,
            sample_rate: 44100.0,
            dt : 1.0 / 44100.0,
            tembroblocks: Arc::new(RwLock::new(vec![Tembroblock::new(
                SineOscillator::new(sine_table.clone()),
                CommonEnvelope::default(),
                CommonEnvelope::default(),
                CommonEnvelope::default(),
            )])),
            master_freq: 440.0,
            editor_state: EguiState::from_size(600, 240),
            err_msg: Arc::new(RwLock::new("✔".to_string())),
        }
    }
}

/*impl ToString for ActiveNote {
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
*/
