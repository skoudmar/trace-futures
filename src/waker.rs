use std::{
    sync::{Arc, Mutex},
    task::{Wake, Waker},
};

use crate::tracer::TraceWaker;

pub(crate) struct MyWaker<Tracer> {
    waker: Mutex<std::task::Waker>,
    tracer: Tracer,
}

impl<Tracer> MyWaker<Tracer> {
    pub(crate) fn reregister(&self, waker: &Waker) {
        let mut self_waker = self.waker.lock().unwrap();
        if !waker.will_wake(&waker) {
            *self_waker = waker.clone();
        }
    }

    pub(crate) fn new(waker: Waker, id: Tracer) -> Arc<Self> {
        Arc::new(Self {
            waker: Mutex::new(waker),
            tracer: id,
        })
    }
}

impl<Tracer: TraceWaker> Wake for MyWaker<Tracer> {
    fn wake(self: Arc<Self>) {
        self.tracer.before_wake();
        self.waker.lock().unwrap().wake_by_ref();
        self.tracer.after_wake();
    }
}
