use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Effects};
use clap::{Command, Parser, Subcommand, ValueEnum};
use clap_complete::{Shell, generate};
use strum_macros::{Display, EnumString, IntoStaticStr};

// Configures Clap v3-style help menu colors
const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = None,
    after_help = "Use `ubigeo help` for more details.",
    styles = STYLES
)]
pub struct Cli {
    /// Path to the input CSV file containing ubigeo data
    #[arg(short, long, global = true, default_value = "data/ubigeos.csv")]
    pub input: String,

    #[command(subcommand)]
    pub subcommand: Option<AppSubcommand>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum AppSubcommand {
    /// Generate shell completions
    Completions {
        #[arg(value_enum)]
        generator: Shell,
    },
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

pub fn print_completion<G>(generator: G, cmd: &mut Command)
where
    G: clap_complete::Generator,
{
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut std::io::stdout(),
    );
}
