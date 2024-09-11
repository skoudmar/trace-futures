# Trace futures

Trace the execution progress of a future or a stream.

## Usage

See the [examples](examples) directory for more examples.

### Printing to stderr

Basic printing of the progress of a future:

```rust
use trace_futures::print_traced;

async fn async_fn() {
    // Your async code here
}

async fn main() {
    let future = print_traced(async_fn(), "name in the log");;

    future.await;
}
```

### Emitting lttng events

First enable the `lttng` feature in your `Cargo.toml`.

Then you can use the `lttng_tracer` module to emit lttng events.

```rust
use trace_futures::lttng_tracer::LttngTracer;
use trace_futures::future::TraceWithWaker;

async fn async_fn() {
    // Your async code here
}

async fn main() {
    // LttngTracer::new() takes a usize as the tracepoint id.
    let future = TraceWithWaker::new(async_fn(), LttngTracer::new(42));

    future.await;
}
```
