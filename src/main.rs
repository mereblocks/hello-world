use anyhow::Result;
use log::info;

fn run_socket() -> Result<String> {
    let ctx = zmq::Context::new();
    let sock = ctx.socket(zmq::REP)?;
    sock.bind("tcp://*:8989")?;
    info!("Send a message to port 8989 to get a response (use a REQ socket)");
    let s = sock
        .recv_string(0)?
        .unwrap_or_else(|_| "not a utf-8 string".into());
    sock.send(&format!("Hello, {s}"), 0)?;
    info!("Check your socket for the response");
    Ok(s)
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Starting hello-world");
    loop {
        run_socket()?;
    }
}
