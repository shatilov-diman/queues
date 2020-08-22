
use log::{info};

use websocket::client::ClientBuilder;
use websocket::{OwnedMessage};

use crossbeam_channel::unbounded;

const CONNECTION: &str = "ws://127.0.0.1:8888";

type Result<T> = std::result::Result<T, String>;

fn main() -> Result<()> {
	simple_logger::init_with_level(log::Level::Info).map_err(|e| format!("->{:?}", e))?;

	let client =
		ClientBuilder::new(CONNECTION).map_err(|e| format!("{}", e))?
		.connect_insecure().map_err(|e| format!("{}", e))?;

		let (mut receiver, mut sender) = client.split().map_err(|e| format!("{}", e))?;
		let (s, r) = unbounded();
		let ss = s.clone();

		let send_thread = std::thread::spawn(move || -> Result<()> {
			while let Ok(ref m) = r.recv() {
				match &m {
					OwnedMessage::Text(t) => info!("->{}", t),
					_ => info!("{:?}", m),
				}
				sender.send_message(m).map_err(|e| format!("->{:?}", e))?;
			}
			println!("send_thread exit");
			Ok(())
		});

		let recv_thread = std::thread::spawn(move || -> Result<()> {
			for message in receiver.incoming_messages() {
				match message {
					Err(e) => return Err(format!("{}", e)),
					Ok(OwnedMessage::Close(_)) => return Ok(()),
					Ok(OwnedMessage::Ping(data)) => s.send(OwnedMessage::Pong(data)).map_err(|e| format!("{}", e))?,
					Ok(OwnedMessage::Text(data)) => info!("<-{}", data),
					Ok(m) => info!("<-{:?}", m),
				}
			}
			println!("recv_thread exit");
			Ok(())
		});

		let gen_thread = std::thread::spawn(move || -> Result<()> {
			for i in 0..10 {
				ss.send(OwnedMessage::Text(i.to_string())).map_err(|e| format!("{}", e))?;
			}
			println!("gen_thread exit");
			Ok(())
		});

		let _ = gen_thread.join();
		let _ = recv_thread.join();
		let _ = send_thread.join();

		println!("exit");

		Ok(())
}

