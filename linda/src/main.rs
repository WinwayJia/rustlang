use linda::handle_connection;
use log::{error, info};
use std::net::TcpListener;

fn main() {
    simple_logger::init().unwrap();
    info!("Starting server...");

    let ip = "0.0.0.0:8594";

    let listener = TcpListener::bind(ip).expect("Unable to create listener.");
    info!("Server started on: {}{}", "http://", ip);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match handle_connection(stream) {
                Ok(_) => (),
                Err(e) => error!("Error handling connection: {}", e),
            },
            Err(e) => error!("Connection failed: {}", e),
        }
    }
}
