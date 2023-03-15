use std::env;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let url = "https://api.openai.com/v1/chat/completions";
    let bearer_auth = env::var("OPENAI_API_KEY");

    let body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{
            "role": "user",
            "content": "What is the OpenAI mission?",
        }],
    });

    match bearer_auth {
        Ok(bearer_auth) => {
            let res = client.post(url)
                .header("Content-Type", "application/json")
                .bearer_auth(bearer_auth)
                .body(body.to_string())
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
