use std::io::{self, BufRead, StdinLock};

use dist_sys::Message;

fn run(stdin: &mut StdinLock) -> io::Result<()> {
    log::debug!("NEW MSG");
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    log::debug!("BUFFER {buffer}");
    let msg: Message = serde_json::from_str(&buffer)?;
    log::debug!("MSG RECV {msg:?}");
    let echo = msg.echo_ok();
    log::debug!("MSG ECHO_OK: {echo:?}");
    let reply = serde_json::to_string(&echo)?;
    log::debug!("MSG ECHO_OK REPLY: {reply}");
    println!("{reply}");
    log::debug!("END MSG");
    Ok(())
}

fn main() -> io::Result<()> {
    stderrlog::new()
        .verbosity(log::Level::Debug)
        .module(module_path!())
        .init()
        .unwrap();
    let mut stdin = io::stdin().lock();
    if let Err(e) = dist_sys::init(&mut stdin) {
        log::error!("Failed to initialize: {e}");
        Ok(())
    } else {
        log::info!("Successfully initialized.");
        loop {
            if let Err(e) = run(&mut stdin) {
                log::error!("{e}");
            }
        }
    }
}
