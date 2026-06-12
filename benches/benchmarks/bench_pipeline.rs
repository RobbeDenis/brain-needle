use criterion::*;
use super::utils::*;
use std::hint::black_box;
use bn::bncore::create_bnreader;
use bn::bncore::build_config_from;

#[allow(unused)]
fn mandelbrot(c: &mut Criterion) {
    let config = black_box(build_config_from("bf/mandelbrot.bf", std_lin_int_ctx()));

    c.bench_function("Mandelbrot Single", |b| {
        b.iter_with_setup(|| {
            black_box(build_config_from("bf/mandelbrot.bf", std_lin_int_ctx()))
        }, 
        |config| {
            let output = bn::istream::intermediate::generate_intermediate(create_bnreader(config.file_path)).unwrap_or_else(|err| {
            println!("{err}");
            std::process::exit(1);
            });
            bn::istream::codegen::generate_output::<bn::istream::emit::BCEmitterFactoryDefault>(output, config.context);
        });
    });
}

#[allow(unused)]
fn mandelbrot_enum(c: &mut Criterion) {
    let config = black_box(build_config_from("bf/mandelbrot.bf", std_lin_int_ctx()));

    c.bench_function("Mandelbrot Enum", |b| {
        b.iter_with_setup(|| {
            black_box(build_config_from("bf/mandelbrot.bf", std_lin_int_ctx()))
        }, 
        |config| {
            let output = bn::bnintermediate::generate_intermediate_representation(create_bnreader(config.file_path)).unwrap_or_else(|err| {
                println!("{err}");
                std::process::exit(1);
            });
            bn::bngen::generate_output::<bn::bnemit::BNEmitterFactoryDefault>(output, config.context);
        });
    });
}

criterion_group!(benches, mandelbrot);
criterion_group!(benches_enum, mandelbrot_enum);

