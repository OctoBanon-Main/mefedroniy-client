use std::error::Error;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedSender;

async fn try_receive_messages(
    addr: &str,
    last_position: &mut usize,
    chat_tx: &UnboundedSender<String>,
) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(addr).await?;
    stream.write_all(&[0x00]).await?;

    let mut size_buf = vec![0u8; 1024];
    let n = stream.read(&mut size_buf).await?;
    let file_size: usize = String::from_utf8_lossy(&size_buf[..n])
        .trim_matches('\0')
        .trim()
        .parse()?;

    match file_size.cmp(last_position) {
        std::cmp::Ordering::Less => *last_position = 0,
        std::cmp::Ordering::Equal => return Ok(()),
        std::cmp::Ordering::Greater => {
            stream.write_all(format!("\x02{}", last_position).as_bytes()).await?;
            let mut messages_buf = vec![0u8; file_size - *last_position];
            stream.read_exact(&mut messages_buf).await?;
            
            let messages = String::from_utf8_lossy(&messages_buf).into_owned();
            chat_tx.send(messages)?;
            *last_position = file_size;
        }
    }

    Ok(())
}

pub async fn receive_messages(addr: String, update_interval: u64, chat_tx: UnboundedSender<String>) {
    let mut last_position = 0;
    loop {
        match try_receive_messages(&addr, &mut last_position, &chat_tx).await {
            Ok(()) => (),
            Err(e) => eprintln!("Ошибка получения сообщений: {}", e),
        }
        tokio::time::sleep(Duration::from_secs(update_interval)).await;
    }
}