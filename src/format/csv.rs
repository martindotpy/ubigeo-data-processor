use crate::format::formatter::{create_output_file, Formatter};
use anyhow::Result;

pub struct CsvFormatter;

impl Formatter for CsvFormatter {
    fn format(&self, data: &crate::ubigeo::UbigeoMap, output_path: &str) -> Result<()> {
        // Ensure directory exists
        create_output_file(output_path)?;

        // Create CSV writer
        let file = create_output_file(output_path)?;
        let mut wtr = csv::Writer::from_writer(file);

        // Write header
        wtr.write_record(&["Department", "Province", "District"])?;

        // Write data
        for (department, provinces) in data {
            for (province, districts) in provinces {
                for district in districts {
                    wtr.write_record(&[department, province, district])?;
                }
            }
        }

        wtr.flush()?;

        Ok(())
    }
}
