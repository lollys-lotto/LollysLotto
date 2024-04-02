use anyhow::anyhow;
use csv::Writer;
use std::{fs::File, path::Path};

pub fn checked_open(p: impl AsRef<Path>, overwrite: bool) -> anyhow::Result<Writer<File>> {
    if !overwrite && p.as_ref().exists() {
        return Err(anyhow!(
            "Cannot open {} because file already exists",
            p.as_ref().display()
        ));
    }
    let f = File::create(p)?;
    Ok(Writer::from_writer(f))
}
