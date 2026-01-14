use clap::{Parser, Subcommand, ValueEnum};
use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the input CSV file containing ubigeo data
    #[arg(short, long, global = true, default_value = "data/ubigeos.csv")]
    pub input: String,

    #[command(subcommand)]
    pub command: FormatCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FormatCommand {
    /// Export data to CSV format
    Csv {
        /// Output file path
        output: String,
    },
    /// Export data to JSON format
    Json {
        /// Output file path
        output: String,
    },
    /// Export data to SQL format
    Sql {
        /// Output file path
        output: String,

        /// SQL dialect to use
        #[arg(long, short, default_value_t)]
        dialect: SqlDialect,

        /// Department table name
        #[arg(long, default_value = "department")]
        table_department: String,

        /// Province table name
        #[arg(long, default_value = "province")]
        table_province: String,

        /// District table name
        #[arg(long, default_value = "district")]
        table_district: String,
    },
}

#[derive(Clone, Debug, ValueEnum, Default, Display, EnumString, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum SqlDialect {
    #[default]
    Postgres,
    Mysql,
    Sqlite,
}
