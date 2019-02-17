//

use crate::inner::SafeInner;
use crate::peg::Peg;
use crate::{MemoryMode, Stream, Subscription};

/// Imitators are used to create cyclic streams. Created by
/// [`Stream::imitator()`](struct.Stream.html#method.imitator).
///
/// ## Anatomy of an FRP app
///
/// [cycle.js](https://cycle.js.org) taught me one one way to structure an app built on FRP.
/// A key ingredient are "cyclic streams".
///
/// * A _driver_ is something isolating communcation to the rest of thew world. This could be
///   requests against an API, database or a UI.
/// * Drivers are two-way communications.
/// * An app is a _main function_ that have driver input as argument and driver output as
///   return value.
/// * App input is called _sources_.
/// * App output is called _sinks_.
///
/// These components would be in different files, but here's a distilled example:
/// ```
/// use froop::{Stream, Sink};
///
/// enum DriverIn {
///   // ... event type of input _from_ the driver.
/// }
/// #[derive(Clone)] // needed for imitate
/// enum DriverOut {
///   // ... event type of output _to_ the driver.
/// }
///
/// // The driver, which also is a function.
/// fn my_driver(out: Stream<DriverOut>) -> Stream<DriverIn> {
///     let sink = Stream::sink();
///
///     // React to input from `out` and
///     // produce output into `sink`.
///
///     sink.stream()
/// }
///
/// // Input to the app_main from the drivers
/// struct AppSources {
///     my_driver: Stream<DriverIn>,
/// }
///
/// // Output from the app_main to the drivers
/// struct AppSinks {
///     my_driver: Stream<DriverOut>,
/// }
///
/// // The main function of the app
/// fn app_main(sources: AppSources) -> AppSinks {
///
///     // React to input from the sources, and derive
///     // output to the sinks. Produce an "app state".
///
///     // This is just to make it compile. The output
///     // would be derived from the app state.
///     let my_driver_out = Stream::never();
///
///     return AppSinks {
///         my_driver: my_driver_out,
///     }
/// }
///
/// // This function does what "cycle.js run" does (but in
/// // javascript it can be dynamic).
/// fn run() {
///     // imitator to cycle back output from main
///     let driver_out = Stream::imitator();
///
///     // connect driver
///     let driver_in = my_driver(driver_out.stream());
///
///     // create sources for app main function
///     let app_sources = AppSources {
///         my_driver: driver_in,
///     };
///
///     // run main function
///     let app_sinks = app_main(app_sources);
///
///     // cycle back output to driver
///     driver_out.imitate(&app_sinks.my_driver);
/// }
/// ```
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
