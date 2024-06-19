mod error;
mod parse;
mod reader;

pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reader_works() {
        let buf = "1,2,3\r\n4,5,6".as_bytes();
        let mut reader = reader::CsvReader::new(buf);
        assert_eq!(
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            reader.next().unwrap()
        )
    }
}
