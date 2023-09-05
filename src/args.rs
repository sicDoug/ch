use crate::color::Color;

use std::path::PathBuf;
use clap::Parser;

/// ChatGPT CLI
#[derive(Parser)]
#[command(author, about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    /// Text prompt to ChatGPT
    #[arg(value_name = "TEXT")]
    pub prompt: Vec<String>,
    /// Input file to append to message
    #[arg(short, long, value_name = "FILE")]
    pub input: Option<Vec<PathBuf>>,
    /// Delete chat history
    #[arg(short, long)]
    pub clear: bool,
    /// Add System message
    #[arg(short, long, requires = "prompt")]
    pub system: bool,
    /// Bypass chat history
    #[arg(short, long, requires = "prompt", conflicts_with = "system")]
    pub bypass: bool,
    /// List (print) chat history
    #[arg(short, long, conflicts_with_all = &["clear", "bypass"])]
    pub list: bool,
    /// Dump last message to file
    #[arg(short, long,
          value_name = "FILE",
          conflicts_with_all = &["prompt", "input", "system", "bypass", "list"]
    )]
    pub dump: Option<PathBuf>,
    /// Print current config
    #[arg(short = 'P', long)]
    pub print_config: bool,
    /// Set API key
    #[arg(long, value_name = "API_KEY")]
    pub set_api_key: Option<String>,
    /// Set chat model
    #[arg(long, value_name = "CHAT_MODEL")]
    pub set_model: Option<String>,
    /// Set temperature (between 0 and 2)
    #[arg(long, value_name = "FLOAT")]
    pub set_temp: Option<f32>,
    /// Set output color
    #[arg(value_enum, long, value_name = "COLOR", hide_possible_values = true)]
    pub set_color: Option<Color>,
}
