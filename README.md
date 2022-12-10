# advent-2022

Rusty Advent of Code solutions — 2022

## Performance

On Apple M2 (MacBook Air M2, 2022):

```bash
❯ hyperfine -N --warmup 100 ./target/release/run-all
Benchmark 1: ./target/release/run-all
  Time (mean ± σ):       4.1 ms ±   0.1 ms    [User: 3.3 ms, System: 0.5 ms]
  Range (min … max):     3.9 ms …   5.0 ms    736 runs
```