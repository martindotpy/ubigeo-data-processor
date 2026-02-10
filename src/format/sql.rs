use crate::command::SqlDialect;
use crate::format::formatter::{Formatter, create_output_file};
use anyhow::Result;
use std::io::Write;

pub struct SqlFormatter {
    pub dialect: SqlDialect,
    pub table_department: String,
    pub table_province: String,
    pub table_district: String,
}

impl Formatter for SqlFormatter {
    fn format(&self, data: &crate::ubigeo::UbigeoMap, output_path: &str) -> Result<()> {
        let mut file = create_output_file(output_path)?;

        // Helper closure to sanitize SQL strings
        let escape = |s: &str| s.replace("'", "''");

        match self.dialect {
            SqlDialect::Mysql => {
                for (department, provinces) in data {
                    // Deparment insertion
                    writeln!(
                        file,
                        "INSERT INTO {} (name) VALUES ('{}');",
                        self.table_department,
                        escape(department)
                    )?;
                    writeln!(file, "SET @dep_id = LAST_INSERT_ID();")?;

                    for (province, districts) in provinces {
                        // Province insertion
                        writeln!(
                            file,
                            "INSERT INTO {} (name, {}_id) VALUES ('{}', @dep_id);",
                            self.table_province,
                            self.table_department,
                            escape(province)
                        )?;
                        writeln!(file, "SET @prov_id = LAST_INSERT_ID();")?;

                        // District insertion
                        writeln!(
                            file,
                            "INSERT INTO {} (name, {}_id)",
                            self.table_district, self.table_province
                        )?;
                        writeln!(file, "  VALUES")?;

                        for district in districts {
                            write!(file, "    ('{}', @prov_id)", escape(district))?;
                            if district != districts.last().unwrap() {
                                writeln!(file, ",")?;
                            } else {
                                writeln!(file, ";")?;
                            }
                        }
                    }
                }
            }
            SqlDialect::Postgres => {
                writeln!(file, "DO $$")?;
                writeln!(file, "DECLARE")?;
                writeln!(file, "    d_id integer;")?;
                writeln!(file, "    p_id integer;")?;
                writeln!(file, "BEGIN")?;

                for (department, provinces) in data {
                    writeln!(
                        file,
                        "    INSERT INTO {} (name) VALUES ('{}') RETURNING id INTO d_id;",
                        self.table_department,
                        escape(department)
                    )?;

                    for (province, districts) in provinces {
                        writeln!(
                            file,
                            "    INSERT INTO {} (name, {}_id) VALUES ('{}', d_id) RETURNING id INTO p_id;",
                            self.table_province,
                            self.table_department,
                            escape(province)
                        )?;

                        writeln!(
                            file,
                            "    INSERT INTO {} (name, {}_id)",
                            self.table_district, self.table_province
                        )?;
                        writeln!(file, "      VALUES")?;

                        for (i, district) in districts.iter().enumerate() {
                            write!(file, "        ('{}', p_id)", escape(district))?;

                            if i < districts.len() - 1 {
                                writeln!(file, ",")?;
                            } else {
                                writeln!(file, ";")?;
                            }
                        }
                    }
                }
                writeln!(file, "END $$;")?;
            }
            SqlDialect::Sqlite => {
                // SQLite doesn't support variables well in scripts, using subqueries with MAX(id)
                // Assuming sequential insertion
                for (department, provinces) in data {
                    writeln!(
                        file,
                        "INSERT INTO {} (name) VALUES ('{}');",
                        self.table_department,
                        escape(department)
                    )?;

                    for (province, districts) in provinces {
                        writeln!(
                            file,
                            "INSERT INTO {} (name, {}_id) VALUES ('{}', (SELECT MAX(id) FROM {}));",
                            self.table_province,
                            self.table_department,
                            escape(province),
                            self.table_department
                        )?;

                        writeln!(
                            file,
                            "INSERT INTO {} (name, {}_id)",
                            self.table_district, self.table_province
                        )?;
                        writeln!(file, "  VALUES")?;

                        for (i, district) in districts.iter().enumerate() {
                            write!(
                                file,
                                "    ('{}', (SELECT MAX(id) FROM {}))",
                                escape(district),
                                self.table_province
                            )?;

                            if i < districts.len() - 1 {
                                writeln!(file, ",")?;
                            } else {
                                writeln!(file, ";")?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
