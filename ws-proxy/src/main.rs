use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;

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
	println!("Opening server of port: {}", port);
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

async fn spawn_ws(mut socket: TcpStream, addr: SocketAddr) {
	// print bytes from the byte stream and close the connection
	println("Recieved connection from: {}", addr);
	// the connection closes when it goes out of scope.
}