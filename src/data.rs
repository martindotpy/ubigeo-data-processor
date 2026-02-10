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

#[cfg(not(feature = "data"))]
/// When `data` feature is disabled, this function is not available.
pub fn embedded_ubigeos_csv() -> &'static [u8] {
    compile_error!("embedded_ubigeos_csv is only available with feature \"data\"");
}
