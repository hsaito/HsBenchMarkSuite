# HsBenchMarkSuite

A high-performance Rust benchmark suite for measuring CPU, memory, and disk performance.

## Disclaimer

**This benchmark suite provides runtime metrics and performance measurements for specific test scenarios.** The results do **not necessarily equate to actual system capability** or real-world performance for your specific use cases. These benchmarks:

- Measure synthetic workloads under controlled conditions
- Test specific algorithms and data patterns (prime calculation, matrix multiplication, sequential I/O)
- Do not account for complex real-world scenarios, caching effects, kernel scheduling, or system load
- Reflect a single point-in-time snapshot of system performance
- Should not be used as the sole factor in system purchasing or deployment decisions

Use these results as **one of many data points** to understand your system's characteristics, not as definitive measures of overall capability or suitability for production workloads.

## Project Structure

```
src/
├── main.rs             - Entry point and benchmark orchestration
├── args.rs             - Command-line argument parsing
├── cpu.rs              - CPU performance benchmarks (primes, matrix, mandelbrot, FFT)
├── memory.rs           - Memory bandwidth benchmarks (sequential read/write)
├── disk.rs             - Disk I/O benchmarks (read/write operations)
├── stats.rs            - Statistical analysis utilities (mean, stddev, percentiles)
├── sysinfo_capture.rs  - System information capture (CPU, RAM, OS)
└── board_game.rs       - Easter egg simulation
```

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

This will execute all three benchmark suites sequentially with itemized results showing:
- **Memory**: Write and Read throughput separately, plus combined average
- **Disk**: Write and Read throughput separately, plus combined average
- **CPU**: Prime number sum calculation result

### CLI Arguments

The benchmark suite supports command-line arguments to customize behavior:

```bash
# Display help
cargo run --release -- --help

# Run with custom scale (default: 1.0)
# Higher scale = more intensive, longer duration
cargo run --release -- --scale 2.0

# Run benchmarks multiple times (default: 1)
# Results from multiple runs are averaged and analyzed statistically
cargo run --release -- --count 5

# Set number of threads for parallel benchmarks (default: 4)
cargo run --release -- --thread 8

# Export results to CSV with full statistical analysis
cargo run --release -- --csv --count 10

# Export results to JSON with detailed statistics and system info
cargo run --release -- --json --count 10

# Combine all options
cargo run --release -- --scale 2.0 --count 5 --thread 8 --csv --json
```

### Statistical Analysis

When running multiple benchmarks (`--count > 1`), the suite now provides comprehensive statistical analysis:
- **Mean**: Average value across all runs
- **Standard Deviation**: Measure of variance
- **Min/Max**: Range of results
- **Percentiles**: P50 (median), P95, P99
- **Coefficient of Variation**: Normalized measure of variability (%)

### System Information Capture

Every benchmark run now captures and displays:
- CPU brand and model
- Physical and logical core count
- Total system memory
- Operating system and version
- Hostname

### Warmup Phase

All benchmarks now include a warmup iteration before timing to:
- Prime CPU caches and branch predictors
- Stabilize disk caches
- Ensure consistent results across runs

## Benchmarks

### CPU Benchmark (Multiple calculations)
- **Prime Numbers**: Calculates primes and measures throughput (primes/sec)
- **Matrix Multiplication (Single-threaded)**: 256×256 matrix operation (GFLOPS)
- **Matrix Multiplication (Multi-threaded)**: Parallel matrix computation with configurable threads
- **Parallel Speedup**: Ratio of multi-threaded to single-threaded performance
- **Mandelbrot Set**: Fractal computation (pixels/sec)
- **Fast Fourier Transform (FFT)**: Signal processing benchmark (Msamples/sec)

### Memory Benchmark
Tests memory bandwidth by performing sequential writes and reads on a buffer.
- Sequential write throughput (MB/s)
- Sequential read throughput (MB/s)
- Combined average throughput

### Disk Benchmark
Evaluates disk I/O performance by writing and reading a test file.
- Sequential write throughput (MB/s)
- Sequential read throughput (MB/s)
- Combined average throughput
- Includes sync operations to measure actual disk persistence

