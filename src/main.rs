use std::env;
use std::io::{stdin, stdout, Write};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    id: String,
    object: String,
    created: u32,
    model: String,
    usage: Usage,
    choices: Vec<Choices>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choices {
    message: Message,
    finish_reason: String,
    index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let url = "https://api.openai.com/v1/chat/completions";
    let bearer_auth = env::var("OPENAI_API_KEY");

    match bearer_auth {
        Ok(bearer_auth) => {
            print!("> ");
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let body = json!({
                "model": "gpt-3.5-turbo",
                "messages": [{
                    "role": "user",
                    "content": input,
                }],
            });

            let res = client
                .post(url)
                .header("Content-Type", "application/json")
                .bearer_auth(bearer_auth)
                .body(body.to_string())
                .send()
                .await?
                .json::<Response>()
                .await?;

            for str in serde_json::to_string(&res.choices[0].message.content)
                .unwrap()
                .trim_matches('"')
                .split("\\n")
                .collect::<Vec<&str>>()
                .iter()
            {
                println!("{}", str)
            }
        }
        Err(e) => println!("OPENAI_API_KEYが設定されていません: {:?}", e),
    }

    Ok(())
}
