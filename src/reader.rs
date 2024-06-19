use std::io::BufRead;

use crate::parse;

pub struct CsvReader<R> {
    source: R,
}

impl<R: BufRead> CsvReader<R> {
    pub fn new(source: R) -> Self {
        Self { source }
    }

    pub fn next_row(&mut self) -> crate::Result<Vec<String>> {
        let mut line = String::new();
        loop {
            let n = self.source.read_line(&mut line)?;
            if n == 0 {
                break Err(crate::Error::Eof);
            }
            let x = parse::record(&line);
            match x {
                Ok((_, fields)) => break Ok(fields),
                Err(e) => match e {
                    nom::Err::Incomplete(_) => {
                        continue;
                    }
                    nom::Err::Error(e) | nom::Err::Failure(e) => {
                        break Err(format!("Nom failed: {}", e).into())
                    }
                },
            }
        }
    }
}

impl<R: BufRead> Iterator for CsvReader<R> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_row().ok()
    }
}
