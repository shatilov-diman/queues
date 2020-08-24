
use log::{error, info, debug};

use ws::connect;

const CONNECTION: &str = "ws://127.0.0.1:8888";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


struct WsFnMut<H> where
	H: FnMut(ws::Message) -> std::result::Result<(), ws::Error>
{
	h: H,
}

impl<H> WsFnMut<H> where
	H: FnMut(ws::Message) -> std::result::Result<(), ws::Error>
{
	fn new(h: H) -> Self {
		return Self {
			h,
		}
	}
}

impl<H> ws::Handler for WsFnMut<H> where
	H: FnMut(ws::Message) -> std::result::Result<(), ws::Error>
{
    fn on_message(&mut self, msg: ws::Message) -> std::result::Result<(), ws::Error> {
        (self.h)(msg)
    }
}

fn main() -> Result<()> {
	simple_logger::init_with_level(log::Level::Info)?;

	let max_messages = 1_000_000;

	connect(CONNECTION, |socket| {

		let s = socket.clone();
		std::thread::spawn(move || {
			for i in 0..max_messages {
				if let Err(e) = s.send(i.to_string()) {
					error!("{}", e);
					break;
				}
			}
			info!("send_thread exit");
		});

		let mut received_messages = std::rc::Rc::<u32>::new(0);

		WsFnMut::new(move |_msg| {
			let counter = *received_messages + 1;
			*std::rc::Rc::get_mut(&mut received_messages).unwrap() = counter;
			if counter < max_messages {
				Ok(())
			} else {
				socket.close(ws::CloseCode::Normal)
			}
		})
	})?;

	println!("exit");
	Ok(())
}

