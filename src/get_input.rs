use tokio::io::{self, AsyncBufReadExt};

pub async fn get_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("{}", prompt);
    let mut input = String::new();
    let mut reader = io::BufReader::new(io::stdin());
    reader.read_line(&mut input).await?;
    Ok(input.trim().to_string())
}
