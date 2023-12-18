extern crate flatbuffers;

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fs::File, io::Write, time::Instant};

use clap::{Parser, Subcommand};
use flatbuffers::FlatBufferBuilder;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use zmq::Context;

use schema::schema_generated::{AntiFraudInputBuilder, AntiFraudResponse};

const DEFAULT_ROUTER_ADDRESS: &str = "ipc:///tmp/router";
const DEFAULT_LOOP_COUNT: u64 = 100;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    /// Number of requests to send to the router socket
    count: Option<u64>,

    #[arg(short, long)]
    /// Address where to send messages
    router_address: Option<String>,

    #[command(subcommand)]
    request_config: Option<Config>,
}

#[derive(Subcommand)]
enum Config {
    Request {
        #[arg(short, long)]
        /// Sleep time in ms before sending next request
        sleep: u64,

        #[arg(short, long)]
        /// Number of batch request to send and then sleep
        batch: Option<u64>,
    },
}

fn random_model_inputs() -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..41).map(|_| rng.gen_range(0f64..2f64)).collect()
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

fn setup_requester<'a>(context: Arc<Context>, address: &'a str) -> zmq::Socket {
    let requester = context
        .socket(zmq::REQ)
        .expect("Failed to create REQ socket");
    requester.connect(&address).expect("Failed to connect");
    requester
}

fn process_request(
    context: Arc<Context>,
    router_address: &str,
    file: &Mutex<File>,
    request_nbr: u64,
) -> Result<(), Box<dyn Error>> {
    let binding = router_address.to_string();
    let requester = setup_requester(context.clone(), &binding);
    let start = Instant::now();

    let binding = serialize_data(); // Assuming this returns a Result
    let data = binding.finished_data();
    requester.send(data, zmq::DONTWAIT)?;

    let msg = requester.recv_msg(0)?;
    let data = msg.as_ref();
    let _ = flatbuffers::root::<AntiFraudResponse>(data)?; // Handle this result as needed

    let elapsed = start.elapsed().as_nanos();
    writeln!(file.lock().unwrap(), "{},{}", request_nbr, elapsed)?;

    println!("Received reply {} in {:?} ns", request_nbr, elapsed);
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let count = cli.count.unwrap_or(DEFAULT_LOOP_COUNT);
    let binding = DEFAULT_ROUTER_ADDRESS.to_string();
    let router_address = Arc::new(cli.router_address.unwrap_or(binding));
    let request_config = cli.request_config;

    println!("Loop count: {}", count);

    let context = Arc::new(Context::new()); // Shared ZMQ context
    println!("[client]: connected to {}", router_address);

    let file = Arc::new(Mutex::new(
        File::create(format!("extended_request_reply_{}.csv", count)).unwrap(),
    ));
    writeln!(&mut file.lock().unwrap(), "id,elapsed_time_ns").expect("Failed to write header");

    match request_config {
        None => {
            (0..count).into_par_iter().for_each_with(
                (context, file),
                |(context, file), request_nbr| {
                    if let Err(e) =
                        process_request(context.clone(), &router_address, &file, request_nbr)
                    {
                        eprintln!("Failed to process request {}: {}", request_nbr, e);
                    }
                },
            );
        }
        Some(Config::Request { sleep, batch }) => {
            let default_batch_counter = batch.unwrap_or(10);
            let mut batch_counter = default_batch_counter;
            println!("[client]: sleep={}ms batch={} ", sleep, batch_counter);

            (0..count).into_iter().for_each(|request_nbr| {
                if batch_counter == 0 {
                    std::thread::sleep(Duration::from_millis(sleep));
                    batch_counter = default_batch_counter;
                } else {
                    batch_counter -= 1;
                }

                if let Err(e) =
                    process_request(context.clone(), &router_address, &file, request_nbr)
                {
                    eprintln!("Failed to process request {}: {}", request_nbr, e);
                }
            })
        }
    }
}
