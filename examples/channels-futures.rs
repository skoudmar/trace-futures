use std::{fmt::Display, future::ready};

use futures::{
    channel::mpsc::{self, Sender},
    SinkExt, StreamExt,
};
use trace_futures::{future::TraceWithWaker, future::Traced, tracer::PrintTracer};

async fn send<T: Display + Copy, It: IntoIterator<Item = T>>(mut sender: Sender<T>, iter: It) {
    for item in iter {
        TraceWithWaker::new(sender.send(item), PrintTracer::from(format!("Send {item}")))
            .await
            .unwrap();
    }

    println!("Send complete");
}

fn main() {
    let (sender, receiver) = mpsc::channel(1);

    let combined = async move {
        let sender1 = send(sender.clone(), 0..10);
        let sender2 = send(sender, 10..20);
        let senders = TraceWithWaker::new(
            futures::future::join(sender1, sender2),
            PrintTracer::from("Senders"),
        );

        let receive_fut = receiver.for_each(|item| {
            println!("Received {item}");
            Traced::new(
                ready(()),
                PrintTracer::from(format!("Receive ready {item}")),
            )
        });

        let receive_fut = Traced::new(receive_fut, PrintTracer::from("Receive for_each"));

        futures::future::join(senders, receive_fut).await;
    };

    futures::executor::block_on(combined);
}
