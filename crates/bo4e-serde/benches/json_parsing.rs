//! JSON parsing benchmarks comparing simd-json vs serde_json.

use bo4e_core::bo::Meter;
use bo4e_core::enums::Division;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn create_test_json() -> String {
    // Create a realistic meter JSON
    serde_json::to_string(&Meter {
        meter_number: Some("1EMH0012345678".to_string()),
        division: Some(Division::Electricity),
        ..Default::default()
    })
    .unwrap()
}

fn create_large_json() -> String {
    // Array of 1000 meters
    let meters: Vec<Meter> = (0..1000)
        .map(|i| Meter {
            meter_number: Some(format!("1EMH{:010}", i)),
            division: Some(Division::Electricity),
            ..Default::default()
        })
        .collect();
    serde_json::to_string(&meters).unwrap()
}

fn bench_single_object(c: &mut Criterion) {
    let json = create_test_json();
    let json_bytes = json.as_bytes().to_vec();

    let mut group = c.benchmark_group("single_object");
    group.throughput(Throughput::Bytes(json.len() as u64));

    group.bench_function("serde_json", |b| {
        b.iter(|| {
            let meter: Meter = serde_json::from_str(black_box(&json)).unwrap();
            black_box(meter)
        })
    });

    group.bench_function("simd_json", |b| {
        b.iter(|| {
            let mut bytes = json_bytes.clone();
            let meter: Meter = bo4e_serde::from_slice(black_box(&mut bytes)).unwrap();
            black_box(meter)
        })
    });

    group.finish();
}

fn bench_array_1000(c: &mut Criterion) {
    let json = create_large_json();
    let json_bytes = json.as_bytes().to_vec();

    let mut group = c.benchmark_group("array_1000");
    group.throughput(Throughput::Bytes(json.len() as u64));

    group.bench_function("serde_json", |b| {
        b.iter(|| {
            let meters: Vec<Meter> = serde_json::from_str(black_box(&json)).unwrap();
            black_box(meters)
        })
    });

    group.bench_function("simd_json", |b| {
        b.iter(|| {
            let mut bytes = json_bytes.clone();
            let meters: Vec<Meter> = bo4e_serde::from_slice(black_box(&mut bytes)).unwrap();
            black_box(meters)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_single_object, bench_array_1000);
criterion_main!(benches);
