pub use config::{Config as CsvWriterConfig, NewLine};

use std::io::Write;

/// CSV writer to save UTF-8 content as comma separated values.
/// Basically CSV document is a slice of records which are basically `&str` slices.
///
/// # Example
///
/// ```
/// let mut buf = Vec::new();
/// let row = vec!["1", "2", "3"];
/// let mut writer = justcsv::CsvWriter::new(&mut buf);
/// writer.write_row(&row).unwrap();
/// writer.write_row(["4", "5\"abc\"X", "6\n\tagain"]).unwrap();
/// assert_eq!(
///     b"1,2,3\r\n4,\"5\"\"abc\"\"X\",\"6\n\tagain\"",
///     buf.as_slice()
/// );
/// ```
pub struct CsvWriter<W> {
    dest: W,
    config: CsvWriterConfig,
    is_dirty: bool,
}

impl<W: Write> CsvWriter<W> {
    /// Creates a CSV writer with default options
    pub fn new(dest: W) -> Self {
        Self::with_config(dest, Default::default())
    }

    /// Creates a CSV writer with options passed as [config](CsvWriterConfig)
    pub fn with_config(dest: W, config: CsvWriterConfig) -> Self {
        Self {
            dest,
            config,
            is_dirty: false,
        }
    }

    /// Save next row of comma separated values
    pub fn write_row<Field: AsRef<str>>(&mut self, row: impl AsRef<[Field]>) -> crate::Result<()> {
        if self.is_dirty {
            write!(self.dest, "{}", self.config.newline)?;
        } else {
            self.is_dirty = true;
        }
        let output = row
            .as_ref()
            .iter()
            .map(|field| self.escape_if_needed(field.as_ref()))
            .collect::<Vec<_>>()
            .join(self.config.separator.as_str());
        write!(self.dest, "{}", output)?;
        Ok(())
    }

    /// Save CSV headers. Basically the same as `write_row` but returns error if headers are saved after any records
    pub fn write_headers(&mut self, headers: &[impl AsRef<str>]) -> crate::Result<()> {
        if self.is_dirty {
            return Err(crate::Error::WriteHeadersAfterRecords);
        }
        self.write_row(headers)
    }

    /// Save whole CSV document
    pub fn write_document<Field: AsRef<str>, Record: AsRef<[Field]>>(
        &mut self,
        doc: &[Record],
    ) -> crate::Result<()> {
        for row in doc.iter() {
            self.write_row(row)?;
        }
        Ok(())
    }

    fn escape_if_needed(&self, field: &str) -> String {
        if field
            .chars()
            .any(|c| c < ' ' || self.config.separator.contains(c) || c == self.config.escape)
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
    /// New line type
    pub enum NewLine {
        /// According RFC 4180 `CRLF`
        Rfc,
        /// Unix style option: '\n'
        Unix,
        /// Custom end of line characters
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

    /// Data struct with CSV writer options
    pub struct Config {
        /// Value separator, default is ','
        pub separator: String,
        /// Escape character, default is '"'
        pub escape: char,
        /// New line type
        pub newline: NewLine,
    }

    impl Config {
        /// Create config with default options
        pub fn new() -> Self {
            Default::default()
        }

        /// Part of Builder pattern. Sets custom value separator
        pub fn separator(mut self, comma: char) -> Self {
            self.separator = comma.to_string();
            self
        }

        /// Part of Builder pattern. Sets custom escape character instead of '"'
        pub fn escape(mut self, dquote: char) -> Self {
            self.escape = dquote;
            self
        }

        /// Part of Builder pattern. Sets end of line according RFC 4180 (default value)
        pub fn rfc_end_of_line(mut self) -> Self {
            self.newline = NewLine::Rfc;
            self
        }

        /// Part of Builder pattern. Sets end of line in Unix style, i.e. '\n'
        pub fn unix_end_of_line(mut self) -> Self {
            self.newline = NewLine::Unix;
            self
        }

        /// Part of Builder pattern. Sets custom end of line
        pub fn custom_end_of_line(mut self, eoln: impl ToString) -> Self {
            self.newline = NewLine::Custom(eoln.to_string());
            self
        }
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                separator: String::from(","),
                escape: '"',
                newline: NewLine::Rfc,
            }
        }
    }
}
