mod subscriber;

use rand::Rng;
use rouille::Response;
use serde_json::json;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc};
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::Mutex;
use tokio::task;
use uuid::Uuid;
use zmq::{Context, Result, Socket};

fn rand() -> f32 {
    let mut rng = rand::thread_rng();
    let n: f32 = rng.gen_range(0f32..2f32);

    return n;
}

// fn publisher_server(socket: Arc<Mutex<Socket>>, total_requests: Arc<AtomicU64>) -> Result<()> {
//     let mut inputs = Vec::with_capacity(41);
//
//     // let start_time = SystemTime::now();
//     // let end_time = start_time + Duration::from_secs(60 * 1);
//
//     let id = Uuid::new_v4();
//
//     for _ in 0..41 {
//         inputs.push(rand());
//     }
//
//     // while SystemTime::now() < end_time {
//     let timestamp_nanos = SystemTime::now()
//         .duration_since(SystemTime::UNIX_EPOCH)
//         .unwrap()
//         .as_nanos(); // u128
//
//     let data = format!(
//         r#"{{"id": "{}", "model_inputs": {:?}, "start_execution_ns": {}}}"#,
//         id.to_string(),
//         inputs,
//         timestamp_nanos
//     );
//
//     {
//         let s = socket.lock().unwrap();
//         s.send(&data.to_string(), 0).unwrap();
//     }
//
//     total_requests.fetch_add(1, Ordering::Relaxed);
//     println!("[publisher]: {:?}", total_requests.load(Ordering::Relaxed));
//
//     // Adjust sleep time if needed
//     // tokio::time::sleep(Duration::from_millis(10)).await; // Sleep for a short duration
//     // }
//
//     Ok(())
// }

async fn publisher(socket: Arc<Mutex<Socket>>, total_requests: Arc<AtomicU64>) -> Result<()> {
    let mut inputs = Vec::with_capacity(41);

    /*let start_time = SystemTime::now();
    let end_time = start_time + Duration::from_secs(60);*/

    let id = Uuid::new_v4();

    for _ in 0..41 {
        inputs.push(rand());
    }

    for _ in 0..10_001 {
        let timestamp_nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos(); // u128

        // 1702031616471719886

        let data = format!(
            r#"{{"id": "{}", "model_inputs": {:?}, "start_execution_ns": {}}}"#,
            id.to_string(),
            inputs,
            timestamp_nanos
        );

        {
            let s = socket.lock().await;
            s.send(&data.to_string(), 0).unwrap();
        }

        total_requests.fetch_add(1, Ordering::Relaxed);
        println!("[publisher]: {:?}", total_requests.load(Ordering::Relaxed));

        // Adjust sleep time if needed
        tokio::time::sleep(Duration::from_secs(1 / 60)).await; // Sleep for a short duration
    }
    Ok(())
}

// #[tokio::main]
// async fn main() {
//     let ctx = Context::new();
//     tokio::time::sleep(Duration::from_secs(1)).await;
//
//     let socket = Arc::new(Mutex::new(ctx.socket(zmq::PUB).unwrap()));
//     let s1 = socket.clone();
//     {
//         s1.lock()
//             .unwrap()
//             .bind("ipc:///tmp/zeromq_sub_uds")
//             .unwrap();
//     }
//
//     let total_requests = Arc::new(AtomicU64::new(0));
//
//     tokio::time::sleep(Duration::from_secs(1)).await;
//     println!("Server started");
//     rouille::start_server("127.0.0.1:5000", move |_| {
//         let c = total_requests.clone();
//         publisher_server(s1.clone(), c).unwrap();
//         Response::empty_204()
//     });
// }

#[tokio::main]
async fn main() {
    let ctx = Context::new();
    println!("[publisher]: sleeping for 2s");
    tokio::time::sleep(Duration::from_secs(1)).await;

    let socket = Arc::new(Mutex::new(ctx.socket(zmq::PUB).unwrap()));
    let s1 = socket.clone();
    {
        // s1.lock().unwrap().bind("tcp://127.0.0.1:5757").unwrap();
        s1.lock()
            .await
            .bind("ipc:///tmp/zeromq_sub_uds")
            .unwrap();
    }
    let total_requests = Arc::new(AtomicU64::new(0));

    // thread::sleep(Duration::from_secs(1));
    println!("[publisher]: ready!");

    let num_threads = 850;

    let mut handles = Vec::with_capacity(num_threads);
    for _ in 0..num_threads {
        let socket_clone = Arc::clone(&socket);
        let req_clone = Arc::clone(&total_requests);
        let handle = task::spawn(async move {
            publisher(socket_clone, req_clone).await.unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Final total requests: {:?}", total_requests.load(Ordering::Relaxed));
}
