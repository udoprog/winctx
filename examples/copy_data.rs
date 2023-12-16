use anyhow::Result;
use winctx::window::FindWindow;

pub fn main() -> Result<()> {
    let Some(window) = FindWindow::new().class("se.tedro.Example").find()? else {
        println!("Could not find window");
        return Ok(());
    };

    window.copy_data(42, b"foobar")?;
    Ok(())
}
