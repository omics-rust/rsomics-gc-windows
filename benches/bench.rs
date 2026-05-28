use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_gc_windows(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-gc-windows");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fasta = manifest.join("tests/golden/small.fa");
    c.bench_function("rsomics-gc-windows golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .args([fasta.to_str().unwrap(), "-w", "100"])
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_gc_windows);
criterion_main!(benches);
