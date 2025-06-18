use tokio::io::{self, AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio::io::stdin;
use tokio::io::stdout;

pub async fn read_input(prompt: &str) -> io::Result<String> {
    let mut stdout = stdout();
    stdout.write_all(format!("\x1B[32m{}\x1B[0m", prompt).as_bytes()).await?;
    stdout.flush().await?;

    let mut input = String::new();
    let mut stdin = BufReader::new(stdin());
    stdin.read_line(&mut input).await?;
    Ok(input.trim().to_string())
}