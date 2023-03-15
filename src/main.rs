#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.openai.com/v1/chat/completions";
    let client = reqwest::Client::new();

    let res = client.get(url)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", res);
    Ok(())
}
