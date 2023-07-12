// Project: BLANE
// Author: PURE BLACK
const PROJECT_NAME: &str = "BLANE";
const AUTHOR_NAME: &str = "PURE BLACK";


mod chat;
mod encryption;

use std::io::{self, Write};
use std::thread;

use chat::{Client, Server};

fn main() {
    // 获取服务器地址和客户端名称
    let server_address = "127.0.0.1:14514";
    let client_name = "Alice";

    // 创建客户端实例并连接到服务器
    let client = Client::new(server_address).expect("Failed to create client");
    client.connect().expect("Failed to connect to server");

    // 创建服务器实例并启动服务器
    let server = Server::new(server_address).expect("Failed to create server");
    let server_handle = thread::spawn(move || {
        server.start().expect("Failed to start server");
    });

    // 在主线程中处理客户端输入和接收消息
    println!("Connected to server. You can start sending messages.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let message = input.trim();

        if message.is_empty() {
            continue;
        }

        // 发送消息给服务器
        client.send_message(message).expect("Failed to send message");

        // 接收服务器广播的消息
        let received_message = client.receive_message().expect("Failed to receive message");
        println!("Received message: {}", received_message);
    }

    // 等待服务器线程结束
    server_handle.join().expect("Server thread panicked");
}
