<h1 align="center">
  Ubigeo Data Processor 🇵🇪
</h1>

A small command-line tool to transform an input UBIGEO CSV (department →
province → district) into one of several output formats: CSV, JSON, or SQL
scripts.

This README is focused for end users: installation, usage, and examples.

## Features

- Parses a CSV with INEI-style columns (`desc_dep_inei`, `desc_prov_inei`,
  `desc_ubigeo_inei`).
- Normalizes names (capitalization, handles Spanish particles like `de`/`del`).
- Removes duplicates and outputs a nested
  `department -> province -> [districts]` structure.
- Exports to CSV, pretty JSON, or SQL scripts targeting Postgres, MySQL, or
  SQLite.

## Usage

The tool exposes three subcommands: `csv`, `json`, and `sql`.

Global option:

- `-i, --input <PATH>`: Path to the input CSV (default: `data/ubigeos.csv`).

Subcommands:

- `csv <OUTPUT>` — Write a flat CSV with header `Department,Province,District`.
- `json <OUTPUT>` — Write a pretty-printed JSON file containing the nested map.
- `sql <OUTPUT>` — Generate a SQL script. Additional flags:
  - `--dialect <postgres|mysql|sqlite>` (default: `postgres`)
  - `--table-department <name>` (default: `department`)
  - `--table-province <name>` (default: `province`)
  - `--table-district <name>` (default: `district`)

Examples (using `cargo run`):

```bash
# Use default input and produce CSV
ubigeo csv sql_out/ubigeos_out.csv

# Produce pretty JSON
ubigeo json sql_out/ubigeos.json

# Generate a MySQL script with custom table names
ubigeo sql sql_out/mysql.sql --dialect mysql --table-department dept --table-province prov --table-district dist
```

Notes about SQL output:

- The generated SQL assumes the tables already exist with column like `id`.
- Dialect differences:
  - MySQL uses `LAST_INSERT_ID()` and session variables.
  - Postgres uses `RETURNING id INTO` inside a `DO $$` block.
  - SQLite scripts use `SELECT MAX(id)` as a simple fallback.

## Input format

By default the tool reads `data/ubigeos.csv`. Expected columns (headers):

- `desc_dep_inei` — department name
- `desc_prov_inei` — province name
- `desc_ubigeo_inei` — district name

The processor will normalize names, skip empty or invalid departments (e.g.,
`NA`), and deduplicate districts.
