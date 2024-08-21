#![cfg(feature = "lttng")]

use std::task::Poll;

use lttng_ust::import_tracepoints;

use crate::tracer::{TraceFuture, TraceWaker};

import_tracepoints!(concat!(env!("OUT_DIR"), "/tracepoints.rs"), tp);

#[derive(Debug, Clone, Copy)]
pub struct LttngTracer {
    id: usize,
}

impl LttngTracer {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

impl TraceFuture for LttngTracer {
    fn before_poll(&self) {
        tp::trace_futures::before_poll(self.id);
    }

    fn after_poll(&self, res: Poll<()>) {
        let b = match res {
            Poll::Ready(_) => 1,
            Poll::Pending => 0,
        };

        tp::trace_futures::after_poll(self.id, b);
    }
}

impl TraceWaker for LttngTracer {
    fn before_wake(&self) {
        tp::trace_futures::before_wake(self.id);
    }

    fn after_wake(&self) {
        tp::trace_futures::after_wake(self.id);
    }
}
