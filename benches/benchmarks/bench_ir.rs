use criterion::*;
use std::hint::black_box;
use brain_needle::bnintermediate::generate_intermediate_representation;

pub fn generate_ir(c: &mut Criterion)
{
    let contents = std::fs::read_to_string("bf/mandelbrot_extreme.bf").unwrap();
    let heavy_load = contents.repeat(8).into_bytes();

    c.bench_function("Generate IR", |b| {
        b.iter_with_setup(|| {
                std::io::Cursor::new(heavy_load.clone())
            },
            |reader| {
                generate_intermediate_representation(black_box(reader))
            }
        )
    });
}

criterion_group!(benches, generate_ir);