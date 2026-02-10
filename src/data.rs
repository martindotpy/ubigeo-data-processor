#[cfg(feature = "data")]
use include_dir::{Dir, include_dir};

#[cfg(feature = "data")]
static DATA_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/data");

/// Return the embedded `data/ubigeos.csv` contents as bytes.
/// This function is only available when the `data` feature is enabled.
#[cfg(feature = "data")]
pub fn embedded_ubigeos_csv() -> &'static [u8] {
    let file = DATA_DIR
        .get_file("ubigeos.csv")
        .expect("embedded data/ubigeos.csv not found");
    file.contents()
}

// When `data` feature is disabled this file is not compiled because
// the `data` module is conditionally declared with `#[cfg(feature = "data")]`.
