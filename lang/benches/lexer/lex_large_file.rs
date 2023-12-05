use criterion::{criterion_group, criterion_main, Criterion};
use lang::lexer::Lexer;

#[allow(clippy::unwrap_used)]
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("lex large file");

    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(75));
    group.bench_function("lex large file", |b| {
        b.iter(|| {
            let contents = std::fs::read_to_string("../examples/large_file.lt").unwrap();

            let mut lexer = Lexer::new(&contents);
            let _ = lexer.tokenize().unwrap();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
