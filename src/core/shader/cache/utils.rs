use std::{fs::{File, FileTimes}, path::Path, time::SystemTime};

use anyhow::Result;

pub fn reset_atime(path: &Path) -> Result<()> {
    let now = SystemTime::now();
    let times = FileTimes::new().set_accessed(now);
    File::open(path)?.set_times(times)?;
    Ok(())
}

pub fn probe_atime(sentinel: &Path) -> Result<()> {
    std::fs::write(sentinel, b"")?;
    let times = FileTimes::new().set_accessed(SystemTime::now());
    File ::open(sentinel)?.set_times(times)?;
    std::fs::metadata(sentinel)?.accessed()?;
    Ok(())
}

