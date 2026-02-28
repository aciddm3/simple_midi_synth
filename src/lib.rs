use nih_plug::prelude::*;
use std::sync::Arc;

use crate::{adsr::Adsr, oscillator::WaveformOscillator};
mod adsr;
mod oscillator;
mod utils;
mod simple_synth_struct;
mod simple_synth_parameters;
mod gui;


impl Plugin for simple_synth_struct::SimpleSynth {
    type SysExMessage = ();
    type BackgroundTask = ();
    const NAME: &'static str = "Simple Monophonic Synth";
    const VENDOR: &'static str = "Gemma";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = env!("CARGO_PKG_AUTHORS");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::Basic;

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_output_channels: NonZeroU32::new(2),
            main_input_channels: NonZeroU32::new(0),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_output_channels: NonZeroU32::new(1),
            main_input_channels: NonZeroU32::new(0),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default(),
        },
    ];
    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        self.osc_sin = WaveformOscillator::new(
            (-512..=512)
                .map(|s| (s as f32 * std::f32::consts::PI / 512.0).sin())
                .collect::<Vec<_>>(),
            0.5,
        );
        self.osc_saw = WaveformOscillator::new(vec![-1.0, 1.0], 0.5);
        self.amp_env = Adsr::new(0.5, 0.25, 1.0, 1.0);
        self.osc_freq = 440.0;
        self.amp_env.reset();
        self.osc_sin.reset();
        self.osc_saw.reset();
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers, // aux не используется
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { note, velocity, .. } => {
                    if velocity > 0.0 {
                        self.active_note = Some(simple_synth_struct::ActiveNote { midi_note: note });
                        self.amp_env.gate_on();
                    }
                }
                NoteEvent::NoteOff { note, .. } => {
                    if let Some(active) = &self.active_note {
                        if active.midi_note == note {
                            self.active_note = None;
                            self.amp_env.gate_off();
                        }
                    }
                }
                _ => (),
            }
        }

        if let Some(simple_synth_struct::ActiveNote { midi_note }) = &mut self.active_note {
            self.osc_freq = utils::note_to_freq(
                (*midi_note as i32 + self.params.transpose.value()) as f32,
                440.0,
            );
        };

        let num_samples = buffer.samples();

        for sample_index in 0..num_samples {
            let x_fader_ratio = self.params.osc_xfade.smoothed.next();
            let gain = util::db_to_gain(self.params.gain.smoothed.next());
            for channels in buffer.as_slice() {
                channels[sample_index] = utils::xfader(
                    self.osc_sin.get_value(),
                    self.osc_saw.get_value(),
                    x_fader_ratio,
                ) * self.amp_env.get_value()
                    * gain;
            }
            self.amp_env.attack = self.params.amp_adsr_attack.smoothed.next();
            self.amp_env.decay = self.params.amp_adsr_decay.smoothed.next();
            self.amp_env.sustain = self.params.amp_adsr_sustain.smoothed.next() / 100.0;
            self.amp_env.release = self.params.amp_adsr_release.smoothed.next();

            self.amp_env.step(1.0 / self.sample_rate);
            self.osc_sin.step(self.osc_freq / self.sample_rate);
            self.osc_saw.step(self.osc_freq / self.sample_rate);
        }

        ProcessStatus::Normal
    }
    fn editor(&mut self, async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        self.make_gui(async_executor)
    }
}

impl Vst3Plugin for simple_synth_struct::SimpleSynth {
    const VST3_CLASS_ID: [u8; 16] = [
        98, 218, 94, 45, 78, 44, 74, 204, 167, 126, 143, 79, 37, 188, 237, 20,
    ];
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Instrument];
}

nih_export_vst3!(simple_synth_struct::SimpleSynth);
