use std::io::{self, Write};

pub fn read_input(prompt: &str) -> io::Result<String> {
    print!("\x1B[32m{}\x1B[0m", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}