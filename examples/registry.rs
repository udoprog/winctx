use anyhow::Result;

fn main() -> Result<()> {
    let key = winctx::OpenRegistryKey::local_machine().open("SOFTWARE\\Tesseract-OCR")?;
    let value = key.get_string("Path")?;
    dbg!(value);
    Ok(())
}
