use crate::options::{Message, Payload};
use crate::color::Color::NoColor;
use crate::disc::Config;

use std::env;
use std::io::{Write, Error, ErrorKind, stdout};
use serde::Deserialize;
use reqwest::{header};

const OPENAI_API_URL:   &str = "https://api.openai.com/v1/chat/completions";
const DATA_PREFIX:      &str = "data: ";
const DONE_MARKER:      &str = "[DONE]";

#[derive(Deserialize)]
struct OpenAIDelta {
    content: Option<String>,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    delta: OpenAIDelta,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Deserialize)]
struct OpenAIErrorMessage {
    message: String,
}

#[derive(Deserialize)]
struct OpenAIError {
    error: OpenAIErrorMessage,
}

pub async fn stream(
    messages: &[Message],
    config:   &Config,
) -> Result<String, Box<dyn std::error::Error>> {

    let api_key = config.api_key.clone().unwrap_or_else(|| {
        env::var("OPENAI_API_KEY").expect("No OPENAI_API_KEY found")
    });

    let payload = Payload::construct(config, messages)?;

    print!("{}", config.color.ansi());
    println!();

    let mut stream = reqwest::Client::new()
        .post(OPENAI_API_URL)
        .bearer_auth(api_key)
        .header(header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send()
        .await?;

    let mut accumulated_response = String::new();

    let mut incomplete_part: Option<String> = None;

    while let Some(chunk) = stream.chunk().await? {
        let raw_string = String::from_utf8_lossy(&chunk).into_owned();

        if !stream.status().is_success() {
            match serde_json::from_str::<OpenAIError>(&raw_string) {
                Ok(json)    => {
                    print!("{}", NoColor.ansi());
                    println!("OpenAI responded with an error:\n");
                    println!("{}\n", json.error.message);
                    println!("{}", raw_string);

                    return Err(Box::new(Error::new(
                        ErrorKind::Other, "OpenAI responded with an error"
                    )))
                }
                Err(err)    => {
                    return Err(Box::new(Error::new(
                        ErrorKind::Other, format!("Unexpected error: {}", err)
                    )))
                }
            }
        }

        for line in raw_string.lines().filter(|line| !line.is_empty()) {
            let mut line = line.to_owned();

            // prepend the incomplete object to the line
            if let Some(part) = incomplete_part.take() {
                line = part + &line;
            }

            // chop off data prefix before json parsing
            if let Some(l) = line.strip_prefix(DATA_PREFIX) {
                line = l.to_string();
            }

            if line.starts_with(DONE_MARKER) {
                print!("{}", NoColor.ansi());
                println!("\n");

                return Ok(accumulated_response);
            }

            match serde_json::from_str::<OpenAIResponse>(&line) {
                Ok(res)     => {
                    if let Some(content) = &res.choices[0].delta.content {
                        print!("{}", content);
                        stdout().flush()?;

                        accumulated_response.push_str(&content);
                    }
                }
                Err(err)    => {
                    if err.is_eof() {
                        incomplete_part = Some(line);
                    } else {
                        return Err(Box::new(Error::new(
                            ErrorKind::Other, format!("Parsing failed: {}", line)
                        )))
                    }
                }
            }
        }
    }

    Err(Box::new(Error::new(
            ErrorKind::Other, "No response from OpenAI"
    )))
}
