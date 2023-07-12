use std::net::{SocketAddr, UdpSocket};
use std::collections::HashMap;
const PROJECT_NAME: &str = "BLANE";
const AUTHOR_NAME: &str = "PURE BLACK";
pub struct Server {
    socket: UdpSocket,
    clients: HashMap<SocketAddr, String>,
    pub fn handle_new_connection(&mut self, client_addr: SocketAddr, client_name: String) {
        self.clients.insert(client_addr, client_name);
        println!("New client connected: {}", client_name);
    }

    pub fn handle_client_disconnection(&mut self, client_addr: SocketAddr) {
        if let Some(client_name) = self.clients.remove(&client_addr) {
            println!("Client disconnected: {}", client_name);
        }
    }

    pub fn get_online_clients(&self) -> Vec<String> {
        self.clients.values().cloned().collect()
    }
}

impl Server {
    pub fn new(server_address: &str) -> Result<Self, String> {
        let socket = UdpSocket::bind(server_address)
            .map_err(|e| format!("Failed to bind socket: {}", e))?;
        
        Ok(Self {
            socket,
            clients: HashMap::new(),
        })
    }

    pub fn start(&self) -> Result<(), String> {
        let mut buffer = [0u8; 1024];

        loop {
            match self.socket.recv_from(&mut buffer) {
                Ok((bytes_read, src_addr)) => {
                    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("Received message from {}: {}", src_addr, message);

                    // Broadcast the message to all clients
                    self.broadcast_message(&message)?;
                }
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::WouldBlock {
                        return Err(format!("Error receiving message: {}", e));
                    }
                }
            }

            // Other server logic (e.g., handle new connections, manage clients)
        }
    }

    pub fn broadcast_message(&self, message: &str) -> Result<(), String> {
        for client_addr in self.clients.keys() {
            self.socket
                .send_to(message.as_bytes(), client_addr)
                .map_err(|e| format!("Failed to send message to {}: {}", client_addr, e))?;
        }

        Ok(())
    }

    // Other methods
}
