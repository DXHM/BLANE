use std::net::UdpSocket;

pub struct Client {
    socket: UdpSocket,
    server_address: String,
}

impl Client {
    pub fn new(server_address: &str) -> Result<Self, String> {
        let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| format!("Failed to bind socket: {}", e))?;
        socket.set_nonblocking(true).map_err(|e| format!("Failed to set non-blocking: {}", e))?;

        Ok(Self {
            socket,
            server_address: server_address.to_string(),
        })
    }

    pub fn connect(&self) -> Result<(), String> {
        // Connect to the server
        self.socket
            .connect(&self.server_address)
            .map_err(|e| format!("Failed to connect to server: {}", e))?;

        Ok(())
    }

    pub fn send_message(&self, message: &str) -> Result<(), String> {
        self.socket
            .send(message.as_bytes())
            .map_err(|e| format!("Failed to send message: {}", e))?;

        Ok(())
    }

    pub fn receive_message(&self) -> Result<String, String> {
        let mut buffer = [0u8; 1024];
        match self.socket.recv_from(&mut buffer) {
            Ok((bytes_read, _)) => {
                let message = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                Ok(message)
            }
            Err(e) => Err(format!("Failed to receive message: {}", e)),
        }
    }

    // Other methods
}
