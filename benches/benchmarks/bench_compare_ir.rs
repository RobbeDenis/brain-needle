use super::utils::*;
use criterion::*;
use std::hint::black_box;
use brain_needle::bnbytecode::bytecode_intermediate::generate_bytecode_intermediate_representation;
use brain_needle::bnintermediate::generate_intermediate_representation;
use brain_needle::bstream::bstream_intermediate::generate_bytestream_intermediate_representation;

const TIMES_CONTENT: usize = 8;

fn bytecode_ir(c: &mut Criterion) {
    c.bench_function("Bytecode IR", |b| {
        b.iter_with_setup(|| {
                black_box(std::io::Cursor::new(multiply_contents_into_bytes("bf/big/mandelbrot_extreme.bf", TIMES_CONTENT)))
            },
            |reader| {
                let _result = generate_bytecode_intermediate_representation(black_box(reader)).unwrap();
                black_box(_result);
            }
        )
    });
}

fn enum_vector_ir(c: &mut Criterion) {
    c.bench_function("Enum Vector IR", |b| {
        b.iter_with_setup(|| {
                black_box(std::io::Cursor::new(multiply_contents_into_bytes("bf/big/mandelbrot_extreme.bf", TIMES_CONTENT)))
            },
            |reader| {
                let _result = generate_intermediate_representation(black_box(reader)).unwrap();
                black_box(_result);
            }
        )
    });
}

fn bytestream_ir(c: &mut Criterion) {
    c.bench_function("Bytestream IR", |b| {
        b.iter_with_setup(|| {
                black_box(std::io::Cursor::new(multiply_contents_into_bytes("bf/big/mandelbrot_extreme.bf", TIMES_CONTENT)))
            },
            |reader| {
                let _result = generate_bytestream_intermediate_representation(black_box(reader)).unwrap();
                black_box(_result);
            }
        )
    });
}

criterion_group!(compare, bytecode_ir, enum_vector_ir, bytestream_ir);
criterion_group!(bytecode, bytecode_ir);
criterion_group!(enum_vector, enum_vector_ir);
criterion_group!(bytestream, bytestream_ir);