mod application;
mod buffer;
mod events;
mod viewport;
use std::io::stdout;

use application::*;
use buffer::Buffer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1);
    let mut buffer = Buffer::new(file);

    let mut application = Application::new(stdout(), Mode::Normal, vec![buffer]);

    application.run().await
}
