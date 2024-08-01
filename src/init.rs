use std::io::{self, BufRead, StdinLock};

use serde::{Deserialize, Serialize};

use crate::Message;

pub fn init(stdin: &mut StdinLock) -> io::Result<()> {
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    eprintln!("BUFFER: {buffer}");
    let msg: Message = serde_json::from_str(&buffer)?;
    eprintln!("MESSAGE: {msg:?}");
    let reply = Message::init_ok(&msg.dest, &msg.src, msg.body().msg_id());
    let reply = serde_json::to_string(&reply)?;
    eprintln!("RESPONSE: {reply}");
    println!("{reply}");
    Ok(())
}

#[derive(Debug, Deserialize)]
struct InitMessage {
    #[serde(rename = "type")]
    type_: String,
    msg_id: usize,
    node_id: String,
    node_ids: Vec<String>,
}

impl InitMessage {
    fn node_id(&self) -> &str {
        &self.node_id
    }
}

#[derive(Debug, Serialize)]
struct InitReply {
    #[serde(rename = "type")]
    type_: String,
    in_reply_to: usize,
}

impl InitReply {
    fn init_ok(msg_id: usize) -> Self {
        Self {
            type_: "init_ok".to_string(),
            in_reply_to: msg_id,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {
        let init_msg = r#"{ "type": "init", "msg_id": 1, "node_id": "n3", "node_ids": ["n1", "n2", "n3"] }"#;
        let init_msg: InitMessage = serde_json::from_str(init_msg).unwrap();
        println!("{init_msg:?}");
        let reply = serde_json::to_string_pretty(&InitReply::init_ok(init_msg.msg_id)).unwrap();
        println!("{reply}");
    }
}
