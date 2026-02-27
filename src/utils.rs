#[inline]
pub fn note_to_freq(note: f32, a4_base: f32) -> f32 {
    2f32.powf((note - 69.0) / 12.0) * a4_base
}

#[inline]
pub fn xfader (a : f32, b : f32, ratio : f32) -> f32 {
	let angle = ratio * std::f32::consts::FRAC_PI_2;
	a * angle.cos() + b * angle.sin()
}