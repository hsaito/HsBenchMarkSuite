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

# Run with default settings (count: 3)
cargo run --release

# Run with custom scale (default: 1.0)
# Higher scale = more intensive, longer duration
cargo run --release -- --scale 2.0

# Run benchmarks multiple times for better statistics (default: 3)
cargo run --release -- --count 5

# Set number of threads for parallel benchmarks (default: 4)
cargo run --release -- --thread 8

# Set disk benchmark block size in bytes (default: 524288 = 512 KB)
# Use 131072 for 128 KB, 1048576 for 1 MB
cargo run --release -- --block-size 1048576

# Export results to CSV with full statistical analysis
cargo run --release -- --csv --count 10

# Export results to JSON with detailed statistics and system info
cargo run --release -- --json --count 10

# Combine all options
cargo run --release -- --scale 2.0 --count 5 --thread 8 --block-size 262144 --csv --json
```

### Statistical Analysis

When running multiple benchmarks (`--count > 1`), the suite now provides comprehensive statistical analysis:
- **Mean**: Average value across all runs
- **Standard Deviation**: Measure of variance
- **Min/Max**: Range of results
- **Percentiles**: P50 (median), P95, P99
- **Coefficient of Variation**: Normalized measure of variability (%)

**Note**: Statistical metrics (standard deviation, percentiles, coefficient of variation) are only meaningful when running multiple times (`--count > 1`). Single-run benchmarks will show all values as 0 or N/A for these metrics, as there is no variance to measure. For reliable statistical analysis, use at least 3-5 runs (e.g., `--count 5`).

### Disk Benchmark Configuration

The disk benchmark now supports configurable block sizes for testing different I/O patterns:

```rust
use hs_benchmark_suite::disk::run_disk_benchmark_scaled_with_block_size;

// Use default 512 KB block size
let result = run_disk_benchmark_scaled(1.0);

// Test with custom block sizes
let result_128k = run_disk_benchmark_scaled_with_block_size(1.0, 128 * 1024);  // Small blocks for random access
let result_512k = run_disk_benchmark_scaled_with_block_size(1.0, 512 * 1024);  // Default (sequential)
let result_1m = run_disk_benchmark_scaled_with_block_size(1.0, 1024 * 1024);   // Large blocks for streaming
```

**Default block size**: 512 KB provides a good balance between:
- Amortizing syscall overhead
- Fitting in typical CPU caches (L3: 8-24 MB)
- Matching filesystem page sizes

**Platform support**: Direct I/O with sector alignment (4096 bytes) across Windows, Linux, FreeBSD, and macOS.

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

Each benchmark run automatically exports results in two formats for analysis and comparison.

### CSV Output
- **Filename**: `output_YYYYMMDD_HHMMSS.csv` (e.g., `output_20260125_143022.csv`)
- **Format**: Clean tabular data with no comment lines, directly importable into Excel, Pandas, R, etc.
- **Contents**:
  - Rows represent individual metrics (CPU primes, memory throughput, disk throughput, etc.)
  - Columns include: metric name, results from each run, statistical summaries
  - Headers: `name`, `category`, `mean`, `std_dev`, `min`, `max`, `count`, `p50`, `p95`, `p99`, `cv_percent`
  - Prefixed metric names (e.g., `cpu_primes_per_sec`, `memory_write_throughput_mbs`, `disk_read_throughput_mbs`)
- **Use Case**: Data analysis, spreadsheet applications, statistical tools, trend analysis over time

### JSON Output
- **Filename**: `output_YYYYMMDD_HHMMSS.json` (e.g., `output_20260125_143022.json`)
- **Metadata Section**:
  - `timestamp`: RFC3339 format (e.g., `2026-01-25T14:30:22+00:00`) - enables trend tracking
  - `hostname`: Machine hostname - essential for multi-machine comparisons
- **Contents**:
  - Complete system information (CPU brand/cores, memory, OS version)
  - Benchmark configuration (scale, thread count, number of runs)
  - All metrics with individual run values and full statistical analysis
  - Prefixed metric names for easy programmatic access
- **Use Case**: CI/CD integration, machine-readable format for automated analysis, backup/archiving

### Interactive HTML Visualization (`visualize.html`)

A fully-featured interactive tool included with every release for comparing and analyzing benchmark results across machines and time periods.

**Features**:
- **Multi-file Loading**: Drag and drop multiple JSON files simultaneously to compare them
- **Automatic Metrics Discovery**: Detects all unique metrics across loaded files
- **Interactive Charts**: One chart per metric showing all loaded files side-by-side
- **Machine Identification**: Displays hostname and timestamp for each file; shows warning badges when comparing different machines
- **File Management**: Reorder files by drag-and-drop, remove individual files, or clear all to start fresh
- **Responsive Design**: Charts automatically scale to fit your screen

See [Using the HTML Visualization Tool](#using-the-html-visualization-tool) below for detailed usage instructions and example workflows.

## Using the HTML Visualization Tool

### Preparation

Run benchmarks on different machines or at different times, collecting the JSON output files:

```bash
# Run benchmarks with JSON output
cargo run --release -- --count 5 --json

