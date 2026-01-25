# HsBenchMarkSuite Development Guidelines

## Project Overview
A comprehensive Rust benchmark suite for CPU, memory, and disk performance analysis.

## Architecture

The project is organized into modular benchmark components:
- **cpu.rs**: Prime number calculation to stress test CPU
- **memory.rs**: Sequential read/write operations on large buffers
- **disk.rs**: File I/O operations to measure disk throughput
- **main.rs**: Orchestrates all benchmarks and displays results

## Development Workflow

### Building
```bash
cargo build --release
```

### Running
```bash
cargo run --release
```

### Testing
```bash
cargo test
```

## Adding New Benchmarks

1. Create a new module file in `src/`
2. Implement the benchmark function
3. Call it from `main.rs`
4. Add tests for validation

## Performance Considerations

- Use `--release` mode for optimized builds
- Consider thread-safety when adding concurrent benchmarks
- Minimize allocations within timed sections
- Use volatile writes where needed to prevent compiler optimizations

## Dependencies Management

Current dependencies are minimal and focused:
- `sysinfo`: For system metrics
- `chrono`: For timing utilities
- `criterion`: For detailed benchmarking (future use)
- `libc`: For OS-level operations if needed

Add dependencies judiciously to keep the suite lean.
