use laminar::{SocketEvent, Socket, ErrorKind};
use std::thread;

pub struct Server {
    
}

impl Server {
    pub fn new(something: String) -> () {
        
    }

    pub fn listen(self, port: u16) -> Result<(), ErrorKind> {
        let server_thread = thread::spawn(move || -> Result<(), ErrorKind> {
            // Create the socket
            let mut socket = Socket::bind(format!("127.0.0.1:{}", port))?;
            let event_reciever = socket.get_event_receiver();

            // Start polling the socket
            let _polling_thread = thread::spawn(move || socket.start_polling());

            loop {
                // An event has occured on the socket
                let result = event_reciever.recv();

                match result {
                    Ok(socket_event) => {
                        match socket_event {
                            SocketEvent::Packet(packet) => { // We recieved a packet
                                let received_buffer: &[u8] = packet.payload();
                                
                            }
                            SocketEvent::Timeout(timeout_event) => { // A client timed out

                            }
                            _ => {} // We don't care about Connect or Disconnect, we implement ourselves.
                        }
                    }
                    Err(_) => {} // Something went wrong recieving an event, but we don't care.
                }
            }
        });
        
        match server_thread.join().unwrap() {
            Ok(()) => return Ok(()), // Server stopped successfully
            Err(v) => return Err(v), // Server crashed!
        };
    }
}