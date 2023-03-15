use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.openai.com/v1/chat/completions";
    let client = reqwest::Client::new();

    let bearer_auth = env::var("OPENAI_API_KEY");

    match bearer_auth {
        Ok(bearer_auth) => {
            let res = client.get(url)
                .header("Content-Type", "application/json")
                .bearer_auth(bearer_auth)
                .send()
                .await?
                .text()
                .await?;

            println!("{}", res);
        }
        Err(e) => println!("OPENAI_API_KEYが設定されていません: {:?}", e),
    }

    Ok(())
}
