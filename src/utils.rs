#[inline]
pub fn note_to_freq(note: f32, a4_base: f32) -> f32 {
    2f32.powf((note - 69.0) / 12.0) * a4_base
}

