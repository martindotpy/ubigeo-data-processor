use crate::{
    command::{AppSubcommand, Cli, print_completion},
    format::{csv::CsvFormatter, formatter::Formatter, json::JsonFormatter, sql::SqlFormatter},
    processor::ubigeo_data_processor::process_ubigeo_data,
};
use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};

mod command;
mod data;
mod format;
mod processor;
mod ubigeo;

fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Ensure a subcommand was provided
    let Some(command) = cli.subcommand else {
        Cli::command().print_help()?;

        std::process::exit(1);
    };

    // Handle generate subcommand early to avoid processing data
    if let AppSubcommand::Completions { generator } = &command {
        print_completion(generator.clone(), &mut Cli::command());

        return Ok(());
    }

    // Load and process ubigeo data
    let ubigeo_tree = process_ubigeo_data(&cli.input).context("Failed to process ubigeo data")?;

    // Handle commands
    let (formatter, output_path): (Box<dyn Formatter>, String) = match command {
        AppSubcommand::Csv { output } => (Box::new(CsvFormatter), output),
        AppSubcommand::Json { output } => (Box::new(JsonFormatter), output),
        AppSubcommand::Sql {
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
        AppSubcommand::Completions { .. } => unreachable!(),
    };

    // Format and output data
    formatter
        .format(&ubigeo_tree, &output_path)
        .with_context(|| format!("Failed to format data to {}", output_path))?;

    Ok(())
}
