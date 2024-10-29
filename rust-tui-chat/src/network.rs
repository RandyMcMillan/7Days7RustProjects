use tokio::{
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt, AsyncWriteExt},
};

pub struct Network {
    stream: Option<TcpStream>,
}

impl Network {
    pub async fn new(addr: &str) -> Result<Self, anyhow::Error> {
        let stream = if addr.contains(":") {
            TcpStream::connect(addr).await?
        } else {
            let listener = TcpListener::bind(addr).await?;
            let (stream, _) = listener.accept().await?;
            stream
        };
        Ok(Self { stream: Some(stream) })
    }

    pub async fn send_message(&mut self, msg: &str) -> Result<(), anyhow::Error> {
        if let Some(stream) = self.stream.as_mut() {
            stream.write_all(&[msg.len() as u8]).await?;
            stream.write_all(msg.as_bytes()).await?;
        }
        Ok(())
    }

    pub async fn receive_message(&mut self) -> Result<String, anyhow::Error> {
        if let Some(stream) = self.stream.as_mut() {
            let mut len = [0u8; 1];
            stream.read_exact(&mut len).await?;
            let mut buffer = vec![0; len[0] as usize];
            stream.read_exact(&mut buffer).await?;
            return String::from_utf8(buffer).map_err(|e| e.into());
        }
        Err(anyhow::anyhow!("No stream available"))
    }
}