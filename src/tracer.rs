use std::{fmt::Display, task::Poll};

pub trait TraceFuture: Unpin {
    fn before_poll(&self);
    fn after_poll(&self, res: Poll<()>);
}

pub trait TraceWaker: TraceFuture + Clone + Send + Sync + 'static {
    fn before_wake(&self);
    fn after_wake(&self);
}

#[derive(Copy, Clone)]
pub struct PrintTracer<S>(S);

impl<S: Display + Unpin> TraceFuture for PrintTracer<S> {
    fn before_poll(&self) {
        eprintln!("[PrintTracer] polling '{}'", self.0);
    }

    fn after_poll(&self, res: Poll<()>) {
        match res {
            Poll::Ready(_) => eprintln!("[PrintTracer] polling '{}' done", self.0),
            Poll::Pending => eprintln!("[PrintTracer] polling '{}' still pending", self.0),
        }
    }
}

impl<S: Display + Clone + Unpin + Send + Sync + 'static> TraceWaker for PrintTracer<S> {
    fn before_wake(&self) {
        eprintln!("[PrintTracer] waking '{}' begin", self.0);
    }

    fn after_wake(&self) {
        eprintln!("[PrintTracer] waking '{}' done", self.0);
    }
}

impl<T: Display> From<T> for PrintTracer<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
