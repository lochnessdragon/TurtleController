use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
	// bind a tcp listener to this address
	let listener = TcpListener::bind("localhost:1234").await.unwrap();

	loop {
		// second item has port and ip, so we ignore it
		let (socket, _) = listener.accept().await.unwrap();
		tokio::spawn(async move {
			spawn_ws(socket).await;
		});
	}
}

async fn spawn_ws(mut socket: TcpStream) {
	// print bytes from the byte stream and close the connection
	let mut buffer = Vec::<u8>::new();

	socket.read_to_end(&mut buffer).await;

	println!("Recieved request: {}", std::str::from_utf8(&buffer).unwrap());
	// the connection closes when it goes out of scope.
}