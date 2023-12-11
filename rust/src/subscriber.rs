use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use zmq::{Context, SUB};

fn main() {
    let ctx = Context::new();
    let subscriber = ctx.socket(SUB).expect("Failed to create subscriber");
    let conn = "ipc:///tmp/zeromq_pub_uds";
    let file_path = "metrics_udp.csv";

    subscriber
        .set_subscribe(b"")
        .expect("Failed to set subscribe option");
    subscriber
        .connect(conn)
        .expect("Failed to connect to the publisher");

    println!("[subscriber]: started {}", conn);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open file");

    writeln!(
        file,
        "id,start_execution_ns,end_execution_ns,inference_time_ns"
    )
    .expect("Failed to write header to file");

    loop {
        let s_msg = subscriber
            .recv_string(0)
            .expect("Failed to receive a message")
            .expect("Failed to parse message");

        let end_execution_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();

        println!(
            "[subscriber({})]: ACK",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );

        let msg: Value = serde_json::from_str(&s_msg).expect("Failed to parse JSON");

        if let (Some(id), Some(start_execution_ns), Some(inference_time_ns)) = (
            msg.get("id"),
            msg.get("start_execution_ns"),
            msg.get("inference_time_ns"),
        ) {
            writeln!(
                file,
                "{},{},{},{}",
                id, start_execution_ns, end_execution_ns, inference_time_ns
            )
            .expect("Failed to write to file");
        }
    }
}
