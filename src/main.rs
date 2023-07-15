use std::io::Write;
mod chat {
    pub mod client;
    pub mod server;
}

use chat::client::Client;
use chat::server::Server;

fn main() {
    let server_address = "127.0.0.1:14514";
    let _client_name = "Black";

    let client = Client::new(server_address).expect("Failed to create client");
    client.connect().expect("Failed to connect to server");

    let server = Server::new(server_address).expect("Failed to create server");
    let _server_handle = std::thread::spawn(move || {
        server.start().expect("Failed to start server");
    });

    println!("Connected to server. You can start sending messages.");
    println!("您以上线，请畅所欲言！");

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let message = input.trim();

        if message.is_empty() {
            continue;
        }

        client.send_message(message).expect("Failed to send message");

        let received_message = client.receive_message().expect("Failed to receive message");
        println!("Received message: {}", received_message);
    }
}
