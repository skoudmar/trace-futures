use std::{fmt::Display, future::ready};

use futures::{
    channel::mpsc::{self, Sender},
    SinkExt, StreamExt,
};
use trace_futures::{
    future::{TraceWithWaker, Traced},
    stream::TracedStream,
    tracer::PrintTracer,
};

async fn send<T: Display + Copy, It: IntoIterator<Item = T>>(mut sender: Sender<T>, iter: It) {
    for item in iter {
        TraceWithWaker::new(sender.send(item), PrintTracer::from(format!("Send {item}")))
            .await
            .unwrap();
    }

    println!("Send complete");
}

fn trace_stream<S: futures::Stream>(
    stream: S,
    name: &'static str,
) -> TracedStream<S, PrintTracer<&'static str>> {
    TracedStream::new(stream, PrintTracer::from(name))
}

fn main() {
    let (sender, receiver) = mpsc::channel(1);

    let receiver_stream = trace_stream(receiver, "Receiver");

    let combined = async move {
        let sender1 = send(sender.clone(), 0..10);
        let sender2 = send(sender, 10..20);
        let senders = TraceWithWaker::new(
            futures::future::join(sender1, sender2),
            PrintTracer::from("Senders"),
        );

        let receive_fut = receiver_stream.for_each(|item| {
            eprintln!("Received {item}");

            ready(())
        });

        let receive_fut = Traced::new(receive_fut, PrintTracer::from("Receive for_each"));

        futures::future::join(senders, receive_fut).await;
    };

    futures::executor::block_on(combined);
}
