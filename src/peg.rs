use std::sync::{Arc, Mutex};

type Destructor = Box<dyn Fn()>;

#[derive(Clone)]
pub struct Peg(Arc<Mutex<Option<Destructor>>>, Option<Vec<Peg>>);
pub struct Pegged<P>(Arc<Mutex<Option<P>>>);

unsafe impl Send for Peg {}
unsafe impl Sync for Peg {}
unsafe impl<P> Send for Pegged<P> {}
unsafe impl<P> Sync for Pegged<P> {}

impl<P: 'static> Pegged<P> {
    pub fn new_pair(p: P) -> (Pegged<P>, Peg) {
        let wrap = Arc::new(Mutex::new(Some(p)));
        let clone = wrap.clone();
        let destroy = move || {
            let mut lock = clone.lock().unwrap();
            lock.take();
        };
        let destructor: Destructor = Box::new(destroy);
        (
            Pegged(wrap),
            Peg(Arc::new(Mutex::new(Some(destructor))), None),
        )
    }

    pub fn with_value<R>(&self, mut f: impl FnMut(Option<&mut P>) -> R) -> R {
        let mut lock = self.0.lock().unwrap();
        f(lock.as_mut())
    }
}

impl Peg {
    pub fn new_fake() -> Peg {
        Peg(Arc::new(Mutex::new(None)), None)
    }

    pub fn many(pegs: Vec<Peg>) -> Peg {
        Peg(Arc::new(Mutex::new(None)), Some(pegs))
    }

    pub fn add_related(&mut self, peg: Peg) {
        if let Some(v) = self.1.as_mut() {
            v.push(peg);
        } else {
            self.1 = Some(vec![peg]);
        }
    }

    pub fn keep_mode(&self) {
        let mut lock = self.0.lock().unwrap();
        lock.take(); // just remove destructor without running it
    }

    pub fn unpeg(&self) {
        let mut lock = self.0.lock().unwrap();
        if let Some(destruct) = lock.take() {
            destruct()
        }
    }
}

impl Drop for Peg {
    fn drop(&mut self) {
        if Arc::strong_count(&self.0) == 1 {
            // this is the last peg to drop.
            self.unpeg();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_peg_no_keep() {
        let (pegged, peg) = Pegged::new_pair(());
        {
            let lock = pegged.0.lock().unwrap();
            assert!(lock.is_some());
        }
        drop(peg);
        {
            let lock = pegged.0.lock().unwrap();
            assert!(lock.is_none());
        }
    }

    #[test]
    pub fn test_peg_keep_mode() {
        let (pegged, peg) = Pegged::new_pair(());
        {
            let lock = pegged.0.lock().unwrap();
            assert!(lock.is_some());
        }
        peg.keep_mode();
        drop(peg);
        {
            let lock = pegged.0.lock().unwrap();
            assert!(lock.is_some());
        }
    }
}
