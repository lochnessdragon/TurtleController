use tokio::net::{TcpListener, TcpStream};
use futures_util::{future, StreamExt, TryStreamExt};
use tokio::io::AsyncReadExt;
use std::net::SocketAddr;

enum ConnState {
	Login,
	Proxy,
	Shutdown
}

enum ConnType {
	Client,
	Turtle
}

fn usage() {
	println!("Usage: ws-proxy [-h] [port number]");
	println!("Arguments:");
	println!("\tport number = the port to bind to. default = 1234");
	println!("OPTIONS:");
	println!("\t-h or --help = show help");
}

#[tokio::main]
async fn main() {
	// check args
	let mut port : String = "1234".to_string();
	if std::env::args().len() > 1 {
		for (i, argument) in std::env::args().enumerate() {
			if argument == "--help" || argument == "-h" {
				usage();
				std::process::exit(0);
			} else {
				port = std::env::args().nth(i).expect("crash while reading args");
			}
		}
	}
	println!("Opening ws proxy server on port: {}", port);
	// bind a tcp listener to this address
	let listener = TcpListener::bind("localhost:".to_owned() + &port[..]).await.unwrap();

	loop {
		// second item has port and ip, so we ignore it
		let (socket, addr) = listener.accept().await.unwrap();
		tokio::spawn(async move {
			spawn_ws(socket, addr).await;
		});
	}
}

async fn spawn_ws(socket: TcpStream, addr: SocketAddr) {
	// print bytes from the byte stream and close the connection
	println!("Recieved connection from: {}", addr);

	// initialize websocket connection
	let ws_stream = tokio_tungstenite::accept_async(socket)
		.await
		.expect("Error occured during websocket handshake!");
	
	println!("New websocket connection from: {}", addr);

	let (write, read) = ws_stream.split();
	
	read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
		.forward(write)
		.await
		.expect("Failed to forward messages")
	// the connection closes when it goes out of scope.
}