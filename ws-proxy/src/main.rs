use simple_logger::SimpleLogger;

fn main() {
		SimpleLogger::new().init().unwrap();
    log::warn!("Warning! Idk what tbh.");
}