## Output Formats

### Console Output
Real-time display of benchmark progress and results with system information.

### CSV Export (`--csv`)
Generates `output.csv` with:
- System information as comments (CPU, RAM, OS)
- Individual run results for each metric
- Full statistical analysis (mean, stddev, min, max, percentiles, CV%)
- One row per metric, columns for each run plus statistics

Example CSV structure:
```
# System Information
# CPU: Intel Core i7-9700K
# Cores: 8 physical, 8 logical
# Memory: 32768 MB
Metric,Run 1,Run 2,Run 3,Mean,StdDev,Min,Max,P50,P95,P99,CV%
CPU Primes (primes/sec),12500.00,12450.00,12550.00,12500.00,50.00,...
```

### JSON Export (`--json`)
Generates `output.json` with:
- Complete system information object
- Benchmark configuration
- Nested results structure with:
  - Individual run values
  - Statistical analysis for each metric
- Machine-readable format for CI/CD integration

Example JSON structure:
```json
{
  "system_info": {
    "cpu_brand": "Intel Core i7-9700K",
    "cpu_physical_cores": 8,
    "cpu_logical_cores": 8,
    ...
  },
  "configuration": {
    "scale": 1.0,
    "runs": 5,
    "threads": 4
  },
  "results": {
    "cpu": {
      "primes_per_sec": {
        "runs": [12500.00, 12450.00, ...],
        "statistics": {"mean": 12500.00, "std_dev": 50.00, ...}
      },
      ...
    }
  }
}
```

## Example Output and Interpretation

### Console Output Example

```
DISCLAIMER: Benchmark Results vs Actual System Capability
These results reflect runtime metrics for synthetic test scenarios
and do NOT necessarily equate to actual system capability for
real-world workloads. Use these results as one of many data points,
not as the sole basis for system evaluation.

=== System Information ===
CPU: Intel Core i7-9700K
Physical Cores: 8
Logical Cores: 8
Total Memory: 32768 MB
OS: Windows 10 Build 19045

=== Benchmark Configuration ===
Scale: 1.0
Runs: 3
Threads: 4

--- Run 1 ---
Running CPU Benchmark...
CPU Primes:              12500 primes/sec
CPU Matrix Mult (1T):    2.45 GFLOPS
CPU Matrix Mult (4T):    8.12 GFLOPS
CPU Speedup (4T):        3.32x
CPU Mandelbrot:          2500000 pixels/sec
CPU FFT:                 150 Msamples/sec
Duration: 2.34s

Running Memory Benchmark...
Memory Write: 12500.50 MB/s
Memory Read:  15000.25 MB/s
Memory Avg:   13750.38 MB/s
Duration: 0.52s

Running Disk Benchmark...
Disk Write: 450.75 MB/s
Disk Read:  520.25 MB/s
Disk Avg:   485.50 MB/s
Duration: 1.08s
```

### How to Interpret Results

**CPU Metrics:**
- **Primes/sec**: Higher is better. Measures raw computational throughput. Sensitive to CPU frequency and instruction-level parallelism.
- **GFLOPS (Giga Floating-Point Operations/Second)**: Higher is better. Matrix multiplication performance; single-threaded vs multi-threaded shows parallelization efficiency.
- **Speedup Ratio**: Shows how effectively your system uses multiple cores. A value close to the thread count indicates good scaling; lower values suggest memory bandwidth or lock contention.
- **Pixels/sec (Mandelbrot)**: Higher is better. Complex number calculations; tests floating-point performance.
- **Msamples/sec (FFT)**: Higher is better. Fast Fourier Transform throughput; sensitive to memory access patterns and cache efficiency.

**Memory Metrics:**
- **MB/s (Write/Read)**: Higher is better. Sequential memory throughput. Write vs Read differences indicate asymmetric memory controllers or CPU features (e.g., write-combining).
- **Combined Average**: Geometric mean of write and read speeds; useful for real-world workloads with mixed access patterns.

