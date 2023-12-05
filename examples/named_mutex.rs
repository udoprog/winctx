use anyhow::Result;
use tokio::signal::ctrl_c;
use winctx::NamedMutex;

const NAME: &str = "se.tedro.Example";

#[tokio::main]
async fn main() -> Result<()> {
    let Some(_m) = NamedMutex::create_acquired(NAME)? else {
        println!("Mutex '{NAME}' already acquired");
        return Ok(());
    };

    ctrl_c().await?;
    Ok(())
}
