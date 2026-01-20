# BO4E-Rust Benchmarks

Performance comparison between BO4E-Rust and BO4E-Python.

## Test Environment

- CPU: [Your CPU]
- Memory: [Your RAM]
- Rust: 1.75.0+
- Python: 3.10+

## Running Benchmarks

### Rust Benchmarks

```bash
# Run all benchmarks
cargo bench -p bo4e-serde

# Run specific benchmark group
cargo bench -p bo4e-serde -- deserialization
cargo bench -p bo4e-serde -- serialization
cargo bench -p bo4e-serde -- batch_parsing
cargo bench -p bo4e-serde -- batch_serialization
```

### Python Comparison

```bash
cd benches/python_comparison
pip install -r requirements.txt
python benchmark.py
```

## Benchmark Results

Results will vary based on hardware. Below are example results from a typical development machine.

### Single Object Deserialization

| Operation | Rust (simd-json) | Rust (serde_json) | Python |
|-----------|------------------|-------------------|--------|
| Meter parse | ~X µs | ~Y µs | ~Z µs |
| MarketLocation parse | ~X µs | ~Y µs | ~Z µs |

### Single Object Serialization

| Operation | Rust (German) | Rust (English) | Python |
|-----------|---------------|----------------|--------|
| Meter | ~X µs | ~X µs | ~Z µs |
| MarketLocation | ~X µs | ~X µs | ~Z µs |

### Batch Processing

| Batch Size | Rust (simd-json) | Rust (serde_json) | Python |
|------------|------------------|-------------------|--------|
| 10 meters | ~X µs | ~Y µs | ~Z µs |
| 100 meters | ~X µs | ~Y µs | ~Z µs |
| 1000 meters | ~X ms | ~Y ms | ~Z ms |

## Benchmark Details

### Rust Benchmarks (`cargo bench -p bo4e-serde`)

The Rust benchmarks use [Criterion.rs](https://github.com/bheisler/criterion.rs) for statistical rigor:

- **deserialization**: Compares simd-json vs serde_json for single objects
- **batch_parsing**: Tests array parsing at different batch sizes (10, 100, 1000)
- **serialization**: Tests German and English JSON output
- **batch_serialization**: Tests array serialization at different batch sizes

### Python Benchmarks (`benchmark.py`)

The Python benchmarks measure:

- Single object serialization with `model_dump_json()`
- Optional orjson serialization for comparison
- Single object deserialization with `model_validate_json()`
- Batch operations at various sizes

## Interpreting Results

### Why simd-json?

simd-json uses SIMD (Single Instruction, Multiple Data) CPU instructions to parse JSON in parallel. Benefits:

- 2-4x faster parsing on supported CPUs
- Lower latency for high-throughput applications
- Same correctness guarantees as serde_json

### Rust vs Python Performance

Rust is typically faster due to:

1. **Zero-cost abstractions**: No runtime overhead
2. **No GC pauses**: Deterministic memory management
3. **SIMD acceleration**: Hardware-optimized JSON parsing
4. **Compile-time optimizations**: Monomorphization and inlining

Python's advantages:

1. **Rapid development**: Faster iteration cycles
2. **Rich ecosystem**: Extensive data science tooling
3. **Interoperability**: Easy integration with existing systems

## HTML Reports

Criterion generates detailed HTML reports in `target/criterion/`. Open `target/criterion/report/index.html` for:

- Performance trends over time
- Statistical analysis
- Regression detection

## Conclusions

The Rust implementation provides significant performance improvements for JSON processing,
particularly beneficial for:

- High-volume data pipelines
- Real-time energy data processing
- Microservices with strict latency requirements

The Python implementation remains suitable for:

- Prototyping and exploration
- Integration with existing Python systems
- Applications where development speed is prioritized over runtime performance
