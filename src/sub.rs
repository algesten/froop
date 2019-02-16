use crate::peg::{Peg, Pegged};

#[doc(hidden)]
#[derive(Default)]
pub struct Listeners<T: 'static> {
    ls: Vec<Pegged<Listener<T>>>,
}

#[doc(hidden)]
pub type Listener<T> = Box<dyn FnMut(Option<&T>)>;

impl<T> Listeners<T> {
    pub fn new() -> Self {
        Listeners { ls: vec![] }
    }

    /// Add a new listener.
    pub fn add<F: FnMut(Option<&T>) + 'static>(&mut self, listener: F) -> Peg {
        let boxed: Listener<T> = Box::new(listener);
        let (val, peg) = Pegged::new_pair(boxed);
        self.ls.push(val);
        peg
    }

    pub fn clear(&mut self) {
        self.ls.clear();
    }

    /// Iterate over listeners and receive them one by one in a callback.
    pub fn iter(&mut self, mut cb: impl FnMut(&mut Listener<T>)) {
        self.ls.retain(|p| {
            p.with_value(|v| {
                if let Some(v) = v {
                    cb(v);
                    true
                } else {
                    false
                }
            })
        });
    }
}

/// A subscription is a receipt for adding a listener to a stream. Can be used to stop listening.
#[derive(Clone)]
pub struct Subscription {
    peg: Peg,
}

impl Subscription {
    pub(crate) fn new(peg: Peg) -> Self {
        Subscription { peg }
    }

    /// Stops listening to the stream.
    pub fn unsubscribe(&self) {
        self.peg.unpeg()
    }
}
