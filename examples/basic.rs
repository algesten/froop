use froop::{Sink, Stream};

fn main() {
    // A sink is an originator of events that form a stream.
    let sink: Sink<u32> = Stream::sink();

    // Map the even numbers to their square.
    let stream: Stream<u32> = sink.stream().filter(|i| i % 2 == 0).map(|i| i * i);

    // Print the result
    stream.subscribe(|i| {
        if let Some(i) = i {
            println!("{}", i)
        }
    });

    // Send numbers into the sink.
    for i in 0..10 {
        sink.update(i);
    }
    sink.end();
}
