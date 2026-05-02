
#[derive(Debug, Clone, Copy)]
pub struct SlewLimiter {
	max: f32,
    min: f32,
    curr_value: f32,
}

impl SlewLimiter {
	pub fn new(mut min: f32, mut max: f32) -> Self {
		if min > max {
			std::mem::swap(&mut min, &mut max);
        }
        Self {
            max,
            min,
            curr_value: 0.0,
        }
    }
	#[inline]
    pub fn process(&mut self, value: f32) -> f32 {
        self.curr_value += (value - self.curr_value).clamp(self.min, self.max);
        self.curr_value
    }
}
