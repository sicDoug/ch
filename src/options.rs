use crate::disc::Config;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub role:    Role,
    pub content: String,
}

pub trait Push {
    fn add_new(&mut self, role: Role, content: &str);
}

impl Push for Vec<Message> {
    fn add_new(&mut self, role: Role, content: &str) {
        self.push(Message {
            role:       role,
            content:    content.to_owned(),
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub model:       String,
    pub stream:      bool,
    pub temperature: f32,
    pub messages:    Vec<Message>,
}

impl Payload {
    pub fn construct(
        config: &Config,
        messages: &[Message],
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(&Self {
            model:       config.model.to_owned(),
            temperature: config.temperature,
            messages:    messages.to_vec(),
            stream:      true,
        })
    }
}
