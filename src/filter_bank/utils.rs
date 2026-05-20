/// The ERB critical bandwidth (in Hz) given as a function of its center frequency
pub fn erb(freq: f32) -> f32 {
  24.7 + 0.108 * freq
}

// Get the bandwidth (in Hz) for a given frequency and pitchshift amount
pub fn get_bandwidth(freq: f32, pitchshift: f32, q_erb: f32, is_phase_scaling: bool) -> f32 {
  let q_ps = if is_phase_scaling { pitchshift } else { 1. };
  (q_ps * q_erb).recip() * erb(freq * pitchshift)
}

/// Convert ERB to Hz scale
pub fn erb_to_hz(z: f32) -> f32 {
  228.7 * (10_f32.powf(z / 21.3) - 1.)
}

/// Get a filter frequency based on the uniform distribution of the ERB scale
pub fn get_target_frequency(z_left: f32, index: usize, q_c: f32, q_erb: f32) -> f32 {
  erb_to_hz(z_left + (index as f32 * q_c / q_erb))
}
