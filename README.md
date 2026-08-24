# smart-pch-tsp-rs <sup>v0.1.0</sup>

[![Crates.io](https://img.shields.io/crates/v/smart-pch-tsp)](https://crates.io/crates/smart-pch-tsp)
[![Documentation](https://docs.rs/smart-pch-tsp/badge.svg)](https://docs.rs/smart-pch-tsp)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/smartlegionlab/smart-pch-tsp-rs)](https://github.com/smartlegionlab/smart-pch-tsp-rs)
[![GitHub forks](https://img.shields.io/github/forks/smartlegionlab/smart-pch-tsp-rs?style=social)](https://github.com/smartlegionlab/smart-pch-tsp-rs/network/members)

**Universal TSP path improver** using the PCH (Position-Candidate-Hypothesis) paradigm.

PCH takes ANY TSP path (from ANY algorithm), statistically analyzes it, and synthesizes a shorter path.

---

## Features

- **Universal Improver**: Takes ANY path — returns a shorter one
- **Generic Points**: Works with any type implementing the `Point` trait
- **Statistical Path Synthesis**: Uses transition frequency analysis
- **Parallel Processing**: Multi-agent system for faster computation
- **Better Quality**: Improves paths by 1-2% on average (up to 3% on some datasets)
- **No Dependencies**: Only uses Rust standard library

---

## Installation

```toml
[dependencies]
smart-pch-tsp = "0.1"
```

---

## Quick Start

```rust
use smart_pch_tsp::{City, pch_improve};

let points = vec![
    City { x: 0.0, y: 0.0 },
    City { x: 1.0, y: 0.0 },
    City { x: 0.0, y: 1.0 },
];

let initial_path = vec![0, 1, 2, 0];
let initial_distance = 4.0;

let (improved_distance, improved_path) = pch_improve(
    &points,
    &initial_path,
    initial_distance,
    50,    // permutations_per_hypothesis
    1000,  // batch_size
    30,    // time_limit (seconds)
    300,   // top_candidates
    12,    // num_agents
);

println!("Improved: {:.2} -> {:.2}", initial_distance, improved_distance);
```

---

## How It Works

PCH improves paths through statistical analysis:

1. **Accept ANY initial path** (from any algorithm)
2. **Build candidate framework** (top N candidates per position)
3. **Generate hypotheses** with random permutations
4. **Collect transition statistics** (weighted by improvement)
5. **Synthesize better path** from maximum-weight transitions

---

## Parameters

| Parameter                     | Type  | Default | Description                 |
|-------------------------------|-------|---------|-----------------------------|
| `permutations_per_hypothesis` | usize | 50      | Random swaps per hypothesis |
| `batch_size`                  | u64   | 1000    | Batch size for thread sync  |
| `time_limit`                  | u64   | 30      | Time limit in seconds       |
| `top_candidates`              | usize | 300     | Top candidates per position |
| `num_agents`                  | usize | 12      | Number of parallel agents   |

---

## Examples

### Basic Example: Improving Greedy Path

```bash
cargo run --example basic --release
```

Output:
```
================================================================================
PCH PATH IMPROVER - Improving Greedy Solution
================================================================================

Generating 1000 cities...

[1/2] GREEDY (Initial Solution)
   Distance: 27614.52
   Time: 0.010s
   Path GREEDY: 4 -> 562 -> 153 -> 747 -> 963 -> ... -> 580 -> 328 -> 939 -> 319 -> 4 (1001 total, closed)

[2/2] PCH IMPROVER
   Parameters:
     Permutations:        50
     Batch size:          1000
     Time limit:          5s (quick demo)
     Top candidates:      300
     Agents:              12
   PCH: 1.0s | Hypotheses: 0 | Permutations: 0 | Improvements: 0 | Records: 0 | Best: 27614.52 | Speed: 0/s
   PCH: 2.2s | Hypotheses: 12000 | Permutations: 598167 | Improvements: 12212 | Records: 2070807 | Best: 27153.72 | Speed: 598167/s
   PCH: 3.2s | Hypotheses: 12000 | Permutations: 598167 | Improvements: 12212 | Records: 2070807 | Best: 27153.72 | Speed: 0/s
   PCH: 4.2s | Hypotheses: 24000 | Permutations: 1196413 | Improvements: 12291 | Records: 2921633 | Best: 27093.87 | Speed: 598246/s
   PCH: 5.2s | Hypotheses: 24000 | Permutations: 1196413 | Improvements: 12291 | Records: 2921633 | Best: 27093.87 | Speed: 0/s

   Final Distance: 27058.28
   Time: 5.976s
   Path PCH IMPROVED: 568 -> 715 -> 237 -> 725 -> 94 -> ... -> 307 -> 861 -> 263 -> 834 -> 568 (1001 total, closed)

================================================================================
IMPROVEMENT SUMMARY
================================================================================
   Initial Distance:   27614.52
   Improved Distance:  27058.28

   Improvement: 556.24 (2.0%)
   Time: 0.010s vs 5.976s

   ✅ PCH SUCCESSFULLY IMPROVED THE PATH!
================================================================================
```

### Advanced Example: Improving Dynamic Gravity Path

```bash
cargo run --example improve_dg --release
```

Output:
```
================================================================================
PCH PATH IMPROVER - Improving Dynamic Gravity Solution
================================================================================

CONFIGURATION:
  Cities:            1000
  Seed phrase:       'SmartLegionLab_PCH_2026'

[1/2] DYNAMIC GRAVITY (Initial Solution)
   Distance: 13291.14
   Time: 0.012s
   Path DYNAMIC GRAVITY: 70 -> 918 -> 745 -> 213 -> 785 -> ... -> 133 -> 420 -> 680 -> 183 -> 70 (1001 total, closed)

[2/2] PCH IMPROVER
   Parameters:
     Permutations:        50
     Batch size:          1000
     Time limit:          30s
     Top candidates:      300
     Agents:              12
   PCH: 1.0s | Hypotheses: 0 | Permutations: 0 | Improvements: 0 | Records: 0 | Best: 13291.14 | Speed: 0/s
   PCH: 2.1s | Hypotheses: 12000 | Permutations: 598167 | Improvements: 12015 | Records: 1393619 | Best: 13266.29 | Speed: 598167/s
   PCH: 3.1s | Hypotheses: 12000 | Permutations: 598167 | Improvements: 12015 | Records: 1393619 | Best: 13266.29 | Speed: 0/s
   PCH: 4.1s | Hypotheses: 24000 | Permutations: 1196427 | Improvements: 12026 | Records: 1818344 | Best: 13225.24 | Speed: 598260/s
   PCH: 5.1s | Hypotheses: 24000 | Permutations: 1196427 | Improvements: 12026 | Records: 1818344 | Best: 13225.24 | Speed: 0/s
   PCH: 6.1s | Hypotheses: 36000 | Permutations: 1794637 | Improvements: 12044 | Records: 1953896 | Best: 13223.71 | Speed: 598210/s
   PCH: 7.1s | Hypotheses: 36000 | Permutations: 1794637 | Improvements: 12044 | Records: 1953896 | Best: 13223.71 | Speed: 0/s
   PCH: 8.1s | Hypotheses: 48000 | Permutations: 2392870 | Improvements: 12066 | Records: 2005790 | Best: 13222.53 | Speed: 598233/s
   PCH: 9.1s | Hypotheses: 48000 | Permutations: 2392870 | Improvements: 12066 | Records: 2005790 | Best: 13222.53 | Speed: 0/s
   PCH: 10.1s | Hypotheses: 60000 | Permutations: 2991057 | Improvements: 12083 | Records: 2030773 | Best: 13216.54 | Speed: 598187/s
   PCH: 11.1s | Hypotheses: 60000 | Permutations: 2991057 | Improvements: 12083 | Records: 2030773 | Best: 13216.54 | Speed: 0/s
   PCH: 12.1s | Hypotheses: 72000 | Permutations: 3589243 | Improvements: 12097 | Records: 2047311 | Best: 13198.29 | Speed: 598186/s
   PCH: 13.1s | Hypotheses: 74000 | Permutations: 3688953 | Improvements: 12099 | Records: 2048943 | Best: 13198.29 | Speed: 99710/s
   PCH: 14.1s | Hypotheses: 84000 | Permutations: 4187466 | Improvements: 12112 | Records: 2060000 | Best: 13164.83 | Speed: 498513/s
   PCH: 15.1s | Hypotheses: 90000 | Permutations: 4486602 | Improvements: 12118 | Records: 2065234 | Best: 13164.83 | Speed: 299136/s
   PCH: 16.1s | Hypotheses: 96000 | Permutations: 4785711 | Improvements: 12124 | Records: 2071409 | Best: 13164.83 | Speed: 299109/s
   PCH: 17.1s | Hypotheses: 105000 | Permutations: 5234318 | Improvements: 12132 | Records: 2078596 | Best: 13164.83 | Speed: 448607/s
   PCH: 18.1s | Hypotheses: 108000 | Permutations: 5383879 | Improvements: 12134 | Records: 2080989 | Best: 13164.83 | Speed: 149561/s
   PCH: 19.1s | Hypotheses: 120000 | Permutations: 5982094 | Improvements: 12146 | Records: 2089610 | Best: 13162.07 | Speed: 598215/s
   PCH: 20.1s | Hypotheses: 120000 | Permutations: 5982094 | Improvements: 12146 | Records: 2089610 | Best: 13162.07 | Speed: 0/s
   PCH: 21.1s | Hypotheses: 132000 | Permutations: 6580289 | Improvements: 12152 | Records: 2096881 | Best: 13162.07 | Speed: 598195/s
   PCH: 22.1s | Hypotheses: 132000 | Permutations: 6580289 | Improvements: 12152 | Records: 2096881 | Best: 13162.07 | Speed: 0/s
   PCH: 23.1s | Hypotheses: 144000 | Permutations: 7178483 | Improvements: 12162 | Records: 2103178 | Best: 13162.07 | Speed: 598194/s
   PCH: 24.1s | Hypotheses: 144000 | Permutations: 7178483 | Improvements: 12162 | Records: 2103178 | Best: 13162.07 | Speed: 0/s
   PCH: 25.1s | Hypotheses: 156000 | Permutations: 7776709 | Improvements: 12166 | Records: 2108590 | Best: 13162.07 | Speed: 598226/s
   PCH: 26.1s | Hypotheses: 157000 | Permutations: 7826557 | Improvements: 12166 | Records: 2108747 | Best: 13162.07 | Speed: 49848/s
   PCH: 27.1s | Hypotheses: 168000 | Permutations: 8374949 | Improvements: 12176 | Records: 2113216 | Best: 13162.07 | Speed: 548392/s
   PCH: 28.1s | Hypotheses: 169000 | Permutations: 8424801 | Improvements: 12177 | Records: 2113426 | Best: 13162.07 | Speed: 49852/s
   PCH: 29.1s | Hypotheses: 180000 | Permutations: 8973106 | Improvements: 12181 | Records: 2117001 | Best: 13162.07 | Speed: 548305/s
   PCH: 30.1s | Hypotheses: 182000 | Permutations: 9072833 | Improvements: 12182 | Records: 2117547 | Best: 13162.07 | Speed: 99727/s

   Final Distance: 13162.07
   Time: 31.257s
   Path PCH IMPROVED: 70 -> 918 -> 745 -> 213 -> 785 -> ... -> 133 -> 420 -> 680 -> 183 -> 70 (1001 total, closed)

================================================================================
IMPROVEMENT SUMMARY
================================================================================
   Initial Distance:   13291.14
   Improved Distance:  13162.07

   Improvement: 129.07 (1.0%)
   Time: 0.012s vs 31.257s

   ✅ PCH SUCCESSFULLY IMPROVED THE PATH!
================================================================================
```

### Benchmarks

```bash
cargo bench
```

---

## Performance

| Algorithm           | Distance          | Time    |
|---------------------|-------------------|---------|
| **Dynamic Gravity** | 13,291.14         | 0.011s  |
| **PCH Improved**    | **13,162.07**     | 31.653s |
| **Improvement**     | **129.07 (1.0%)** | —       |

---

## Ecosystem

This library is part of the **NP Problem Ecosystem** - a comprehensive suite of exact and heuristic solvers for the Traveling Salesman Problem:

| Project                                                                                         | Description                                            | Language |
|-------------------------------------------------------------------------------------------------|--------------------------------------------------------|----------|
| **[Exact TSP Solver](https://github.com/smartlegionlab/exact-tsp-solver)**                      | High-performance exact solver using Branch and Bound   | Go       |
| **[Smart TSP Oracle](https://github.com/smartlegionlab/smart-tsp-oracle)**                      | Exact solver with adaptive thresholding                | Python   |
| **[Smart TSP Solver](https://github.com/smartlegionlab/smart-tsp-solver)**                      | Heuristic solver with Angular-Radial & Dynamic Gravity | Python   |
| **[Smart TSP Benchmark](https://github.com/smartlegionlab/smart-tsp-benchmark)**                | Professional testing infrastructure                    | Python   |
| **[smart-dynamic-gravity-tsp](https://github.com/smartlegionlab/smart-dynamic-gravity-tsp-rs)** | Fast physics-inspired TSP solver                       | Rust     |
| **smart-pch-tsp**                                                                               | Universal PCH path improver                            | Rust     |

All projects are grounded in the **Position-Candidate-Hypothesis (PCH)** paradigm for NP-complete problems.

---

## Development

```bash
# Clone repository
git clone https://github.com/smartlegionlab/smart-pch-tsp-rs
cd smart-pch-tsp

# Build
cargo build

# Run tests
cargo test

# Run documentation tests (checks code examples in docs)
cargo test --doc

# Run examples
cargo run --example basic --release
cargo run --example improve_dg --release

# Run benchmarks
cargo bench

# Build documentation
cargo doc --open
```

---

## License

[BSD 3-Clause License](LICENSE)

Copyright © 2026, [Alexander Suvorov](https://github.com/smartlegionlab)

---

## Author

**Alexander Suvorov**

- GitHub: [smartlegionlab](https://github.com/smartlegionlab)
- Website: [smartlegionlab.com](https://smartlegionlab.com)

---

## ⚠️ Disclaimer

**By using this software, you agree to the full disclaimer terms.**

**Summary:** Software provided "AS IS" without warranty. You assume all risks.

**Full legal disclaimer:** See [DISCLAIMER.md](https://github.com/smartlegionlab/smart-pch-tsp-rs/blob/master/DISCLAIMER.md)

