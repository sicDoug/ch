mod args;
mod stream;
mod options;
mod color;
mod disc;

use crate::args::Args;
use crate::stream::stream;
use crate::options::{Role, Push};
use crate::color::Color;
use crate::disc::{load_config, write_config, load_log, clear_log, write_log};

use std::fs;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.clear {
        match clear_log() {
            Ok(()) => println!("History file deleted"),
            Err(_) => println!("No file to delete"),
        }
    }

    // get empty vec instead of log if bypass arg specified
    let mut messages = if args.bypass { Vec::new() } else { load_log()? };

    let mut config = load_config()?;
    if args.set_api_key.is_some()
        || args.set_model.is_some()
        || args.set_temp.is_some()
        || args.set_color.is_some()
    {
        args.set_api_key.map(|api_key| config.api_key = Some(api_key));
        args.set_model.map(|model| config.model = model);
        args.set_temp.map(|temperature| {
            if (0.0..=2.0).contains(&temperature) {
                config.temperature = temperature;
            } else {
                println!("Temperature must be between 0 and 2");
            }
        });
        args.set_color.map(|color| config.color = color);
        write_config(&config)?; // save to disc
        println!("Config file updated");
    }

    if args.list {
        if messages.is_empty() {
            println!("No messages in chat log");
        } else {
            println!();
            for message in &messages {
                let color = match message.role {
                    Role::Assistant => &config.color,
                    _               => &Color::NoColor,
                };
                print!("{}", color.ansi());
                println!("{}\n", message.content);
            }
        }
    }

    if let Some(path) = args.dump {
        if messages.is_empty() {
            println!("No message to write")
        } else {
            fs::write(&path, &messages.last().unwrap().content)?;
        }
    };

    if args.print_config { config.print(); }

    // collect vec of non-flags args to string
    let mut prompt = args.prompt.join(" ");

    if let Some(path) = args.input {
        let contents = fs::read_to_string(&path)?;
        let file_name = path.to_str().unwrap();
        prompt += &format!("\n\n{}\n```\n{}```", file_name, contents);
    }

    if prompt.is_empty() { return Ok(()); }

    if args.system {
        messages.add_new(Role::System, &prompt);
        write_log(&messages)?;
        println!("System message added to log file");
        return Ok(())
    }

    messages.add_new(Role::User, &prompt);
    let response = stream(&messages, &config).await?;
    if !args.bypass {
        messages.add_new(Role::Assistant, &response);
        write_log(&messages)?;
    }

    Ok(())
}
