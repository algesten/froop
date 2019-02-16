use std::cell::RefCell;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::peg::Peg;
use crate::sub::Listeners;

thread_local!(pub static IMITATORS: RefCell<Vec<Box<dyn FnMut()>>> = RefCell::new(vec![]));

pub struct SafeInner<T: 'static>(Arc<Mutex<Inner<T>>>);

impl<T> SafeInner<T> {
    pub(crate) fn new(memory_mode: MemoryMode, memory: Option<T>) -> Self {
        SafeInner(Arc::new(Mutex::new(Inner::new(memory_mode, memory))))
    }
    pub(crate) fn lock<'a>(&'a self) -> MutexGuard<'a, Inner<T>> {
        self.0.lock().unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum MemoryMode {
    NoMemory,
    KeepUntilEnd,
    KeepAfterEnd,
}

impl MemoryMode {
    pub fn is_memory(self) -> bool {
        match self {
            MemoryMode::NoMemory => false,
            _ => true,
        }
    }
}

pub(crate) struct Inner<T: 'static> {
    alive: bool,
    listeners: Listeners<T>,
    memory_mode: MemoryMode,
    memory: Option<T>,
}

impl<T> Inner<T> {
    pub fn new(memory_mode: MemoryMode, memory: Option<T>) -> Self {
        Inner {
            alive: true,
            listeners: Listeners::new(),
            memory_mode,
            memory,
        }
    }

    pub fn memory_mode(&self) -> MemoryMode {
        self.memory_mode
    }

    pub fn add<F: FnMut(Option<&T>) + 'static>(&mut self, mut listener: F) -> Peg {
        if !self.alive {
            listener(None);
            self.run_imitators();
            return Peg::new_fake();
        }
        if self.memory_mode.is_memory() {
            if let Some(v) = self.memory.as_ref() {
                listener(Some(v));
                self.run_imitators();
            }
        }
        self.listeners.add(listener)
    }

    pub fn update_and_imitate(&mut self, t: Option<T>) {
        if !self.alive {
            return;
        }
        self.update_owned(t);
        self.run_imitators();
    }

    fn run_imitators(&self) {
        loop {
            let mut imit = vec![];
            IMITATORS.with(|imit_cell| {
                imit = imit_cell.borrow_mut().split_off(0);
            });
            if imit.is_empty() {
                break;
            }
            for mut i in imit {
                i();
            }
        }
    }

    pub fn update_owned(&mut self, t: Option<T>) {
        if !self.alive {
            return;
        }
        self.listeners.iter(|l| l(t.as_ref()));
        let is_end = t.is_none();
        match self.memory_mode {
            MemoryMode::NoMemory => (),
            MemoryMode::KeepUntilEnd => {
                self.memory = t;
            }
            MemoryMode::KeepAfterEnd => {
                if t.is_some() {
                    self.memory = t;
                }
            }
        }
        if is_end {
            self.end();
        }
    }

    pub fn update_borrowed(&mut self, t: Option<&T>) {
        if !self.alive {
            return;
        }
        self.listeners.iter(|l| l(t));
        let is_end = t.is_none();
        if is_end {
            self.end();
        }
    }

    pub fn take_memory(&mut self) -> Option<T> {
        self.memory.take()
    }

    pub fn peek_memory(&self) -> &Option<T> {
        &self.memory
    }

    fn end(&mut self) {
        self.alive = false;
        self.listeners.clear();
    }
}

impl<T> Clone for SafeInner<T> {
    fn clone(&self) -> Self {
        SafeInner(self.0.clone())
    }
}
