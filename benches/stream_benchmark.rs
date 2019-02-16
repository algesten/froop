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

// fn fold_benchmark(c: &mut Criterion) {
//     let sink: Sink<u32> = Sink::new();
//     let fold = sink.stream().fold(0, |prev, cur| prev + cur);
//     let _ = fold.subscribe(|_| {});
//     c.bench_function("fold", move |b| b.iter(|| sink.update(1)));
// }

criterion_group!(benches, map_benchmark); // , fold_benchmark);
criterion_main!(benches);
