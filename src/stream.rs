use std::{
    sync::Arc,
    task::{Context, Poll},
};

use futures::Stream;
use pin_project_lite::pin_project;

use crate::{tracer::TraceWaker, waker::MyWaker};

pin_project! {
    pub struct TracedStream<S, T> {
        #[pin]
        stream: S,
        tracer: T,
        waker: Option<Arc<MyWaker<T>>>,
    }
}

impl<S: Stream, T: TraceWaker> TracedStream<S, T> {
    pub fn new(stream: S, tracer: T) -> Self {
        Self {
            stream,
            tracer,
            waker: None,
        }
    }
}

impl<S: Stream, T: TraceWaker> Stream for TracedStream<S, T> {
    type Item = S::Item;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let waker = this
            .waker
            .get_or_insert_with(|| MyWaker::new(cx.waker().clone(), this.tracer.clone()));

        waker.reregister(cx.waker());
        let waker = waker.clone().into();

        let mut context = Context::from_waker(&waker);

        this.tracer.before_poll();
        let result = this.stream.poll_next(&mut context);

        match result {
            Poll::Ready(_) => this.tracer.after_poll(Poll::Ready(())),
            Poll::Pending => this.tracer.after_poll(Poll::Pending),
        }

        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}
