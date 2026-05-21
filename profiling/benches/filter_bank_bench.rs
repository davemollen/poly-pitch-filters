#[path = "../src/utils.rs"]
mod utils;
use criterion::{Criterion, criterion_group, criterion_main};
use poly_pitch_dsp::FilterBank;
use utils::generate_signal_stream;

fn filter_bank_bench(c: &mut Criterion) {
  let mut filter_bank = FilterBank::new(44100.);
  let signal_stream = generate_signal_stream(44100);

  c.bench_function("filter_bank", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        filter_bank.process(*signal);
      }
    })
  });
}

criterion_group!(benches, filter_bank_bench);
criterion_main!(benches);
