use std::env;
use std::io::{stdin, stdout, BufRead, BufReader, Write};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    id: Option<String>,
    object: Option<String>,
    created: Option<u32>,
    model: Option<String>,
    usage: Option<Usage>,
    choices: Option<Vec<Choices>>,
    error: Option<Error>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Error {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
    param: String,
    code: String,
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
    let model = env::var("OPENAI_API_MODEL").unwrap_or("gpt-3.5-turbo".to_string());
    let mut messages: Vec<Message> = vec![];

    match env::var("OPENAI_API_KEY") {
        Ok(bearer_auth) => loop {
            let mut input = String::new();
            let mut reader = BufReader::new(stdin().lock());

            loop {
                print!("> ");
                stdout().flush().unwrap();

                let mut line = String::new();
                reader.read_line(&mut line).expect("Failed to read line");

                if line == "\n" {
                    break;
                }

                input.push_str(&line);
            }

            println!();

            messages.push(Message {
                role: "user".to_string(),
                content: input,
            });

            let body = json!({
                "model": model,
                "messages": messages
            });

            let res = client
                .post(url)
                .header("Content-Type", "application/json")
                .bearer_auth(&bearer_auth)
                .body(body.to_string())
                .send()
                .await?
                .json::<Response>()
                .await?;

            match (res.error, res.choices) {
                (None, None) => {
                    println!("Failed request.");
                }
                (Some(error), _) => {
                    println!("{}", error.message);
                }
                (None, Some(choices)) => {
                    for str in choices[0]
                        .message
                        .content
                        .replace("\\\"", "\"")
                        .trim_matches('"')
                        .split("\\n")
                    {
                        println!("{}", str)
                    }

                    messages.push(Message {
                        role: choices[0].message.role.clone(),
                        content: choices[0].message.content.clone(),
                    });
                }
            }

            println!();
        },
        Err(e) => println!("OPENAI_API_KEYが設定されていません: {:?}", e),
    }

    Ok(())
}
