use nih_plug::plugin::{Plugin, TaskExecutor};

use crate::{
    background_tasks::load_synth::load_synth,
    simple_synth_struct::SimpleSynth,
    tembroblock::{Tembroblock, common_envelope::*, oscillator::*},
};

mod load_synth;

pub enum BackgroundTasks {
    OpenFileDialog,
    OpenFileNoDialog,
}

pub trait TaskExec: Plugin {
    fn task_exec(&mut self) -> TaskExecutor<Self>;
}

impl TaskExec for SimpleSynth {
    fn task_exec(&mut self) -> TaskExecutor<Self> {
        let params = self.params.clone();
        let tembroblocks_lock = self.tembroblocks.clone();
        let sine_table = self.sine_table.clone();

        Box::new(move |task| match task {
            BackgroundTasks::OpenFileNoDialog => {
                let path_opt = params.file_path.read().clone();
                if let Some(path_str) = path_opt
                    && !path_str.is_empty()
                {
                    match load_synth::load_synth(&path_str) {
                        Ok(vec) => {
                            tembroblocks_lock.write().clear();
                            for (pitch_envelope, phase_envelope, amplitude_envelope) in vec {
                                tembroblocks_lock.write().push(Tembroblock::new(
                                    SineOscillator::new(sine_table.clone()),
                                    amplitude_envelope,
                                    phase_envelope,
                                    pitch_envelope,
                                ));
                            }
                        }
                        Err(e) => {
                            eprintln!("{e}");
                        }
                    }
                } else {
                    tembroblocks_lock.write().clear();
                    tembroblocks_lock.write().push(Tembroblock::new(
                        SineOscillator::new(sine_table.clone()),
                        CommonEnvelope::default(),
                        CommonEnvelope::default(),
                        CommonEnvelope::default(),
                    ));
                }
            }
            BackgroundTasks::OpenFileDialog => {
                let Some(path) = rfd::FileDialog::new()
                    .add_filter("json", &["json"])
                    .pick_file()
                else {
                    eprintln!("Couldn\'t open file dialog");
                    return;
                };

                let path = path.into_os_string().into_string().unwrap_or_default();

                match load_synth(&path) {
                    Ok(vec) => {
                        tembroblocks_lock.write().clear();
                        for (pitch_envelope, phase_envelope, amplitude_envelope) in vec {
                            tembroblocks_lock.write().push(Tembroblock::new(
                                SineOscillator::new(sine_table.clone()),
                                amplitude_envelope,
                                phase_envelope,
                                pitch_envelope,
                            ));
                        }
                    }
                    Err(e) => {
                        eprintln!("{e}");
                        if params.file_path.read().is_none() {
                            tembroblocks_lock.write().clear();
                            tembroblocks_lock.write().push(Tembroblock::new(
                                SineOscillator::new(sine_table.clone()),
                                CommonEnvelope::default(),
                                CommonEnvelope::default(),
                                CommonEnvelope::default(),
                            ));
                        }
                        return;
                    }
                }
                *params.file_path.write() = Some(path);
            }
        })
    }
}
