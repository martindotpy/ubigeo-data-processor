use crate::format::formatter::{create_output_file, Formatter};
use anyhow::Result;

pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(&self, data: &crate::ubigeo::UbigeoMap, output_path: &str) -> Result<()> {
        let file = create_output_file(output_path)?;

        serde_json::to_writer_pretty(file, data)?;

        Ok(())
    }
}
