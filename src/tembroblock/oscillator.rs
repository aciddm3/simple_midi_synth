use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SineOscillator {
    pub func: Arc<Vec<f32>>,
    position: f32,
    current_value: f32,
    curr_phase : f32,
}

impl SineOscillator {
    pub fn do_dt(&mut self, dt: f32) -> f32 {
        self.position = (self.position + dt).rem_euclid(1.0);
        self.current_value = func_from_vec(self.func.clone(), self.position);
        self.current_value
    }

    pub fn reset(&mut self) {
        self.position = 0.0;
        self.current_value = func_from_vec(self.func.clone(), self.position);
    }

    pub fn new(func: Arc<Vec<f32>>) -> Self {
        Self {
            current_value: func_from_vec(func.clone(), 0.0),
            position: 0.0,
            func,
            curr_phase : 0.0
        }
    }

    pub fn set_phase (&mut self, new_phase : f32) {
        self.position += new_phase - self.curr_phase;
        self.curr_phase = new_phase
    }
}



/// A function created from a Vec<f32>.
/// 
/// Interpolates values between array elements.
/// 
/// Takes x from [0.0, 1.0] otherwise panics.
/// 
/// # Examples
/// 
/// ``` rust
/// let x = 0.5;
/// assert_eq!(func_from_vec(vec![-1,1], x), 0.0);
/// ```
#[inline]
fn func_from_vec(table: Arc<Vec<f32>>, x: f32) -> f32 {
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



