use criterion::{criterion_group, criterion_main, Criterion};
use lang::lexer::Lexer;

#[allow(clippy::unwrap_used)]
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lex hello world", |b| {
        b.iter(|| {
            let contents = std::fs::read_to_string("../examples/hello_world.lt").unwrap();

            let mut lexer = Lexer::new(&contents);
            let _ = lexer.tokenize().unwrap();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
