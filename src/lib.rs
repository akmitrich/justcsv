mod error;
mod parse;
mod reader;

pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reader_works() {
        let buf = "1,2,3\n4,5,6".as_bytes();
        let mut reader = reader::CsvReader::new(buf);
        assert_eq!(vec!["1", "2", "3"], reader.next().unwrap());
        assert_eq!(vec!["4", "5", "6"], reader.next().unwrap());
        assert!(reader.next().is_none())
    }

    #[test]
    fn empty_reader() {
        let buf = "".as_bytes();
        let mut reader = reader::CsvReader::new(buf);
        assert!(dbg!(reader.next()).is_none());
    }
}
