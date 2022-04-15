use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
	let mut address = "localhost:1234";
	let mut data = "Hello sock";
	
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
