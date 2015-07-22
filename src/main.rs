#[macro_use]
extern crate log;
extern crate mio;
extern crate fiesta_net;

use mio::*;

use fiesta_net::*;

mod logger;

struct SampleHandler;

impl PacketProcessor for SampleHandler {
	fn process_packet(&mut self, info: Box<PacketProcessingInfo>) {
		let client = info.client.borrow();

		info!(target: "handling", "packet with header {:04X} in thread '{}' (client {:?} alive: {})", 
			info.packet.header,
			std::thread::current().name().unwrap(),
			client.id(),
			client.alive());
	}

	fn clone(&self) -> Box<PacketProcessor> {
		Box::new(SampleHandler)
	}
}

fn main() {
	logger::EnvLogger::init().unwrap();
    info!(target: "main", "logging test..");

    let addr = "0.0.0.0:1080".parse().unwrap();
    let socket = mio::tcp::TcpSocket::v4().unwrap();
    socket.bind(&addr).unwrap();
    let socket = socket.listen(128).unwrap();

    info!(target: "main", "started to listen");

    let mut event_loop = EventLoop::new().unwrap();

    let handler = Box::new(SampleHandler);
    let mult_handler = PacketProcessingThreadPool::new(5, handler);

    event_loop.register(&socket, fiesta_net::SERVER_TOKEN).unwrap();
    let mut handler = fiesta_net::FiestaHandler::new(socket, Box::new(mult_handler));
    
    info!(target: "main", "starting event loop!");
    event_loop.run(&mut handler).unwrap();
}
