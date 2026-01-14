mod command;
mod format;
mod processor;
mod ubigeo;

use anyhow::{Context, Result};
use clap::Parser;

use crate::{
    command::{Cli, FormatCommand},
    format::{csv::CsvFormatter, formatter::Formatter, json::JsonFormatter, sql::SqlFormatter},
    processor::ubigeo_data_processor::process_ubigeo_data,
};

fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Load and process ubigeo data
    let ubigeo_tree = process_ubigeo_data(&cli.input)
        .context("Failed to process ubigeo data")?;

    // Handle commands
    let (formatter, output_path): (Box<dyn Formatter>, String) = match cli.command {
        FormatCommand::Csv { output } => (Box::new(CsvFormatter), output),
        FormatCommand::Json { output } => (Box::new(JsonFormatter), output),
        FormatCommand::Sql {
            output,
            dialect,
            table_department,
            table_province,
            table_district,
        } => (
            Box::new(SqlFormatter {
                dialect,
                table_department,
                table_province,
                table_district,
            }),
            output,
        ),
    };

    // Format and output data
    formatter
        .format(&ubigeo_tree, &output_path)
        .with_context(|| format!("Failed to format data to {}", output_path))?;

    Ok(())
}