# Creates: output_YYYYMMDD_HHMMSS.json
# Example: output_20260125_143022.json
```

### Getting Started

1. **Open the tool**: Launch `visualize.html` in any modern web browser (Chrome, Firefox, Safari, Edge)
   - File is self-contained with no external dependencies or installation required

2. **Load your data**: Drag and drop one or more `output_*.json` files onto the page
   - Files are appended to the comparison (existing files are retained)
   - Charts appear immediately with automatic scaling

3. **View results**: Each chart displays one metric with bars for each loaded file
   - Colors automatically assigned to distinguish files
   - Hover over bars for exact values
   - Orange warning badges indicate when comparing different machines
   - **Parameter mismatch warnings**: Automatic detection alerts you when files use different benchmark parameters (scale, runs, threads, block_size)

### Managing Files

- **Configuration details**: Each file shows its benchmark configuration (Scale, Runs, Threads, Block size)
- **Parameter warnings**: If files have different parameters, a warning banner appears suggesting fair comparison practices
- **Reorder**: Drag files to change chart ordering (charts update in real-time)
- **Remove**: Click the remove button on individual files to exclude them from analysis
- **Clear all**: Start fresh by removing all files at once

### Example Workflows

#### Trend Analysis
Track performance changes on a single machine over time:

```bash
# Run on Day 1
cargo run --release -- --count 5 --json  # output_20260125_143022.json

# ... a week later ...

# Run on Day 8
cargo run --release -- --count 5 --json  # output_20260201_090000.json

# Load both in visualize.html to see performance trends
```

#### Cross-Platform Comparison
Compare performance across Windows, Linux, and macOS:

```bash
# Run on each platform and collect results
# Windows: output_20260125_100000.json
# Linux:  output_20260125_101000.json
# macOS:  output_20260125_102000.json

# Load all in visualize.html
# Notice orange warning badges for different machines
# Compare CPU/memory/disk performance across platforms
```

#### Configuration Testing
Benchmark different system configurations:

```bash
# Test with 4 threads
cargo run --release -- --thread 4 --count 5 --json   # output_*_config1.json

# Test with 8 threads
cargo run --release -- --thread 8 --count 5 --json   # output_*_config2.json

# Load both to see the performance impact of thread count
# ⚠️ Note: visualize.html will warn about the thread count difference
```

#### Fair Comparison Best Practices

When comparing results, ensure consistent parameters:

```bash
# ✅ GOOD: Same parameters across runs
cargo run --release -- --scale 1.0 --count 5 --thread 4 --json  # Run 1
# ... (wait a bit to let system settle)
cargo run --release -- --scale 1.0 --count 5 --thread 4 --json  # Run 2

# ❌ AVOID: Different parameters make comparison misleading
cargo run --release -- --scale 1.0 --count 3 --json  # Run 1
cargo run --release -- --scale 2.0 --count 5 --json  # Run 2 - different scale & count!
# Loading both in visualize.html will display a parameter mismatch warning
```

### Technical Details

- **Chart Library**: Chart.js 4.4.0 (loaded from CDN)
- **Colors**: Assigned from a predefined palette based on file load order
- **Parameter Validation**: Automatically detects and warns about differences in scale, runs, threads, and block_size
- **Configuration Metadata**: Each JSON file includes benchmark parameters for comparison accuracy
- **Metadata**: Automatically extracted from JSON files (hostname, timestamp)
- **Responsive Layout**: CSS Grid scales charts from 500px to available width
- **No Installation**: Entire tool runs in the browser with no dependencies



Releases are automatically built and published when a git tag is pushed with the format `v{version}` (e.g., `v0.2.1`, `v1.2.3`).

To create a release:

```bash
# Tag the current commit
git tag v0.2.1

# Push the tag (this triggers the release workflow)
git push origin v0.2.1
```

The release workflow will:
1. Build optimized binaries for Windows (x86_64), Linux (x86_64), and macOS (x86_64 & Apple Silicon)
2. Create a GitHub Release with the tag name
3. Bundle each binary with README.md, LICENSE, and visualize.html
4. Upload ZIP archives for each platform (named with version suffix)

### Available Release Artifacts

After a release is published (e.g., `v0.2.1`), the following are available for download:
- `hs-benchmark-suite-linux-x86_64-v0.2.1.zip` - Linux binary + docs + visualization tool
- `hs-benchmark-suite-windows-x86_64.exe-v0.2.1.zip` - Windows binary + docs + visualization tool
- `hs-benchmark-suite-macos-x86_64-v0.2.1.zip` - macOS Intel binary + docs + visualization tool
- `hs-benchmark-suite-macos-aarch64-v0.2.1.zip` - macOS Apple Silicon binary + docs + visualization tool

Each ZIP includes:
- Compiled binary (ready to run)
- README.md (this file with full documentation)
- LICENSE (MIT license)
- visualize.html (interactive visualization tool)

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
