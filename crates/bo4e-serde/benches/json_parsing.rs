//! Comprehensive JSON parsing benchmarks.
//!
//! Compares simd-json vs serde_json performance for BO4E types.

use bo4e_core::bo::{MarketLocation, Meter};
use bo4e_core::com::{Address, MeterRegister};
use bo4e_core::enums::{Division, EnergyDirection, MeterType, Unit};
use bo4e_core::traits::Bo4eMeta;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn create_meter() -> Meter {
    Meter {
        meta: Bo4eMeta::with_type("Zaehler"),
        meter_number: Some("1EMH0012345678".to_string()),
        division: Some(Division::Electricity),
        meter_type: Some(MeterType::ModernMeasuringDevice),
        registers: vec![MeterRegister {
            obis_code: Some("1-0:1.8.0".to_string()),
            energy_direction: Some(EnergyDirection::FeedOut),
            unit: Some(Unit::KilowattHour),
            ..Default::default()
        }],
        ..Default::default()
    }
}

fn create_market_location() -> MarketLocation {
    MarketLocation {
        meta: Bo4eMeta::with_type("Marktlokation"),
        market_location_id: Some("12345678901".to_string()),
        division: Some(Division::Electricity),
        energy_direction: Some(EnergyDirection::FeedOut),
        address: Some(Address {
            street: Some("Musterstraße".to_string()),
            house_number: Some("42".to_string()),
            postal_code: Some("50667".to_string()),
            city: Some("Köln".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn bench_deserialization(c: &mut Criterion) {
    let meter_json = bo4e_serde::to_json_german(&create_meter()).unwrap();
    let malo_json = bo4e_serde::to_json_german(&create_market_location()).unwrap();

    let mut group = c.benchmark_group("deserialization");

    // Meter - simd_json
    group.throughput(Throughput::Bytes(meter_json.len() as u64));
    group.bench_function("meter/simd_json", |b| {
        b.iter(|| {
            let mut bytes = meter_json.as_bytes().to_vec();
            let _: Meter = bo4e_serde::from_json(black_box(&mut bytes)).unwrap();
        })
    });

    // Meter - serde_json
    group.bench_function("meter/serde_json", |b| {
        b.iter(|| {
            let _: Meter = serde_json::from_str(black_box(&meter_json)).unwrap();
        })
    });

    // MarketLocation - simd_json
    group.throughput(Throughput::Bytes(malo_json.len() as u64));
    group.bench_function("market_location/simd_json", |b| {
        b.iter(|| {
            let mut bytes = malo_json.as_bytes().to_vec();
            let _: MarketLocation = bo4e_serde::from_json(black_box(&mut bytes)).unwrap();
        })
    });

    // MarketLocation - serde_json
    group.bench_function("market_location/serde_json", |b| {
        b.iter(|| {
            let _: MarketLocation = serde_json::from_str(black_box(&malo_json)).unwrap();
        })
    });

    group.finish();
}

fn bench_batch_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_parsing");

    for size in [10, 100, 1000] {
        let meters: Vec<Meter> = (0..size)
            .map(|i| Meter {
                meter_number: Some(format!("1EMH{:010}", i)),
                division: Some(Division::Electricity),
                ..Default::default()
            })
            .collect();

        let json = serde_json::to_string(&meters).unwrap();
        let bytes = json.as_bytes().to_vec();

        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(BenchmarkId::new("simd_json", size), &bytes, |b, bytes| {
            b.iter(|| {
                let mut bytes = bytes.clone();
                let _: Vec<Meter> = bo4e_serde::from_json(black_box(&mut bytes)).unwrap();
            })
        });

        group.bench_with_input(BenchmarkId::new("serde_json", size), &json, |b, json| {
            b.iter(|| {
                let _: Vec<Meter> = serde_json::from_str(black_box(json)).unwrap();
            })
        });
    }

    group.finish();
}

fn bench_serialization(c: &mut Criterion) {
    let meter = create_meter();
    let malo = create_market_location();

    let mut group = c.benchmark_group("serialization");

    // Meter serialization
    group.bench_function("meter/german", |b| {
        b.iter(|| {
            let _ = bo4e_serde::to_json_german(black_box(&meter)).unwrap();
        })
    });

    group.bench_function("meter/english", |b| {
        b.iter(|| {
            let _ = bo4e_serde::to_json_english(black_box(&meter)).unwrap();
        })
    });

    // MarketLocation serialization
    group.bench_function("market_location/german", |b| {
        b.iter(|| {
            let _ = bo4e_serde::to_json_german(black_box(&malo)).unwrap();
        })
    });

    group.bench_function("market_location/english", |b| {
        b.iter(|| {
            let _ = bo4e_serde::to_json_english(black_box(&malo)).unwrap();
        })
    });

    group.finish();
}

fn bench_batch_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_serialization");

    for size in [10, 100, 1000] {
        let meters: Vec<Meter> = (0..size)
            .map(|i| Meter {
                meter_number: Some(format!("1EMH{:010}", i)),
                division: Some(Division::Electricity),
                ..Default::default()
            })
            .collect();

        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(
            BenchmarkId::new("serde_json", size),
            &meters,
            |b, meters| {
                b.iter(|| {
                    let _ = serde_json::to_string(black_box(meters)).unwrap();
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_deserialization,
    bench_batch_parsing,
    bench_serialization,
    bench_batch_serialization
);
criterion_main!(benches);
