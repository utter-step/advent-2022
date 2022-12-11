# advent-2022

Rusty Advent of Code solutions — 2022

## Performance

On Apple M2 (MacBook Air M2, 2022):

```bash
❯ hyperfine -N --warmup 100 ./target/release/run-all
Benchmark 1: ./target/release/run-all
  Time (mean ± σ):      10.6 ms ±   0.1 ms    [User: 9.7 ms, System: 0.5 ms]
  Range (min … max):    10.3 ms …  11.0 ms    280 runs
```
