use anyhow::{Context, Result};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedSender;

async fn try_receive_messages(
    addr: &str,
    last_position: &mut usize,
    chat_tx: &UnboundedSender<String>,
) -> Result<()> {
    let mut stream = TcpStream::connect(addr)
        .await
        .with_context(|| format!("Failed to connect to {}", addr))?;
    
    stream.write_all(&[0x00])
        .await
        .context("Failed to send initial byte")?;

    let mut size_buf = [0u8; 1024];
    let n = stream
        .read(&mut size_buf)
        .await
        .context("Failed to read file size")?;
    
    let real_length = size_buf[..n].iter().position(|&x| x == 0).unwrap_or(n);
    let file_size_str = std::str::from_utf8(&size_buf[..real_length])
        .context("Failed to convert file size to UTF-8")?
        .trim();

    let file_size: usize = file_size_str.parse()
        .with_context(|| format!("Failed to parse file size: {}", file_size_str))?;

    match file_size.cmp(last_position) {
        std::cmp::Ordering::Less => *last_position = 0,
        std::cmp::Ordering::Equal => return Ok(()),
        std::cmp::Ordering::Greater => {
            stream
                .write_all(format!("\x02{}", last_position).as_bytes())
                .await
                .context("Failed to send position for message retrieval")?;
            
            let mut messages_buf = vec![0u8; file_size - *last_position];
            stream
                .read_exact(&mut messages_buf)
                .await
                .context("Failed to read messages")?;
            
            let messages = String::from_utf8_lossy(&messages_buf).into_owned();
            chat_tx
                .send(messages)
                .context("Failed to send messages to channel")?;
            
            *last_position = file_size;
        }
    }

    Ok(())
}

pub async fn receive_messages(addr: String, update_interval: u64, chat_tx: UnboundedSender<String>) {
    let mut last_position = 0;
    loop {
        if let Err(e) = try_receive_messages(&addr, &mut last_position, &chat_tx).await {
            eprintln!("Error receiving messages: {:?}", e);
        }
        tokio::time::sleep(Duration::from_secs(update_interval)).await;
    }
}