mod utils;
use poly_pitch_dsp::FilterBank;
use utils::generate_signal;

fn main() {
  let mut filter_bank = FilterBank::new(44100.);

  loop {
    let input = generate_signal();
    filter_bank.process(input);
  }
}
