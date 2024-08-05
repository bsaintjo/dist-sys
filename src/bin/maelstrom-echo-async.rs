use std::{io::{self, BufRead, StdinLock}, ops::DerefMut, sync::Arc};

use dist_sys::Message;
use smol::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter}, lock::Mutex, Unblock
};

// fn run(stdin: &mut StdinLock) -> io::Result<()> {
//     log::debug!("NEW MSG");
//     let mut buffer = String::new();
//     stdin.read_line(&mut buffer)?;
//     log::debug!("BUFFER {buffer}");
//     let msg: Message = serde_json::from_str(&buffer)?;
//     log::debug!("MSG RECV {msg:?}");
//     let echo = msg.echo_ok();
//     log::debug!("MSG ECHO_OK: {echo:?}");
//     let reply = serde_json::to_string(&echo)?;
//     log::debug!("MSG ECHO_OK REPLY: {reply}");
//     println!("{reply}");
//     log::debug!("END MSG");
//     Ok(())
// }

async fn run() -> smol::io::Result<()> {
    let stdin = Arc::new(Mutex::new(BufReader::new(Unblock::new(io::stdin()))));
    let mut stdout = BufWriter::new(Unblock::new(io::stdout()));
    let x = smol::spawn(async move {
        init(stdin.clone()).await
    });
    let y = smol::spawn(async move {
        stdout.write_all(b"test\n").await?;
        stdout.flush().await
    });
    smol::future::try_zip(x, y).await?;
    Ok(())
}

async fn init<R: AsyncBufReadExt + Unpin>(stdin: Arc<Mutex<R>>) -> smol::io::Result<()> {
    let mut stdin = stdin.lock_arc().await;
    let mut buf = String::new();
    stdin.read_line(&mut buf).await?;
    Ok(())
}

fn main() -> io::Result<()> {
    smol::block_on(run())
}
