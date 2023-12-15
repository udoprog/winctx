use std::path::PathBuf;

use anyhow::Result;

fn main() -> Result<()> {
    let key = winctx::OpenRegistryKey::local_machine().open("SOFTWARE\\Tesseract-OCR")?;
    let value = key.get_string("Path")?;
    let path = PathBuf::from(value);
    let dll = path.join("libtesseract-5.dll");
    dbg!(&dll);
    Ok(())
}
