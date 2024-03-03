mod application;
mod buffer;
mod events;
use std::io::stdout;

use application::*;
use buffer::Buffer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1);
    let buffer = Buffer::new(file);

    let mut application = Application::new(stdout(), Mode::Normal, vec![buffer]);

    application.run().await
}
