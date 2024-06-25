use crate::parse;
use std::io::BufRead;

pub use config::Config as CsvReaderConfig;

/// CSV reader for UTF-8 comma separated content.
/// Use it as an iterator to read records line by line.
///
/// # Example
///
/// ```
/// let buf = "\"1\",2,3\r\n4, \"everybody needs\nmilk\",6".as_bytes();
/// let mut reader = justcsv::CsvReader::new(buf);
/// assert_eq!(
///     vec!["1", "2", "3"],
///     reader.next().unwrap().unwrap().into_vec()
/// );
/// assert_eq!(
///     vec!["4", "everybody needs\nmilk", "6"],
///     reader.next().unwrap().unwrap().into_vec()
/// );
/// assert!(reader.next().is_none());
/// ```
pub struct CsvReader<R> {
    source: R,
    config: CsvReaderConfig,
    headers: Option<Box<[String]>>,
}

impl<R: BufRead> CsvReader<R> {
    /// Create reader with default options
    pub fn new(source: R) -> Self {
        Self::with_config(source, Default::default())
    }

    /// Create reader with options passed as [config](CsvReaderConfig)
    pub fn with_config(mut source: R, config: CsvReaderConfig) -> Self {
        let headers = if config.has_headers {
            // if parsing headers fails self.headers is None but self.config.has_headers is still true
            load_headers(&mut source, config.separator, config.escape)
        } else {
            None
        };
        Self {
            source,
            config,
            headers,
        }
    }

    /// Returns CSV headers if they are expected for the stream
    pub fn headers(&self) -> Option<&[String]> {
        self.headers.as_deref()
    }

    fn next_row(&mut self) -> crate::Result<Box<[String]>> {
        parse_row(&mut self.source, self.config.separator, self.config.escape)
    }
}

impl<R: BufRead> Iterator for CsvReader<R> {
    type Item = crate::Result<Box<[String]>>;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.next_row();
        if let Err(crate::Error::StreamComplete) = row {
            return None;
        }
        Some(row)
    }
}

fn parse_row<R: BufRead>(mut source: R, comma: char, dquote: char) -> crate::Result<Box<[String]>> {
    let mut record_line = String::new();
    loop {
        let n = source.read_line(&mut record_line)?;
        if n == 0 {
            if record_line.is_empty() {
                break Err(crate::Error::StreamComplete);
            } else {
                // if source exhausted but we have incomplete record parsing fails
                break Err(crate::Error::UnexpectedEof);
            }
        }
        match parse::record(&record_line, comma, dquote) {
            Ok((_, fields)) => break Ok(fields.into_boxed_slice()),
            Err(e) => match e {
                nom::Err::Incomplete(_) => {
                    //record in CSV-file may consist of several lines if has escaped fields with newlines inside
                    continue;
                }
                nom::Err::Error(e) | nom::Err::Failure(e) => {
                    break Err(crate::Error::NomFailed(format!("Nom failed: {}", e)))
                }
            },
        }
    }
}

fn load_headers<R: BufRead>(source: R, comma: char, dquote: char) -> Option<Box<[String]>> {
    parse_row(source, comma, dquote).ok()
}

mod config {

    /// Data struct with CSV reader options
    pub struct Config {
        /// User expects CSV headers
        pub has_headers: bool,
        /// Value separator, default is ',' as in RFC 4180
        pub separator: char,
        /// Escape character, default is '"' as in RFC 4180
        pub escape: char,
    }

    impl Config {
        /// Part of a Builder pattern. Sets headers flag
        pub fn has_headers(mut self, has: bool) -> Self {
            self.has_headers = has;
            self
        }

        /// Part of a Builder pattern. Sets custom separator value
        pub fn separator(mut self, sep: char) -> Self {
            self.separator = sep;
            self
        }

        /// Part of a Builder pattern. Sets custom escape character value
        pub fn escape(mut self, esc: char) -> Self {
            self.escape = esc;
            self
        }
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                has_headers: Default::default(),
                separator: ',',
                escape: '"',
            }
        }
    }
}
