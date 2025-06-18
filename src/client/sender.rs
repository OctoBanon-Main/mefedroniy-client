use anyhow::{Context, Result};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedReceiver;

pub async fn send_messages(addr: String, mut send_rx: UnboundedReceiver<(String, String)>) -> Result<()> {
    while let Some((nick, text)) = send_rx.recv().await {
        let stream = match TcpStream::connect(&addr).await {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Connection error: {}", e);
                continue;
            }
        };

        let mut stream = stream;
        let formatted_msg = format!("\u{00B0}\u{0298}<{}> {}", nick, text);
        let padded_message = format!("{: <39}", formatted_msg);
        let mut buf = vec![0x01];
        buf.extend_from_slice(format!("\r\x03{}", padded_message).as_bytes());

        stream
            .write_all(&buf)
            .await
            .context("Failed to write message to stream")?;

        stream.shutdown().await.ok();
    }

    Ok(())
}