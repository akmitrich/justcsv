pub use config::{Config as CsvWriterConfig, NewLine};

use std::io::Write;

pub struct CsvWriter<W> {
    dest: W,
    config: CsvWriterConfig,
    is_dirty: bool,
}

impl<W: Write> CsvWriter<W> {
    pub fn new(dest: W) -> Self {
        Self::with_config(dest, Default::default())
    }

    pub fn with_config(dest: W, config: CsvWriterConfig) -> Self {
        Self {
            dest,
            config,
            is_dirty: false,
        }
    }

    pub fn write_row(&mut self, row: &[impl AsRef<str>]) -> crate::Result<()> {
        if self.is_dirty {
            write!(self.dest, "{}", self.config.newline)?;
        } else {
            self.is_dirty = true;
        }
        let output = row
            .iter()
            .map(|field| self.escape_if_needed(field.as_ref()))
            .collect::<Vec<_>>()
            .join(",");
        write!(self.dest, "{}", output)?;
        Ok(())
    }

    pub fn headers(&mut self, headers: &[impl AsRef<str>]) -> crate::Result<()> {
        if self.is_dirty {
            return Err(crate::Error::WriteHeadersAfterRecords);
        }
        self.write_row(headers)
    }

    pub fn write_doc(&mut self, doc: &[&[impl AsRef<str>]]) -> crate::Result<()> {
        for row in doc.iter() {
            self.write_row(row)?;
        }
        Ok(())
    }

    fn escape_if_needed(&self, field: &str) -> String {
        if field
            .chars()
            .any(|c| c < ' ' || c == self.config.separator || c == self.config.escape)
        {
            format!(
                "\"{}\"",
                field.replace(
                    self.config.escape.to_string().as_str(),
                    format!("{}{}", self.config.escape, self.config.escape).as_str(),
                )
            )
        } else {
            field.to_owned()
        }
    }
}

mod config {
    pub enum NewLine {
        Rfc,
        Unix,
        Custom(String),
    }

    impl std::fmt::Display for NewLine {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    NewLine::Rfc => "\r\n",
                    NewLine::Unix => "\n",
                    NewLine::Custom(end_of_line) => end_of_line.as_str(),
                }
            )
        }
    }

    pub struct Config {
        pub separator: char,
        pub escape: char,
        pub newline: NewLine,
    }

    impl Config {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn separator(mut self, comma: char) -> Self {
            self.separator = comma;
            self
        }

        pub fn escape(mut self, dquote: char) -> Self {
            self.escape = dquote;
            self
        }

        pub fn rfc_end_of_line(mut self) -> Self {
            self.newline = NewLine::Rfc;
            self
        }

        pub fn unix_end_of_line(mut self) -> Self {
            self.newline = NewLine::Unix;
            self
        }

        pub fn custom_end_of_line(mut self, eoln: impl ToString) -> Self {
            self.newline = NewLine::Custom(eoln.to_string());
            self
        }
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                separator: ',',
                escape: '"',
                newline: NewLine::Rfc,
            }
        }
    }
}