**Disk Metrics:**
- **MB/s (Write/Read)**: Higher is better. Sequential I/O throughput. Gap between write and read reflects disk scheduler behavior and caching.
- **Note**: Results are heavily influenced by filesystem cache and system load at runtime. Run multiple times (`--count 5+`) for stability.

**Statistical Analysis (with `--count > 1`):**
- **Mean**: Average value across all runs.
- **Std Dev**: Variability across runs. High values (>10% of mean) suggest system instability or background activity.
- **P95/P99**: 95th and 99th percentile values. Use for identifying tail latencies and worst-case scenarios.
- **CV% (Coefficient of Variation)**: Normalized variability. <5% is excellent; >15% suggests noisy results.

### Comparison Tips

1. **Relative Benchmarking**: Use these results to compare *before/after* on the same system (e.g., after software updates or configuration changes).
2. **Multiple Runs**: Always use `--count 5+` for production comparisons. A single run is unreliable due to system noise.
3. **Controlled Environment**: Close background apps, disable dynamic CPU scaling if possible, and run on a quiet system.
4. **Document Context**: Record the exact command, system load, and ambient conditions for reproducibility.
5. **Don't Over-Extrapolate**: These synthetic workloads don't reflect your specific real-world use case. Benchmark your actual workload for deployment decisions.

## Testing

```bash
cargo test
```

## Data Export and Visualization

Each benchmark run automatically exports results in two formats:

### CSV Output
- Filename: `output_YYYYMMDD_HHMMSS.csv` (e.g., `output_20260125_143022.csv`)
- Contains raw benchmark metrics as structured tabular data
- Header includes: `name`, `category`, `mean`, `std_dev`, `min`, `max`, `count`
- No comment lines - data is directly importable into Excel, analysis tools, etc.

### JSON Output
- Filename: `output_YYYYMMDD_HHMMSS.json` (e.g., `output_20260125_143022.json`)
- Includes metadata section with:
  - `timestamp`: RFC3339 format timestamp (e.g., `2026-01-25T14:30:22+00:00`)
  - `hostname`: Machine hostname for multi-machine comparisons
- Contains full system information (CPU, memory, OS)
- Preserves all statistics and raw benchmark results

### Interactive Visualization

Open `visualize.html` in any modern web browser to:
- **Load and compare** multiple JSON benchmark files
- **Drag & drop** files for easy loading
- **View machine names and timestamps** for each data point
- **Analyze trends** across multiple runs or different machines
- **Compare metrics** side-by-side with interactive charts
- **Export visuals** to PNG using browser developer tools

#### Usage Example

1. Run benchmarks on multiple machines or at different times
2. Collect the `output_*.json` files
3. Open `visualize.html` in your web browser
4. Drag and drop the JSON files onto the upload area
5. Select metrics to visualize or view all metrics at once
6. Compare performance across machines and time periods

This makes it easy to track performance trends or compare system configurations.

## Release Process

Releases are automatically built and published when a git tag is pushed with the format `v{version}` (e.g., `v0.1.0`, `v1.2.3`).

To create a release:

```bash
# Tag the current commit
git tag v0.1.0

# Push the tag (this triggers the release workflow)
git push origin v0.1.0
```

The release workflow will:
1. Build optimized binaries for Windows, Linux, and macOS
2. Create a GitHub Release with the tag name
3. Upload binary artifacts for each platform

### Available Artifacts

After a release is published, the following binaries are available:
- `hs-benchmark-suite-linux-x86_64` - Linux binary
- `hs-benchmark-suite-windows-x86_64.exe` - Windows binary
- `hs-benchmark-suite-macos-x86_64` - macOS Intel binary
- `hs-benchmark-suite-macos-aarch64` - macOS Apple Silicon binary

## Dependencies

- **sysinfo**: System information and monitoring
- **chrono**: Date and time utilities
- **criterion**: Benchmarking framework (for future micro-benchmarks)
- **libc**: Low-level C library bindings

## Performance Tips

- Always run benchmarks in **release mode** (`--release`) for accurate measurements
- Close other applications for more consistent results
- Run multiple times to account for system variability

## License

MIT License - Copyright (c) 2026 Hideki Saito

See [LICENSE](LICENSE) for details.
