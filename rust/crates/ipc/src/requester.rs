use zmq::{Context, Socket};

/// Extended Request Reply IPC
pub struct Requester {
    pub socket: Socket,
}

impl Requester {
    pub fn new(address: &str) -> zmq::Result<Self> {
        let ctx = Context::new();
        let socket = ctx.socket(zmq::REQ)?;

        let requester = Self { socket };
        requester.connect(address)?;

        Ok(requester)
    }

    pub fn connect(&self, address: &str) -> zmq::Result<()> {
        self.socket.connect(address)
    }
}
