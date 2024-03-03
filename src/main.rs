mod application;
mod events;
use std::io::stdout;

use application::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut application = Application::new(stdout(), Mode::Normal);

    application.run().await
}
