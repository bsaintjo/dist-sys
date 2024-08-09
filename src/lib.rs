use serde::{Deserialize, Serialize};

mod init;

pub use init::init;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: usize,
    pub src: String,
    pub dest: String,
    pub body: Body,
}

impl Message {
    pub fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }

    pub(crate) fn body(&self) -> &Body {
        &self.body
    }

    pub fn echo_ok(self) -> Self {
        Message {
            id: self.id,
            src: self.dest,
            dest: self.src,
            body: Body::EchoOk {
                type_: "echo_ok".to_string(),
                msg_id: self.body.msg_id(),
                in_reply_to: self.body.msg_id(),
                echo: self.body.echo().to_string(),
            },
        }
    }

    pub fn generate_ok(mut self) -> Self {
        self.body.generate_ok();
        Message {
            id: self.id,
            src: self.dest,
            dest: self.src,
            body: self.body
        }
    }

    pub(crate) fn init_ok(node: &str, dest: &str, msg_id: usize) -> Self {
        Message {
            id: 1,
            src: node.to_string(),
            dest: dest.to_string(),
            body: Body::InitReply {
                type_: "init_ok".to_string(),
                in_reply_to: msg_id,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Body {
    Echo {
        #[serde(rename = "type")]
        type_: String,
        msg_id: usize,
        echo: String,
    },
    EchoOk {
        #[serde(rename = "type")]
        type_: String,
        msg_id: usize,
        in_reply_to: usize,
        echo: String,
    },
    Init {
        #[serde(rename = "type")]
        type_: String,
        node_id: String,
        node_ids: Vec<String>,
        msg_id: usize,
    },
    InitReply {
        #[serde(rename = "type")]
        type_: String,
        in_reply_to: usize,
    },
    Generate {
        #[serde(rename = "type")]
        type_: String,
        msg_id: usize,
    },
    GenerateOk {
        #[serde(rename = "type")]
        type_: String,
        in_reply_to: usize,
        msg_id: usize,
        id: uuid::Uuid,
    },
}

impl Body {
    fn type_(&self) -> &str {
        match self {
            Body::Echo { type_, .. } => type_,
            Body::EchoOk { type_, .. } => type_,
            Body::Init { type_, .. } => type_,
            Body::InitReply { type_, in_reply_to } => todo!(),
            Body::Generate { type_, .. } => type_,
            Body::GenerateOk { type_, .. } => type_,
        }
    }
    pub(crate) fn msg_id(&self) -> usize {
        match self {
            Body::Echo { msg_id, .. } => *msg_id,
            Body::EchoOk { msg_id, .. } => *msg_id,
            Body::Init { msg_id, .. } => *msg_id,
            Body::InitReply { type_, in_reply_to } => todo!(),
            Body::Generate { msg_id, .. } => *msg_id,
            Body::GenerateOk { msg_id, .. } => *msg_id,
        }
    }

    fn echo(&self) -> &str {
        match self {
            Body::Echo { echo, .. } => echo,
            Body::EchoOk { echo, .. } => echo,
            Body::Init { .. } => todo!(),
            Body::InitReply { .. } => todo!(),
            Body::Generate { .. } => todo!(),
            Body::GenerateOk { .. } => todo!(),
        }
    }

    pub fn echo_ok(&mut self) {
        if let Body::Echo { .. } = self {
            *self = Body::EchoOk {
                type_: "echo_ok".to_string(),
                msg_id: self.msg_id(),
                in_reply_to: self.msg_id(),
                echo: self.echo().to_string(),
            }
        }
    }

    pub fn generate_ok(&mut self) {
        if let Body::Generate { .. } = self {
            let id = Uuid::new_v4();
            *self = Body::GenerateOk {
                type_: "generate_ok".to_string(),
                in_reply_to: self.msg_id(),
                msg_id: self.msg_id(),
                id,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    // #[test]
    // fn test_ser() {
    //     let msg = Body::Echo {
    //         type_: "test".to_string(),
    //         msg_id: 1,
    //         echo: "echoecho".to_string(),
    //     };
    //     let msg = Message {
    //         src: "src".to_string(),
    //         dest: "dest".to_string(),
    //         body: msg,
    //     };
    //     println!("{}", serde_json::to_string_pretty(&msg).unwrap());
    // }

    #[test]
    fn test_file() {
        let mut buf = String::new();
        let mut echo = File::open("extra/data/echo.json").unwrap();
        echo.read_to_string(&mut buf).unwrap();
        let parsed: Message = serde_json::from_str(&buf).unwrap();
        println!("{parsed:?}");
    }

    #[test]
    fn it_works() {
        let msg = r#"{ "id": 0, "src": "c1", "dest": "n1", "body": { "type": "echo", "msg_id": 1, "echo": "Please echo 35" } }"#;
        let parsed: Message = serde_json::from_str(msg).unwrap();
        println!("{parsed:?}");

        let msg = r#"{"id":0,"src":"c0","dest":"n0","body":{"type":"init","node_id":"n0","node_ids":["n0"],"msg_id":1}}"#;
        let parsed: Message = serde_json::from_str(msg).unwrap();
        println!("{parsed:?}");
    }
}
