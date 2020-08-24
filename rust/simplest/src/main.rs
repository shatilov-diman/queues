
const CONNECTION: &str = "127.0.0.1:8888";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


fn main() -> Result<()> {
	simple_logger::init_with_level(log::Level::Info)?;

	let mut settings = ws::Settings::default();
	settings.queue_size = 1000;

	ws::Builder::new()
	.with_settings(settings)
	.build(|socket: ws::Sender| {
		move |msg: ws::Message| {
			socket.broadcast(msg)
		}
	})?
	.listen(CONNECTION)?;

	log::info!("exit");
	Ok(())
}

