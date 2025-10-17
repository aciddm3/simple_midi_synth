use nih_plug::prelude::*;
use std::sync::Arc;


struct ActiveNote {
    /// MIDI-номер ноты (например, 60 для C5)
    midi_note: u8,
    /// Текущая позиция в семпле (используем f64 для точности)
    current_sample_pos: f32,
    /// Шаг приращения за один семпл (определяет высоту тона)
    /// rate = (freq_ноты / sample_rate)
    sample_rate_ratio: f32,
}

struct SimpleSampler {
    params: Arc<SimpleSamplerParams>,
    /// Аудио данные нашего "семпла"
    sample_data: Vec<f32>,
    /// Единственная активная нота (для монофонического семпла)
    active_note: Option<ActiveNote>,
    sample_rate: f32,
}

impl Default for SimpleSampler {
    fn default() -> Self {
        Self {
            params: Arc::new(SimpleSamplerParams::default()),
            sample_data: Vec::new(),
            active_note: None,
            sample_rate: 44100.0,
        }
    }
}

#[derive(Params)]
struct SimpleSamplerParams {
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for SimpleSamplerParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(-10.0), // Дефолтное значение: -10 dB
                FloatRange::Linear {
                    min: util::db_to_gain(-20.0),
                    max: util::db_to_gain(20.0),
                },
            )
            .with_unit(" dB")
            .with_value_to_string(Arc::new(move |v| format!("{:.1}", util::gain_to_db(v))))
            .with_string_to_value(Arc::new(move |s| s.parse().ok().map(util::db_to_gain))),
        }
    }
}

impl SimpleSampler {
    /// Генерирует простой зацикленный синус на 1 Гц
    fn generate_default_sample(sample_rate: f32) -> Vec<f32> {
        (0..sample_rate as usize)
            .map(|i| {
                let t = i as f32 / sample_rate;
                (std::f32::consts::TAU * t).sin()
            })
            .collect()
    }
}

impl Plugin for SimpleSampler {
    // --- Метаданные ---
    type SysExMessage = ();
    type BackgroundTask = ();
    const NAME: &'static str = "Simple MIDI Sampler";
    const VENDOR: &'static str = "My Awesome Dev";
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
        self.sample_data = Self::generate_default_sample(self.sample_rate);
        println!("Simple Sample Init...");
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers, // aux не используется
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let gain = self.params.gain.smoothed.next();
        let num_samples = buffer.samples(); // Оптимизация

        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { note, velocity, .. } => {
                    if velocity > 0.0 {
                        // ✅ ИСПРАВЛЕНИЕ: Приведение f64 к f32 для Pitch Ratio (Устраняет тишину/неверный тон)
                        let sample_rate_ratio = util::midi_note_to_freq(note) as f32;

                        self.active_note = Some(ActiveNote {
                            midi_note: note,
                            current_sample_pos: 0.0,
                            sample_rate_ratio,
                        })
                    }
                    println!("NoteOn:\t note: {note}\tvel: {velocity}");
                }
                NoteEvent::NoteOff { note, .. } => {
                    if let Some(active) = &self.active_note {
                        if active.midi_note == note {
                            self.active_note = None;
                        }
                    }
                    println!("NoteOff:\tnote: {note}");
                }
                _ => {println!("Other MIDI event")},
            }
        }

        if let Some(note) = &mut self.active_note {
            let sample_len = self.sample_data.len();

            if sample_len == 0 {
                // ✅ ИСПРАВЛЕНИЕ: Используем buffer.as_slice_mut() для записи тишины
                for channel_buffer in buffer.as_slice() {
                    channel_buffer.fill(0.0);
                }
                return ProcessStatus::Normal;
            }

            for sample_idx in 0..num_samples {
                let read_idx = (note.current_sample_pos.floor() as usize) % sample_len;
                let sample_val = self.sample_data[read_idx];
                let output_sample = sample_val * gain;

                // ✅ ИСПРАВЛЕНИЕ: Запись в каналы с помощью buffer.as_slice_mut()
                for channel_buffer in buffer.as_slice() {
                    channel_buffer[sample_idx] = output_sample;
                }

                note.current_sample_pos += note.sample_rate_ratio;

                if note.current_sample_pos >= sample_len as f32 {
                    note.current_sample_pos -= sample_len as f32;
                }
            }
        } else {
            // ✅ ИСПРАВЛЕНИЕ: Заполнение тишиной, если нот нет
            for channel_buffer in buffer.as_slice() {
                channel_buffer.fill(0.0);
            }
        }

        ProcessStatus::Normal
    }
}

// ✅ ИСПРАВЛЕНИЕ: Обновление VST3 констант до современного API
impl Vst3Plugin for SimpleSampler {
    // В новейшей версии VST3Plugin не требует Processor/Controller CID, 
    // а требует один CLASS_ID.
    const VST3_CLASS_ID: [u8; 16] = [
        98, 218, 94, 45, 78, 44, 74, 204, 167, 126, 143, 79, 37, 188, 237, 20,
    ]; 
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Instrument];
}

nih_export_vst3!(SimpleSampler);