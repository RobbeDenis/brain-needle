pub use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::bench_pipeline::benches,
    // benchmarks::bench_pipeline::benches_enum,
    // benchmarks::bench_compare_ir::compare,
    // benchmarks::bench_compare_ir::enum_vector,
    // benchmarks::bench_compare_ir::bytestream,
}