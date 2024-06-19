use crate::parse;
use std::io::BufRead;

pub use config::Config;

pub struct CsvReader<R> {
    source: R,
    config: Config,
}

impl<R: BufRead> CsvReader<R> {
    pub fn new(source: R) -> Self {
        Self::with_config(source, Default::default())
    }

    pub fn with_config(source: R, config: Config) -> Self {
        Self { source, config }
    }

    pub fn next_row(&mut self) -> crate::Result<Vec<String>> {
        let mut line = String::new();
        loop {
            let n = self.source.read_line(&mut line)?;
            if n == 0 {
                if line.is_empty() {
                    break Err(crate::Error::StreamComplete);
                } else {
                    break Err(crate::Error::UnexpectedEof);
                }
            }
            match parse::record(&line, self.config.separator, self.config.escape) {
                Ok((_, fields)) => break Ok(fields),
                Err(e) => match e {
                    nom::Err::Incomplete(_) => {
                        continue;
                    }
                    nom::Err::Error(e) | nom::Err::Failure(e) => {
                        break Err(crate::Error::NomFailed(format!("Nom failed: {}", e)))
                    }
                },
            }
        }
    }
}

impl<R: BufRead> Iterator for CsvReader<R> {
    type Item = crate::Result<Vec<String>>;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.next_row();
        if let Err(crate::Error::StreamComplete) = row {
            return None;
        }
        Some(row)
    }
}

mod config {
    pub struct Config {
        pub has_headers: bool,
        pub separator: char,
        pub escape: char,
    }

    impl Config {
        pub fn has_headers(mut self, has: bool) -> Self {
            self.has_headers = has;
            self
        }

        pub fn separator(mut self, sep: char) -> Self {
            self.separator = sep;
            self
        }
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
