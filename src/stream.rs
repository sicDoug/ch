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

    let api_key: String = config.api_key.clone().unwrap_or_else(|| {
        env::var("OPENAI_API_KEY").expect("No OPENAI_API_KEY found")
    });
    let payload = Payload::construct(config, messages)?;
    // this var accumulates each delta of the response
    let mut full_response = String::new();

    // set output color by printing ansi
    print!("{}", config.color.ansi());
    println!(); // aesthetic af

    let mut stream = reqwest::Client::new()
        .post(OPENAI_API_URL)
        .bearer_auth(api_key)
        .header(header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send()
        .await?;

    while let Some(chunk) = stream.chunk().await? {
        let raw_string = String::from_utf8_lossy(&chunk);

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

        // handle each line since each chunk can contain multimple objects
        for line in raw_string.lines().filter(|line| line.starts_with(DATA_PREFIX)) {
            // chop off data prefix before json parsing
            let line = &line[DATA_PREFIX.len()..];

            if line.starts_with(DONE_MARKER) {
                print!("{}", NoColor.ansi());
                println!("\n"); // double aesthetic af

                return Ok(full_response);
            }

            let response = serde_json::from_str::<OpenAIResponse>(line)
                .map_err(|error| {
                    // TODO
                    // It seems the server very rarely responds either with
                    // an incomplete json object, or adds a non-escaped newline.
                    // Figure out if this can be mitigated in some way.
                    // Does the rest of the (possibly) incomplete chunk
                    // come in the next chunk? Test it.
                    print!("{}", NoColor.ansi());
                    eprintln!("Error is EOF?: {}", error.is_eof());
                    eprintln!("\nReceived an unparsable chunk: {}", line);
                    format!("{}", error)
                })?;

            if let Some(content) = &response.choices[0].delta.content {
                print!("{}", content);
                stdout().flush()?;

                full_response.push_str(&content);
            }
        }
    }

    Err(Box::new(Error::new(
            ErrorKind::Other, "No response from OpenAI"
    )))
}
