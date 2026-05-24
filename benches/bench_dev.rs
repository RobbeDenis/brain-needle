use criterion::*;
use std::hint::black_box;
use brain_needle::*;
use brain_needle::bnintrep::generate_intermediate_representation;
use brain_needle::bnlex::tokenize_file_from_path;
use brain_needle::bnparse::create_flat_instr_tree_from_tokens;

pub fn compare_generate_ir(c: &mut Criterion) 
{
    let mut group = c.benchmark_group("Generate IR: compare");

    group.sample_size(200);
    group.measurement_time(std::time::Duration::new(20, 0));

    for id in ["base", "extreme", "max"] {
        let setup_reader = || bncore::create_bnreader(format!("src-bf\\mandelbrot_{}.bf", id));
        group.bench_function(BenchmarkId::new(id, "Old"), |b| {
            b.iter_with_setup(setup_reader, |reader| {
                create_flat_instr_tree_from_tokens(tokenize_file_from_path(black_box(reader)).unwrap())
            })
        });
        group.bench_function(BenchmarkId::new(id, "New"), |b| {
            b.iter_with_setup(setup_reader, |reader| {
                generate_intermediate_representation(black_box(reader))
            })
        });
    }

    group.finish();
}

pub fn generate_ir(c: &mut Criterion)
{
    let file_path = std::path::PathBuf::from("src-bf\\mandelbrot_max.bf");

    c.bench_function("Generate IR", |b| {
        b.iter_with_setup(|| {
                bncore::create_bnreader(&file_path)
            },
            |reader| {
                generate_intermediate_representation(black_box(reader))
            }
        )
    });
} 

criterion_group!(benches, generate_ir);
// criterion_group!(benches, compare_generate_ir);
criterion_main!(benches);