use std::{
    io::{self},
    sync::Arc,
};

use dist_sys::Message;
use smol::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    lock::Mutex,
    Unblock,
};
use tracing::instrument;
use tracing_subscriber::FmtSubscriber;

async fn run() -> smol::io::Result<()> {
    let stdin = Arc::new(Mutex::new(BufReader::new(Unblock::new(io::stdin()))));
    let stdout = Arc::new(Mutex::new(BufWriter::new(Unblock::new(io::stdout()))));
    let x = smol::spawn({
        let stdout = stdout.clone();
        async move { init(stdin.clone(), stdout).await }
    });
    let y = smol::spawn({
        let stdout = stdout.clone();
        async move {
            let mut stdout = stdout.lock_arc().await;
            stdout.write_all(b"test\n").await?;
            stdout.flush().await
        }
    });
    smol::future::try_zip(x, y).await?;
    Ok(())
}

#[instrument(skip_all)]
async fn init<R: AsyncBufReadExt + Unpin, W: AsyncWriteExt + Unpin>(
    stdin: Arc<Mutex<R>>,
    stdout: Arc<Mutex<W>>,
) -> smol::io::Result<()> {
    let mut buf = String::new();
    {
        let mut stdin = stdin.lock_arc().await;
        stdin.read_line(&mut buf).await?;
    }
    tracing::info!(buffer = buf);

    let msg: Message = serde_json::from_str(&buf)?;
    tracing::info!(message = ?msg);

    let echo = msg.echo_ok();
    tracing::info!(echo = ?echo);

    let reply = serde_json::to_string(&echo)?;
    tracing::info!(reply = reply);
    {
        let mut stdout = stdout.lock_arc().await;
        stdout.write_all(reply.as_bytes()).await?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    smol::block_on(run())
}
