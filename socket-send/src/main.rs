use std::io::prelude::*;
use std::net::TcpStream;

fn usage() {
	println!("Usage: socket-send <hostname> <data to send>");
}

fn main() -> std::io::Result<()> {
	if std::env::args().len() < 3 {
		usage();
		return Ok(());
	}	
	let args: Vec<String> = std::env::args().collect();
	
	let address = &args[1];
	let data = &args[2];
	
	println!("Opening socket to {:?} and writing data {:?}", address, data);
	
	let mut stream = TcpStream::connect(address)?;

	let mut pos = 0;
	while pos < data.len() {
        let bytes_written = stream.write(&(data.as_bytes())[pos..])?;
        pos += bytes_written;
    }

	println!("Bytes written: {}", pos);

	Ok(())
}
