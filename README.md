# Froop

A functional reactive stream library for rust.

* Small (~20 operations)
* Synchronous
* No dependencies
* Is FRP (ha!)

Modelled on André Staltz' javascript library [xstream][xstrem] which nicely distills
the ideas of [reactive extensions (Rx)][reactx] down to the essential minimum.

This library is not FRP (Functional Reactive Programming) in the way it was
defined by Conal Elliot, but as a paradigm that is both functional and reactive.
[Why I cannot say FRP but I just did][notfrp].

[xstrem]: https://github.com/staltz/xstream
[reactx]: http://reactivex.io
[notfrp]: https://medium.com/@andrestaltz/why-i-cannot-say-frp-but-i-just-did-d5ffaa23973b

## Example

```
use froop::{Sink, Stream};

// A sink is an originator of events that form a stream.
let sink: Sink<u32> = Stream::sink();

// Map the even numbers to their square.
let stream: Stream<u32> = sink.stream()
    .filter(|i| i % 2 == 0)
    .map(|i| i * i);

// Print the result
stream.subscribe(|i| if let Some(i) = i {
    println!("{}", i)
});

// Send numbers into the sink.
for i in 0..10 {
    sink.update(i);
}
sink.end();
```

# Idea

Functional Reactive Programming is a good foundation for functional programming (FP).
The step-by-step approach of composing interlocked operations, is a relatively
easy way to make an FP structure to a piece of software.

## Synchronous

Libraries that deals with streams as values-over-time (or events) often conflate the
idea of moving data from point A to B, with the operators that transform the data. The
result is that the library must deal with queues of data, queue lengths and backpressure.

_Froop has no queues_

Every `Sink::update()` of data into the tree of operations executes synchronously. Froop
has no operators that dispatches "later", i.e. no `delay()` or other time shifting
operations.

That also means froop also has no internal threads, futures or otherwise.

## Thread safe

Every part of the froop tree is thread safe. You can move a `Sink` into another thread,
or subscribe and propagate on a UI main thread. The thread that calls `Sink::update()` is
the thread executing the entire tree.

That safety comes at a cost, froop is not a zero cost abstraction library. Every part of
the tree is protected by a mutex lock. This is fine for most applications since a lock
without contention is not much overhead in the execution. But if you plan on having
lots of threads simultaneously updating many values into the tree, you might
experience a performance hit due to lock contention.

## Be out of your way

Froop tries to impose a minimum of cognitive load when using it.

* Every operator is an `FnMut(&T)` to make it the most usable possible.
* Not require `Sync` and/or `Send` on operator functions.
* Froop stream instances themselves are `Sync` and `Send`.
* Impose a minimum of constraints the event value `T`.
