#[cfg(not(feature = "lttng"))]
fn main() {}

#[cfg(feature = "lttng")]
fn main() {
    use std::env;
    use std::path::PathBuf;

    use lttng_ust_generate::{create_tracepoints, CIntegerType, CTFType, Generator, Provider};

    let mut provider = Provider::new("trace_futures");
    create_tracepoints!(
        in provider {
            fn before_poll(id: CTFType::Integer(CIntegerType::USize));
            fn after_poll(id: CTFType::Integer(CIntegerType::USize), res: CTFType::Integer(CIntegerType::U8));
            fn before_wake(id: CTFType::Integer(CIntegerType::USize));
            fn after_wake(id: CTFType::Integer(CIntegerType::USize));
        }
    );

    Generator::default()
        .generated_lib_name("trace_futures_lttng_ust")
        .register_provider(provider)
        .output_file_name(PathBuf::from(env::var("OUT_DIR").unwrap()).join("tracepoints.rs"))
        .generate()
        .expect("Unable to generate LTTng tracepoint bindings for crate tracefutures");
}
