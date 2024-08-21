use std::{
    future::{Future, IntoFuture},
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use crate::{
    tracer::{TraceFuture, TraceWaker},
    waker::MyWaker,
};
use pin_project_lite::pin_project;

pin_project! {
    pub struct Traced<Fut, Tracer> {
        #[pin]
        future: Fut,
        tracer: Tracer,
    }
}

impl<Fut, Tracer: TraceFuture> Traced<Fut, Tracer> {
    pub fn new<IntoFut>(future: IntoFut, tracer: Tracer) -> Self
    where
        IntoFut: IntoFuture<IntoFuture = Fut>,
    {
        Self {
            future: future.into_future(),
            tracer,
        }
    }
}

impl<Fut: Future, Tracer: TraceFuture> Future for Traced<Fut, Tracer> {
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        this.tracer.before_poll();
        let res = this.future.poll(cx);
        match &res {
            Poll::Ready(_) => this.tracer.after_poll(Poll::Ready(())),
            Poll::Pending => this.tracer.after_poll(Poll::Pending),
        }

        res
    }
}

pin_project! {
    pub struct TraceWithWaker<Fut, Tracer> {
        #[pin]
        future: Traced<Fut, Tracer>,
        waker: Option<Arc<MyWaker<Tracer>>>,
    }
}

impl<Fut, Tracer: TraceFuture> TraceWithWaker<Fut, Tracer> {
    pub fn new<IntoFut>(future: IntoFut, tracer: Tracer) -> Self
    where
        IntoFut: IntoFuture<IntoFuture = Fut>,
    {
        Self {
            future: Traced::new(future, tracer),
            waker: None,
        }
    }
}

impl<Fut: Future, Tracer: TraceWaker> Future for TraceWithWaker<Fut, Tracer> {
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let waker = this
            .waker
            .get_or_insert_with(|| MyWaker::new(cx.waker().clone(), this.future.tracer.clone()));

        waker.reregister(cx.waker());
        let waker = waker.clone().into();

        let mut context = Context::from_waker(&waker);
        this.future.poll(&mut context)
    }
}
