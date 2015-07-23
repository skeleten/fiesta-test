#[macro_use]
extern crate log;
extern crate mio;
extern crate fiesta_net;
extern crate rand;

use std::sync::{Arc, RwLock};
use std::io::Write;
use mio::*;
use fiesta_net::*;
use rand::Rng;

mod logger;

struct SampleHandler {
	packets_processed:		Arc<RwLock<usize>>,
}

impl SampleHandler {
	pub fn new() -> SampleHandler {
		let arc = Arc::new(RwLock::new(0));
		let handler = SampleHandler {
			packets_processed:		arc.clone(),
		};
		std::thread::spawn(move || {
			let mut last = 0;
			let mut rate = 0;
			let mut max_rate = 0;
			loop {
				std::thread::sleep_ms(1000);
				{
					let guard = arc.read().unwrap();
					{
						rate = ((*guard) - last);
						last = *guard;
					};
					if rate > max_rate {
						max_rate = rate;
					};
					print!("processed {:10} packets! rate: {:10} max: {:10}\r", *guard, rate, max_rate);
					std::io::stdout().flush();
				}
			}
		});

		handler
	}
}

impl PacketProcessor for SampleHandler {
	fn process_packet(&mut self, info: Arc<RwLock<Box<PacketProcessingInfo>>>) {
		let mut rng = rand::thread_rng();
		let sleepms = rng.gen_range(100, 1000);
		// std::thread::sleep_ms(sleepms);
		let mut guard = self.packets_processed.write().unwrap();
		*guard = (*guard) + 1;
	}

	fn clone(&self) -> Box<PacketProcessor> {
		Box::new(SampleHandler {
			packets_processed: 		self.packets_processed.clone(),
		})
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

    let handler = Box::new(SampleHandler::new());
    let mult_handler = PacketProcessingThreadPool::new(20, handler);

    event_loop.register(&socket, fiesta_net::SERVER_TOKEN).unwrap();
    let mut handler = fiesta_net::FiestaHandler::new(socket, Box::new(mult_handler));
    
    info!(target: "main", "starting event loop!");
    event_loop.run(&mut handler).unwrap();
}
