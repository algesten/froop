//

use crate::inner::SafeInner;
use crate::peg::Peg;
use crate::{MemoryMode, Stream, Subscription};

/// Imitators are used to create cyclic streams. Created by `Stream::imitator()`.
///
pub struct Imitator<T: 'static> {
    inner: SafeInner<T>,
}

impl<T: Clone> Imitator<T> {
    pub(crate) fn new() -> Self {
        Imitator {
            inner: SafeInner::new(MemoryMode::NoMemory, None),
        }
    }

    /// Start imitating another stream. This consumes the imitator since it can only
    /// imitate one other stream.
    pub fn imitate(self, other: &Stream<T>) -> Subscription {
        let peg = other.imitate(self.inner);
        peg.keep_mode();
        Subscription::new(peg)
    }

    /// Get a stream of events from this imitator. One stream instance is created for each call,
    /// and they all receive the events from the imitated stream.
    ///
    /// ```
    /// let imitator = froop::Stream::imitator();
    ///
    /// let coll1 = imitator.stream().collect();
    /// let coll2 = imitator.stream().collect();
    ///
    /// let sink = froop::Stream::sink();
    /// let stream = sink.stream();
    ///
    /// imitator.imitate(&stream);
    ///
    /// sink.update(42);
    /// sink.end(); // imitator also ends here
    ///
    /// assert_eq!(coll1.wait(), vec![42]);
    /// assert_eq!(coll2.wait(), vec![42]);
    /// ```
    pub fn stream(&self) -> Stream<T> {
        Stream {
            peg: Peg::new_fake(),
            inner: self.inner.clone(),
        }
    }
}
