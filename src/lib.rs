use aciddm3_atom_func::func::EnvFunctionArguments;
use nih_plug::prelude::*;
use std::sync::Arc;

use crate::background_tasks::TaskExec;

mod background_tasks;
mod gui;
mod simple_synth_parameters;
mod simple_synth_struct;
mod tembroblock;
mod utils;

impl Plugin for simple_synth_struct::SimpleSynth {
    type SysExMessage = ();
    type BackgroundTask = background_tasks::BackgroundTasks;
    const NAME: &'static str = "aciddm daleth";
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
        context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sine_table = Arc::new(
            (0..=1024)
                .map(|s| (std::f32::consts::TAU * s as f32 / 1024_f32).sin())
                .collect(),
        );
        self.sample_rate = buffer_config.sample_rate;
        self.dt = if self.sample_rate != 0.0 {
            1.0 / self.sample_rate
        } else {
            0.0
        };
        self.master_freq = 440.0;
        context.execute(background_tasks::BackgroundTasks::OpenFileNoDialog);

        self.tembroblocks.write().iter_mut().for_each(|s| s.reset());
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { note, velocity, .. } if velocity > 0.0 => {
                    self.active_note = Some(simple_synth_struct::ActiveNote {
                        midi_note: note,
                        velocity_normalized: velocity,
                    });
                    self.tembroblocks
                        .write()
                        .iter_mut()
                        .for_each(|s| s.gate_on());
                }
                NoteEvent::NoteOff { note, .. }
                    if let Some(active) = &self.active_note
                        && active.midi_note == note =>
                {
                    self.active_note = None;
                    self.tembroblocks
                        .write()
                        .iter_mut()
                        .for_each(|s| s.gate_off());
                }
                _ => (),
            }
        }

        if let Some(simple_synth_struct::ActiveNote { midi_note, .. }) = &mut self.active_note {
            self.master_freq = utils::note_to_freq(
                (*midi_note as i32 + self.params.transpose.value()) as f32,
                440.0,
            );
        };

        let num_samples = buffer.samples();

        for sample_index in 0..num_samples {
            let gain = util::db_to_gain(self.params.gain.smoothed.next());
            let mut out;
            {
                let mut tembroblocks_guard = self.tembroblocks.write();
                // полностью забираем себе tembroblocks
                out = tembroblocks_guard.iter().map(|s| s.get_value()).sum();
                out *= gain;
                tembroblocks_guard.iter_mut().for_each(|s| {
                    s.do_dt(
                        self.dt,
                        EnvFunctionArguments {
                            t: 0.0, // invariant
                            prev_val: 0.0,
                            freq: self.master_freq,
                            vel: self.active_note.unwrap_or_default().velocity_normalized,
                            p1: self.params.p1.smoothed.next(),
                            p2: self.params.p2.smoothed.next(),
                            p3: self.params.p3.smoothed.next(),
                            p4: self.params.p4.smoothed.next(),
                        },
                    );
                });
                // дропаем тут
            }

            for channels in buffer.as_slice() {
                channels[sample_index] = out
            }
        }

        ProcessStatus::Normal
    }
    fn editor(&mut self, async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        self.make_gui(async_executor)
    }

    fn task_executor(&mut self) -> TaskExecutor<Self> {
        self.task_exec()
    }
}

impl Vst3Plugin for simple_synth_struct::SimpleSynth {
    const VST3_CLASS_ID: [u8; 16] = [
        98, 218, 94, 45, 255, 44, 74, 204, 167, 126, 143, 79, 37, 188, 237, 20,
    ];
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Instrument];
}

nih_export_vst3!(simple_synth_struct::SimpleSynth);
