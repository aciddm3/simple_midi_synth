#[derive(Debug, Clone)]
pub struct WaveformOscillator {
    pub func: Vec<f32>,
    pub phase_offset: f32,
    position: f32,
    current_value: f32,
}

impl WaveformOscillator {
    pub fn step(&mut self, step: f32) {
        self.position = (self.position + step).rem_euclid(1.0);
        self.current_value = func_from_vec(&self.func, self.position);
    }

    pub fn reset(&mut self) {
        self.position = self.phase_offset;
        self.current_value = func_from_vec(&self.func, self.position);
    }

    pub fn get_value(&self) -> f32 {
        self.current_value
    }

    pub fn new(func: Vec<f32>, phase_offset: f32) -> Self {
        WaveformOscillator {
            current_value: func_from_vec(&func, phase_offset),
            phase_offset,
            position: phase_offset,
            func,
        }
    }
}

#[doc = r"
A periodic function with period of 1 created from a Vec<f32>.
Interpolates values between array elements.
Takes any real number.
"]
#[inline]
pub fn pereodic_func_from_vec(table: &Vec<f32>, x: f32) -> f32 {
    let samples_count = table.len();
    if samples_count == 0 {
        return 0.0;
    }
    let vec_position_f = x.rem_euclid(1.0) * (samples_count - 1) as f32;
    let index_int = vec_position_f.floor() as usize;
    let index_float = vec_position_f.fract();

    if let Some(val) = table.get(index_int + 1) {
        table[index_int] + (val - table[index_int]) * index_float
    } else {
        table[index_int] + (table[0] - table[index_int]) * index_float
    }
}

#[doc = r"
A function created from a Vec<f32>.
Interpolates values between array elements.
Takes x from [0.0, 1.0] otherwise panics.
"]
#[inline]
pub fn func_from_vec(table: &Vec<f32>, x: f32) -> f32 {
    let samples_count = table.len();
    if samples_count == 0 {
        return 0.0;
    }
    let vec_position_f = x * (samples_count - 1) as f32;
    let index_int = vec_position_f.floor() as usize;
    let index_float = vec_position_f.fract();

    if index_int == samples_count - 1 {
        table[index_int]
    } else {
        table[index_int] + (table[index_int + 1] - table[index_int]) * index_float
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HarmonicParameters {
    pub amplitude: f32,
    pub phase: f32,
}

#[derive(Debug, Clone)]
pub struct HarmonicOscilator {
    sine_table: Vec<f32>,
    pub harmonic_seies: Vec<HarmonicParameters>,
    current_value: f32,
    position: f32,
    phase_offset: f32,
}

impl HarmonicOscilator {
    pub fn step(&mut self, step: f32) {
        self.position = (self.position + step).rem_euclid(1.0);
        self.current_value = value_from_harmonics(&self.harmonic_seies, &self.sine_table, self.position);
    }

    pub fn reset(&mut self) {
        self.position = self.phase_offset;
        self.current_value = value_from_harmonics(&self.harmonic_seies, &self.sine_table, self.position);
    }

    pub fn get_value(&self) -> f32 {
        self.current_value
    }

    pub fn new(
        harmonic_seies: Vec<HarmonicParameters>,
        phase_offset: f32,
        table_definition: usize,
    ) -> Self {
        let sine_table = (0..table_definition)
                .map(|s| {
                    ((s as f32 / table_definition as f32 - 0.5) * 2.0 * std::f32::consts::PI).sin()
                })
                .collect::<Vec<_>>();
        Self {
            current_value: value_from_harmonics(&harmonic_seies, &sine_table, phase_offset),
            phase_offset,
            position: phase_offset,
            harmonic_seies,
            sine_table,
        }
    }
}

#[inline]
pub fn value_from_harmonics(
    harmonic_seies: &Vec<HarmonicParameters>,
    func: &Vec<f32>,
    x: f32,
) -> f32 {
    let mut res = 0.0;
    for (harmonic_number, HarmonicParameters { amplitude, phase }) in
        harmonic_seies.iter().enumerate()
    {
        res += amplitude * pereodic_func_from_vec(func, harmonic_number as f32 * x + phase)
    }
    res
}
