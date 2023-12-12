extern crate flatbuffers;
mod extended_request_reply_broker;
mod schema_generated;

use crate::schema_generated::{AntiFraudInputBuilder, AntiFraudResponse};
use flatbuffers::FlatBufferBuilder;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::{Arc, Mutex};
use std::{env, fs::File, io::Write, time::Instant};
use zmq::Context;

const ROUTER_ENDPOINT: &str = "ipc:///tmp/router";
const DEFAULT_LOOP_COUNT: u64 = 100;
const DEFAULT_THREADS: usize = 25;

fn random_model_inputs() -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..41).map(|_| rng.gen_range(0f32..2f32)).collect()
}

fn serialize_data() -> FlatBufferBuilder<'static> {
    let model_inputs = random_model_inputs();
    let mut builder = FlatBufferBuilder::with_capacity(1024); // Arbitrary capacity

    let inputs = builder.create_vector(&model_inputs);
    let mut af_input_builder = AntiFraudInputBuilder::new(&mut builder);
    af_input_builder.add_inputs(inputs);

    let af_input = af_input_builder.finish();
    builder.finish(af_input, None);

    builder
}

fn setup_requester(context: Arc<Context>) -> zmq::Socket {
    let requester = context
        .socket(zmq::REQ)
        .expect("Failed to create REQ socket");
    requester
        .connect(ROUTER_ENDPOINT)
        .expect("Failed to connect");
    requester
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let count = args
        .get(1)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(DEFAULT_LOOP_COUNT);

    println!("Loop count: {}", count);

    let context = Arc::new(Context::new()); // Shared ZMQ context
    println!("[client]: connected to {}", ROUTER_ENDPOINT);

    let file = Arc::new(Mutex::new(
        File::create(format!("extended_request_reply_{}.csv", count)).unwrap(),
    ));
    writeln!(file.lock().unwrap(), "id,elapsed_time_ns").expect("Failed to write header");

    let mut file = Arc::new(Mutex::new(
        File::create(format!("extended_request_reply_{}.csv", count)).unwrap(),
    ));
    writeln!(&mut file.lock().unwrap(), "id,elapsed_time_ns").expect("Failed to write header");

    (0..count)
        .into_par_iter()
        .for_each_with((context, file), |(context, file), request_nbr| {
            let requester = setup_requester(context.clone());
            let start = Instant::now();

            let binding = serialize_data();
            let data = binding.finished_data();
            requester.send(data, zmq::DONTWAIT).unwrap();

            let msg = requester.recv_msg(0).unwrap();
            let data = msg.as_ref();
            let af_response = flatbuffers::root::<AntiFraudResponse>(data).unwrap();

            let elapsed = start.elapsed().as_nanos();
            writeln!(&mut file.lock().unwrap(), "{},{}", request_nbr, elapsed)
                .expect("Failed to write data");

            println!(
                "Received reply {} in {:?} ns - Response: {:?}",
                request_nbr,
                elapsed,
                af_response.response()
            );
        });
}
