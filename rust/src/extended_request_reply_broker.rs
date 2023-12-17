use zmq::Context;

fn main() {
    let context = Context::new();
    let frontend = context.socket(zmq::ROUTER).unwrap();
    let backend = context.socket(zmq::DEALER).unwrap();

    // assert!(client.bind("tcp://*:5559").is_ok());
    // assert!(worker.bind("tcp://*:5560").is_ok());

    assert!(frontend.bind("ipc:///tmp/router").is_ok());
    assert!(backend.bind("ipc:///tmp/dealer").is_ok());

    println!("[err_broker] connected");
    zmq::proxy(&frontend, &backend).unwrap();
}
