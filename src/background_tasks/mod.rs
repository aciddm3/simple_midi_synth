use nih_plug::plugin::{Plugin, TaskExecutor};

use crate::{
    adsr::Adsr,
    background_tasks::load_synth::load_synth,
    oscillator::SineOscillator,
    simple_synth_struct::SimpleSynth,
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
        let osc_lock = self.osc.clone();
        let env_lock = self.amp_env.clone();
        let sine_table = self.sine_table.clone();
        Box::new(move |task| match task {
            BackgroundTasks::OpenFileNoDialog => {
                let path_opt = params.file_path.read().clone();
                if let Some(path_str) = path_opt
                    && !path_str.is_empty()
                {
                    match load_synth::load_synth(&path_str, sine_table.clone()) {
                        Ok((oscs, envs)) => {
                            *osc_lock.write() = oscs;
                            *env_lock.write() = envs;
                        }
                        Err(e) => {
                            eprintln!("{e}");
                        }
                    }
                } else {
                    *osc_lock.write() = vec![SineOscillator::new(sine_table.clone(), 0.0); 2];
                    *env_lock.write() = vec![Adsr::new(0.0, 0.0, 1.0, 0.0); 2];
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

                match load_synth(&path, sine_table.clone()) {
                    Ok((oscs, envs)) => {
                        *osc_lock.write() = oscs;
                        *env_lock.write() = envs;
                    }
                    Err(e) => {
                        eprintln!("{e}");
                        if let None = *params.file_path.read() {
                            *osc_lock.write() =
                                vec![SineOscillator::new(sine_table.clone(), 0.0); 2];
                            *env_lock.write() = vec![Adsr::new(0.0, 0.0, 1.0, 0.0); 2];
                        }
                        return;
                    }
                }
                *params.file_path.write() = Some(path);
            }
        })
    }
}
