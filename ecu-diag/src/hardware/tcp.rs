use std::sync::Arc;
use tokio::io::AsyncWriteExt;

use tokio::net::TcpStream;

use tokio::sync::Mutex;

pub struct TcpProtocol {
    pub reader: Arc<Mutex<tokio::io::ReadHalf<tokio::net::TcpStream>>>,
    pub writer: Arc<Mutex<tokio::io::WriteHalf<tokio::net::TcpStream>>>,
    pub connection_status: bool,
}

impl TcpProtocol {
    /// Creates a new Native ISOTP channel
    pub async fn new() -> Option<Self> {
        let tcp_client = TcpStream::connect("192.168.7.1:50130").await;

        match tcp_client {
            Ok(stream) => {
                let (reader, writer) = tokio::io::split(stream);

                let reader_arc = Arc::new(Mutex::new(reader));

                let tcp_struct = TcpProtocol {
                    reader: reader_arc,
                    writer: Arc::new(Mutex::new(writer)),
                    connection_status: true,
                };

                Some(tcp_struct)
            }
            Err(e) => {
                // Connection failed, `e` contains the error
                println!("Failed to connect tp tcp server: {}", e);
                log::debug!("Failed to connect tp tcp server: {}", e);
                None
            }
        }
    }

    pub async fn write(&mut self, msg: &str) {
        let _ = self.writer.lock().await.write_all(msg.as_bytes()).await;
    }
}
