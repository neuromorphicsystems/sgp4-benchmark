# SGP4 benchmark
A comparison of different SGP4 implementations:
- `cpp`: the Celestrak implementation [[1]](#1) in improved mode
- `cpp-afspc`: the Celestrak implementation [[1]](#1) in AFSPC mode
- `cpp-fastmath`: the Celestrak implementation [[1]](#1) in improved mode with the `fast-math` compiler flag
- `cpp-afspc-fastmath`: the Celestrak implementation [[1]](#1) in AFSPC mode with the `fast-math` compiler flag
- `rust`: our Rust implementation in improved mode
- `rust-afspc`: our Rust implementation in AFSPC mode

Our Rust implementation can be found at https://github.com/neuromorphicsystems/sgp4.

## Dependencies

- Rust - https://rustup.rs
- Meson - https://mesonbuild.com

## Accuracy

Accuracy calculates the maximum propagation error of each implementation with respect to the reference implementation (`cpp-afspc`) over the full catalogue (1 minute timestep over 24 hours).

Results are printed in the terminal.

To run the *accuracy* benchmark, use the command:
```
cargo run --release --bin accuracy
```

## Speed

Speed measures the time it takes to propagate the full catalogue (1 minute timestep over 24 hours) using 100 samples per implementation.

Results are saved in `results.json` upon completion (values are in μs).

To run the *speed* benchmark, use the command:
```
cargo run --release --bin accuracy
```

## Download

You can download a more recent satellite catalogue (*omms.json*) by running:
```
cargo run --release --bin download
```

This repository provides a catalogue downloaded on 2020-07-13. Keeping the same catalogue over time is required to evaluate the impact of implementation changes on accurracy and speed.

## References

<a id="1">[1]</a> David A. Vallado, Paul Crawford, R. S. Hujsak and T. S. Kelso, "Revisiting Spacetrack Report #3", presented at the AIAA/AAS Astrodynamics Specialist Conference, Keystone, CO, 2006 August 21–24, https://doi.org/10.2514/6.2006-6753
