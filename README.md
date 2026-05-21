## dm-PolyPitch

<img src="screenshot.png" alt="plugin screenshot" style="display:block;width:60%;max-width:840px;">

This is just a small part of the code of the PolyPitch plugin. Soon to be released.

### Code

The partial plugin code can be found in the [dsp](/dsp) directory.

### Benchmarks

You can run the filterbank benchmark by going into the [profiling](/profiling) directory.

Then you can run `sudo cargo bench -- filter_bank`.

Comment out the SIMD code parts [here](dsp/src/filter_bank.rs) to see how that impacts the performance when you run the benchmark again.
