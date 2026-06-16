use std::{num::NonZeroUsize, path::PathBuf, sync::LazyLock};

use anyhow::Result;
use lru::LruCache;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

use crate::core::shader::transpilation::TranspileContext;

pub mod keys;
pub mod utils;

const DEFAULT_CACHE_RETENTION_DAYS: u64 = 30;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProgramCacheEntry {
    pub format: u32,
    pub binary: Vec<u8>,
    pub transpile_context: TranspileContext,
}

pub struct Cache {
    pub enabled: bool,
    pub target_dir: PathBuf,
    pub base_dir: PathBuf,
    pub retention_days: u64,
    spv_index: Mutex<LruCache<u64, Vec<u8>>>,
    program_index: Mutex<LruCache<u64, ProgramCacheEntry>>,
}

impl Cache {
    fn from_env() -> Self {
        let base_dir = PathBuf::from(
            std::env::var("MESA_GLSL_CACHE_DIR").unwrap_or_default()
        );

        Self {
            enabled: std::env::var("FOGLE_ENABLE_SHADER_CACHE")
                .unwrap_or_else(|_| "1".into()) == "1",
            target_dir: base_dir.join("fogle_cache"),
            base_dir,
            retention_days: std::env::var("FOGLE_CACHE_RETENTION_DAYS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_CACHE_RETENTION_DAYS),
            spv_index: Mutex::new(LruCache::new(NonZeroUsize::new(50).unwrap())),
            program_index: Mutex::new(LruCache::new(NonZeroUsize::new(30).unwrap())),
        }
    }

    pub fn init(&mut self) {
        if self.base_dir.as_os_str().is_empty() {
            self.enabled = false;

            log::info!("Cache dir is empty, disabling cache");

            return;
        }

        if !self.enabled {
            log::info!("Shader cache disabled via env");
            return;
        }

        if let Err(e) = std::fs::create_dir_all(&self.target_dir) {
            log::warn!("Shader cache disabled: could not create dir: {e}");
            self.enabled = false;
            return;
        }

        let sentinel = self.target_dir.join(".atime_probe");
        if let Err(e) = utils::probe_atime(&sentinel) {
            log::warn!("Shader cache disabled: atime probe failed: {e}");
            self.enabled = false;
            return;
        }

        log::info!("Shader cache ready at {:?}", self.target_dir);
        self.evict_stale();
    }

    pub fn evict_stale(&self) {
        let cutoff = std::time::SystemTime::now()
            - std::time::Duration::from_secs(self.retention_days * 86400);
        let Ok(entries) = std::fs::read_dir(&self.target_dir) else { return };

        let mut spv_count = 0usize;
        let mut progbin_count = 0usize;
        let mut deleted = 0usize;

        for entry in entries.flatten() {
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str());
            match ext {
                Some("spv") => spv_count += 1,
                Some("progbin") => progbin_count += 1,
                _ => continue,
            }

            let Ok(meta) = std::fs::metadata(&path) else { continue };
            let Ok(atime) = meta.accessed() else { continue };

            if atime < cutoff {
                if std::fs::remove_file(&path).is_ok() {
                    deleted += 1;
                    match ext {
                        Some("spv") => spv_count -= 1,
                        Some("progbin") => progbin_count -= 1,
                        _ => {}
                    }
                }
                continue;
            }
        }

        let total = spv_count + progbin_count;
        log::info!("Deleted {deleted} old cache files.");
        log::info!("Cache index populated: {total} entries");
        if total > 0 {
            log::info!("  spv     : {spv_count} ({:.1}%)", spv_count as f64 / total as f64 * 100.0);
            log::info!("  progbin : {progbin_count} ({:.1}%)", progbin_count as f64 / total as f64 * 100.0);
        }
    }
}

static CACHE: LazyLock<Cache> = LazyLock::new(
    || {
        let mut c = Cache::from_env();
        c.init();

        c
    }
);

pub fn get_spv(key: u64) -> Result<Vec<u8>> {
    anyhow::ensure!(CACHE.enabled, "cache disabled");
    
    if let Some(spv) = CACHE.spv_index.lock().get(&key) {
        return Ok(spv.clone());
    }

    let path = CACHE.target_dir.join(format!("{key:016x}.spv"));
    let bytes = std::fs::read(&path)?;
    utils::reset_atime(&path).ok();
    CACHE.spv_index.lock().put(key, bytes.clone());
    Ok(bytes)
}

pub fn put_spv(key: u64, spv: &[u8]) -> Result<()> {
    anyhow::ensure!(CACHE.enabled, "cache disabled");
    let path = CACHE.target_dir.join(format!("{key:016x}.spv"));
    std::fs::write(&path, spv)?;
    CACHE.spv_index.lock().put(key, spv.to_vec());
    Ok(())
}

pub fn get_program(key: u64) -> Result<ProgramCacheEntry> {
    anyhow::ensure!(CACHE.enabled, "cache disabled");

    if let Some(entry) = CACHE.program_index.lock().get(&key) {
        return Ok(entry.clone());
    }

    let path = CACHE.target_dir.join(format!("{key:016x}.progbin"));
    let bytes = std::fs::read(&path)?;
    utils::reset_atime(&path).ok();
    
    let entry: ProgramCacheEntry = postcard::from_bytes(&bytes)?;
    
    CACHE.program_index.lock().put(key, entry.clone());
    Ok(entry)
}

pub fn put_program(key: u64, entry: &ProgramCacheEntry) -> Result<()> {
    anyhow::ensure!(CACHE.enabled, "cache disabled");
    let path = CACHE.target_dir.join(format!("{key:016x}.progbin"));
    
    let bytes = postcard::to_allocvec(entry)?;
    std::fs::write(&path, bytes)?;
    
    CACHE.program_index.lock().put(key, entry.clone());
    Ok(())
}

