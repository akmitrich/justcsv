use std::io::BufRead;

pub struct CsvReader<R> {
    source: std::io::Lines<R>,
}

impl<R: BufRead> CsvReader<R> {
    pub fn new(reader: R) -> Self {
        let source = reader.lines();
        Self { source }
    }

    pub fn next_row(&mut self) -> crate::Result<Vec<String>> {
        let line = self.source.next().ok_or(crate::Error::Eof)??;
        todo!()
    }
}

impl<R: BufRead> Iterator for CsvReader<R> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_row().ok()
    }
}
