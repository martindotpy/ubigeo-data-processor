use anyhow::{Context, Result};
use csv::Reader;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
#[cfg(feature = "data")]
use std::io::Cursor;

use crate::ubigeo::UbigeoMap;

// Record structure for CSV deserialization
#[derive(Debug, Deserialize)]
struct UbigeoRecord {
    #[serde(rename = "desc_dep_inei")]
    department: String,

    #[serde(rename = "desc_prov_inei")]
    province: String,

    #[serde(rename = "desc_ubigeo_inei")]
    district: String,
}

fn to_namecase(s: &str) -> String {
    // Split the string into words
    let words: Vec<&str> = s.split_whitespace().collect();

    // Handle empty string
    if words.is_empty() {
        return String::new();
    }

    let mut result = String::with_capacity(s.len());

    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }

        let lower = word.to_lowercase();

        // Handle Spanish particles
        if i > 0 && (lower == "de" || lower == "del") {
            result.push_str(&lower);
            continue;
        }

        let mut c = word.chars();

        if let Some(f) = c.next() {
            result.extend(f.to_uppercase());
            result.push_str(&c.as_str().to_lowercase());
        }
    }

    result
}

pub fn process_ubigeo_data(input_path: &str) -> Result<UbigeoMap> {
    // Choose source: either the filesystem or the embedded CSV when the
    // default data path is requested and the `data` feature is enabled.
    let rdr = {
        #[cfg(feature = "data")]
        {
            if input_path == "data/ubigeos.csv" {
                let bytes = crate::data::embedded_ubigeos_csv();

                return _process_reader(Reader::from_reader(Cursor::new(bytes)));
            }
        }

        // Fallback to reading from filesystem
        let file = File::open(input_path)
            .with_context(|| format!("Failed to open file: {}", input_path))?;
        Reader::from_reader(BufReader::new(file))
    };

    // If we reached here, `rdr` is ready to be used
    _process_reader(rdr)
}

fn _process_reader<R: std::io::Read>(mut rdr: Reader<R>) -> Result<UbigeoMap> {
    // Initialize the Ubigeo map
    let mut ubigeo_map: UbigeoMap = HashMap::new();

    // Process each record in the CSV
    for result in rdr.deserialize() {
        let record: UbigeoRecord = result.with_context(|| "Failed to deserialize record")?;

        // Clean and extract fields
        let department = to_namecase(record.department.trim());

        if department.eq_ignore_ascii_case("na") || department.is_empty() {
            continue;
        }

        let province = to_namecase(record.province.trim());
        let district = to_namecase(record.district.trim());

        // Insert into the Ubigeo map
        let provinces = ubigeo_map.entry(department).or_default();
        let districts = provinces.entry(province).or_default();

        // Avoid duplicates
        if !districts.contains(&district) {
            districts.push(district);
        }
    }

    Ok(ubigeo_map)
}
