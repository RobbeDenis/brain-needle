use super::utils::*;
use criterion::*;
use std::hint::black_box;
use bn::*;

const TIMES_CONTENT: usize = 8;

#[allow(unused)]
fn enum_vector_ir(c: &mut Criterion) {
    c.bench_function("Enum Vector IR", |b| {
        b.iter_with_setup(|| {
                black_box(std::io::Cursor::new(multiply_contents_into_bytes("bf/big/mandelbrot_extreme.bf", TIMES_CONTENT)))
            },
            |reader| {
                let _result = bnintermediate::generate_intermediate_representation(black_box(reader)).unwrap();
                black_box(_result);
            }
        )
    });
}

#[allow(unused)]
fn bytestream_ir(c: &mut Criterion) {
    c.bench_function("Bytestream IR", |b| {
        b.iter_with_setup(|| {
                black_box(std::io::Cursor::new(multiply_contents_into_bytes("bf/big/mandelbrot_extreme.bf", TIMES_CONTENT)))
            },
            |reader| {
                let _result = istream::intermediate::generate_intermediate(black_box(reader)).unwrap();
                black_box(_result);
            }
        )
    });
}

criterion_group!(compare, enum_vector_ir, bytestream_ir);
criterion_group!(enum_vector, enum_vector_ir);
criterion_group!(bytestream, bytestream_ir);