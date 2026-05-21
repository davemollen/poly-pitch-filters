mod biquad_filters;
mod utils;
use crate::filter_bank::utils::{get_bandwidth, get_target_frequency};
use std::{hint::black_box, simd::StdFloat};
use {biquad_filters::BiquadFilters, std::array, std::simd::Simd};

const Z_LEFT: f32 = 1.5;
const Q_C: f32 = 4.;
const Q_ERB: f32 = 6.;
pub const NR_OF_BANDS: usize = 48;
pub const LANES: usize = 16;

pub struct FilterBank {
  filters: BiquadFilters,
}

impl FilterBank {
  pub fn new(sample_rate: f32) -> Self {
    let frequencies = array::from_fn(|i| get_target_frequency(Z_LEFT, i, Q_C, Q_ERB));
    let bandwidths = frequencies.map(|freq| get_bandwidth(freq, 1., Q_ERB, true));

    Self {
      filters: BiquadFilters::new(sample_rate, frequencies, bandwidths),
    }
  }

  pub fn reset(&mut self) {
    self.filters.reset();
  }

  #[inline(always)]
  pub fn process(&mut self, input: f32) {
    let mut i = 0;
    let simd_input = Simd::<f32, LANES>::splat(input);

    // Process in chunks of LANES for SIMD optimization
    while i + LANES <= NR_OF_BANDS {
      let (re, im) = self.filters.process_simd(simd_input, i);

      /*
      NOTE: The plugin normally processes the filter output here.
      For demonstration purposes, let's apply some heavy processing to simulate the workload.
      Using black_box to prevent the compiler from removing this redundant computation.
      */
      for _ in 0..32 {
        black_box((re * re + im * im).sqrt());
      }

      i += LANES;
    }

    // Handle remaining bands that don't fit into a full SIMD vector
    while i < NR_OF_BANDS {
      let (re, im) = self.filters.process_scalar(input, i);

      /*
      NOTE: The plugin normally processes the filter output here.
      For demonstration purposes, let's apply some heavy processing to simulate the workload.
      Using black_box to prevent the compiler from removing this redundant computation.
      */
      for _ in 0..32 {
        black_box((re * re + im * im).sqrt());
      }

      i += 1;
    }
  }
}
