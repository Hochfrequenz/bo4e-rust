#!/usr/bin/env python3
"""Benchmark BO4E-Python for comparison with Rust.

This script benchmarks BO4E-Python serialization and deserialization
for comparison with the Rust implementation.

Usage:
    pip install -r requirements.txt
    python benchmark.py
"""

import json
import statistics
import time

from bo4e import Zaehler
from bo4e.enum import Sparte

try:
    import orjson

    HAS_ORJSON = True
except ImportError:
    HAS_ORJSON = False


def create_meter(i: int) -> Zaehler:
    """Create a meter object for benchmarking."""
    return Zaehler(
        zaehlernummer=f"1EMH{i:010d}",
        sparte=Sparte.STROM,
    )


def benchmark(name: str, func, iterations: int = 1000):
    """Run a benchmark and report results."""
    # Warmup
    for _ in range(min(100, iterations // 10)):
        func()

    # Actual benchmark
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        func()
        end = time.perf_counter()
        times.append((end - start) * 1_000_000)  # microseconds

    mean = statistics.mean(times)
    stddev = statistics.stdev(times) if len(times) > 1 else 0
    median = statistics.median(times)
    p95 = sorted(times)[int(len(times) * 0.95)]

    print(f"{name}:")
    print(f"  mean:   {mean:.2f} µs")
    print(f"  stddev: {stddev:.2f} µs")
    print(f"  median: {median:.2f} µs")
    print(f"  p95:    {p95:.2f} µs")


def main():
    """Run all benchmarks."""
    print("=" * 60)
    print("BO4E-Python Benchmarks")
    print("=" * 60)
    print()

    # Single object serialization
    print("Single Object Serialization")
    print("-" * 40)
    meter = create_meter(0)

    def serialize_model_dump_json():
        return meter.model_dump_json()

    benchmark("model_dump_json", serialize_model_dump_json)

    if HAS_ORJSON:

        def serialize_orjson():
            return orjson.dumps(meter.model_dump())

        benchmark("orjson", serialize_orjson)
    print()

    # Single object deserialization
    print("Single Object Deserialization")
    print("-" * 40)
    json_str = meter.model_dump_json()

    def deserialize():
        return Zaehler.model_validate_json(json_str)

    benchmark("model_validate_json", deserialize)
    print()

    # Batch operations
    print("Batch Operations")
    print("-" * 40)
    for size in [10, 100, 1000]:
        print(f"\nBatch size: {size}")
        meters = [create_meter(i) for i in range(size)]

        # Use default_factory to avoid closure issues
        def make_batch_serialize(m):
            def batch_serialize():
                return json.dumps([obj.model_dump() for obj in m])

            return batch_serialize

        iterations = max(10, 1000 // size)
        benchmark(f"  serialize {size} meters", make_batch_serialize(meters), iterations)

        # Batch deserialization
        json_array = json.dumps([m.model_dump() for m in meters])

        def make_batch_deserialize(j):
            def batch_deserialize():
                data = json.loads(j)
                return [Zaehler.model_validate(obj) for obj in data]

            return batch_deserialize

        benchmark(
            f"  deserialize {size} meters", make_batch_deserialize(json_array), iterations
        )

    print()
    print("=" * 60)
    print("Benchmark complete")
    print("=" * 60)


if __name__ == "__main__":
    main()
