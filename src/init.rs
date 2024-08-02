use std::io::{self, BufRead, StdinLock};

use crate::Message;

pub fn init(stdin: &mut StdinLock) -> io::Result<()> {
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    log::debug!("BUFFER: {buffer}");
    let msg: Message = serde_json::from_str(&buffer)?;
    log::debug!("MESSAGE: {msg:?}");
    let reply = Message::init_ok(&msg.dest, &msg.src, msg.body().msg_id());
    let reply = serde_json::to_string(&reply)?;
    log::debug!("RESPONSE: {reply}");
    println!("{reply}");
    Ok(())
}
