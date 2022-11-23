//!# Pushdeer SDK for Rust
//!
//!## install
//!
//!```toml
//![dependencies]
//!rupushdeer = "0.1.0"
//!```
//!
//!## Usage:
//!
//!### 1. Use pushdeer default server
//!
//!```rust
//!use rupushdeer::PushDeer;
//!
//!let deer = PushDeer::new("PDU5315TCkMN0KiBRqbceaXxpYx3DvdbiZ3JpAIE");
//!deer.send_text("title").expect("");
//!deer.send_text_with_desp("Hello", "send_text_with_desp").expect("");
//!deer.send_markdown("# markdown\npushdeer").expect("");
//!deer.send_markdown_with_desp("# markdown\npushdeer", "send_markdown_with_desp").expect("");
//!deer.send_image("https://gitee.com/easychen/pushdeer/raw/main/doc/image/clipcode.png").expect("");
//! ```
//!
//!### 2. Use self-hosted server
//!
//!```rust
//!use rupushdeer::PushDeer;
//!
//!let deer = PushDeer::new("PDU5315TCkMN0KiBRqbceaXxpYx3DvdbiZ3JpAIE").set_server("http://127.0.0.1:12345");
//!deer.send_text("title").expect("");
//!```

use log::debug;
use serde_json::Value;
use std::ops::Add;

pub struct PushDeer {
    server: String,
    endpoint: String,
    push_key: String,
}

impl PushDeer {
    pub fn new(push_key: impl Into<String>) -> Self {
        let push_key = push_key.into();
        PushDeer {
            server: String::from("https://api2.pushdeer.com"),
            endpoint: String::from("/message/push"),
            push_key,
        }
    }

    pub fn set_server(mut self, server: impl Into<String>) -> Self {
        let server = server.into();
        self.server = server;
        self
    }

    fn push(
        &self,
        text: impl Into<String>,
        desp: Option<impl Into<String>>,
        text_type: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let text = text.into();
        let desp = if desp.is_some() {
            desp.unwrap().into()
        } else {
            String::from("")
        };

        let client = reqwest::blocking::Client::new();
        let mut params = vec![("pushkey", &self.push_key), ("text", &text)];
        if desp.ne("") {
            params.push(("desp", &desp));
        }
        if !text_type.eq("text") {
            params.push(("type", &text_type));
        }

        let res = client
            .get(String::from(&self.server).add(&self.endpoint))
            .query(&params)
            .send()?;
        debug!("pushing url: {}", res.url().as_str());

        let json_serde: Value = res.json()?;
        debug!("pushing result: {}", &json_serde);
        let json_serde = json_serde["content"]["result"][0].as_str().unwrap();
        let json_res: Value = serde_json::from_str(json_serde)?;

        if json_res["success"] == "ok" {
            Ok(())
        } else {
            Err("push error".into())
        }
    }

    /// Any text are accepted when type is text.
    pub fn send_text(&self, body: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.push(body, None::<String>, String::from("text"))
    }

    /// Any text with the second part are accepted when type is text.
    pub fn send_text_with_desp(
        &self,
        body: impl Into<String>,
        desp: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.push(body, Some(desp), String::from("text"))
    }

    /// Text in Markdown format are accepted when type is markdown.
    pub fn send_markdown(&self, body: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.push(body, None::<String>, String::from("markdown"))
    }

    /// Text in Markdown format with the second part are accepted when type is markdown.
    pub fn send_markdown_with_desp(
        &self,
        body: impl Into<String>,
        desp: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.push(body, Some(desp), String::from("markdown"))
    }

    /// Only image src are accepted by API now, when type is image.
    pub fn send_image(&self, body: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.push(body, None::<String>, String::from("image"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_server() -> Result<(), Box<dyn std::error::Error>> {
        PushDeer::new("PDU5315TCkMN0KiBRqbceaXxpYx3DvdbiZ3JpAIE")
            .set_server("http://127.0.0.1:12345");
        Ok(())
    }
}
