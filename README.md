# advent-2022

Rusty Advent of Code solutions — 2022

## Performance

On Apple M2 (MacBook Air M2, 2022):

```shell
❯ hyperfine ./target/release/run-all -N --warmup 100
Benchmark 1: ./target/release/run-all
  Time (mean ± σ):       3.7 ms ±   0.1 ms    [User: 3.0 ms, System: 0.4 ms]
  Range (min … max):     3.5 ms …   4.6 ms    782 runs
```