
use log::{error, info, debug};

use websocket::client::ClientBuilder;
use websocket::{OwnedMessage};

const CONNECTION: &str = "ws://127.0.0.1:8888";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
	simple_logger::init_with_level(log::Level::Error)?;

	let client =
		ClientBuilder::new(CONNECTION)?
		.connect_insecure()?;

		let (mut receiver, mut sender) = client.split()?;

		let max_messages = 1000000;

		let send_thread = std::thread::spawn(move || {
			for i in 0..max_messages {
				if let Err(e) = sender.send_message(&OwnedMessage::Text(i.to_string())) {
					error!("{}", e);
					break;
				}
			}
			debug!("send_thread exit");
		});

		let recv_thread = std::thread::spawn(move || {
			for message in receiver.incoming_messages().take(max_messages) {
				match message {
					Ok(OwnedMessage::Close(_)) => break,
					Ok(OwnedMessage::Text(data)) => info!("{}", data),
					Ok(m) => info!("{:?}", m),
					Err(e) => {
						error!("{}", e);
						break;
					},
				}
			}
			debug!("recv_thread exit");
		});

		let _ = recv_thread.join();
		let _ = send_thread.join();

		println!("exit");

		Ok(())
}

