use std::fs::File;
use std::path::Path;

pub fn load_obj<T: AsRef<Path>>(path: T) -> anyhow::Result<()> {
    File::open(path)?;

    Ok(())
}