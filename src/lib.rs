use std::{fmt::Display, future::Future};

use future::TraceWithWaker;
use tracer::{PrintTracer, TraceWaker};

pub mod future;
pub mod stream;
pub mod tracer;
pub(crate) mod waker;

#[cfg(feature = "lttng")]
pub mod lttng_tracer;

pub fn print_traced<F, S>(future: F, name: S) -> TraceWithWaker<F, PrintTracer<S>>
where
    F: Future,
    S: Display + Clone + Unpin + Send + Sync + 'static,
{
    TraceWithWaker::new(future, PrintTracer::from(name))
}

pub trait FutureExtTraced {
    fn trace_using<T>(self, tracer: T) -> TraceWithWaker<Self, T>
    where
        Self: Future + Sized,
        T: TraceWaker,
    {
        TraceWithWaker::new(self, tracer)
    }
}

impl<F> FutureExtTraced for F where F: Future {}
