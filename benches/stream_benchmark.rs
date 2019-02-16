#[macro_use]
extern crate criterion;

use criterion::Criterion;

use froop::Stream;

fn map_benchmark(c: &mut Criterion) {
    let sink = Stream::sink();
    let map = sink.stream().map(|x| x * 2);
    let _ = map.subscribe(|_| {});
    c.bench_function("map", move |b| b.iter(|| sink.update(42)));
}

fn imitator_benchmark(c: &mut Criterion) {
    let imitator = Stream::imitator();

    let fold = imitator
        .stream()
        .fold(1, |p, c| if *c < 10 { p + c } else { p })
        .dedupe();

    let sink = Stream::sink();

    let merge = Stream::merge(vec![fold, sink.stream()]);
    imitator.imitate(&merge);

    let _ = merge.subscribe(|_| {});

    c.bench_function("imitator", move |b| b.iter(|| sink.update(1)));
}

criterion_group!(benches, map_benchmark, imitator_benchmark);
criterion_main!(benches);
