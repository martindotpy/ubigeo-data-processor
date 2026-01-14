use anyhow::Result;
use std::fs::{self, File};
use std::path::Path;

pub trait Formatter {
    fn format(&self, data: &crate::ubigeo::UbigeoMap, output_path: &str) -> Result<()>;
}

/// Helper function to create parent directories if they don't exist
pub fn create_output_file(path: &str) -> Result<File> {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    Ok(File::create(path)?)
}
