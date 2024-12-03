# Advent of Code 2024

My solutions to Advent of Code 2024 [https://adventofcode.com/2024].

Requires nightly toolchain: `rustup toolchain install nightly` and in this repo run `rustup override set nightly` so set nightly toolchain as default.

Test the implementations:

```bash
cargo test
```

To run individual days:

```bash
cargo run dayn inputfile
```
where dayn is the day, e.g. `day03` for day 3, and the input is either a .txt file or a string input, depending on the day.