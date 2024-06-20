mod error;
mod parse;
mod reader;

pub use error::{Error, Result};
pub use reader::{Config as CsvReaderConfig, CsvReader};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reader_works() {
        let buf = "1,2,3\n4,5,6".as_bytes();
        let mut reader = reader::CsvReader::new(buf);
        assert_eq!(
            vec!["1", "2", "3"],
            reader.next().unwrap().unwrap().into_vec()
        );
        assert_eq!(
            vec!["4", "5", "6"],
            reader.next().unwrap().unwrap().into_vec()
        );
        assert!(reader.next().is_none())
    }

    #[test]
    fn empty_reader() {
        let buf = "".as_bytes();
        let mut reader = reader::CsvReader::new(buf);
        assert!(dbg!(reader.next()).is_none());
    }

    #[test]
    fn escaped_read() {
        let buf = "\"1\",2,3\r\n4, \"5\",6".as_bytes();
        let mut reader = reader::CsvReader::new(buf);
        assert_eq!(
            vec!["1", "2", "3"],
            reader.next().unwrap().unwrap().into_vec()
        );
        assert_eq!(
            vec!["4", "5", "6"],
            reader.next().unwrap().unwrap().into_vec()
        );
        assert!(reader.next().is_none())
    }

    #[test]
    fn multiline_read() {
        let buf = "\"1\",2,3\r\n4, \"everybody needs\nmilk\",6".as_bytes();
        let mut reader = reader::CsvReader::new(buf);
        assert_eq!(
            vec!["1", "2", "3"],
            reader.next().unwrap().unwrap().into_vec()
        );
        assert_eq!(
            vec!["4", "everybody needs\nmilk", "6"],
            reader.next().unwrap().unwrap().into_vec()
        );
        assert!(reader.next().is_none())
    }

    #[test]
    fn quoted_read_fail() {
        let buf =
            "\"1\",2,3\r\n4, \"everybody needs\nmilk,6\r\nsome other stuff which is never read"
                .as_bytes();
        let mut reader = reader::CsvReader::new(buf);
        assert_eq!(
            vec!["1", "2", "3"],
            reader.next().unwrap().unwrap().into_vec()
        );
        assert!(reader.next().unwrap().is_err());
        assert!(reader.next().is_none())
    }

    #[test]
    fn read_last_line() {
        let buf = "1,2,3\r\n4,5,\r\n".as_bytes();
        let mut reader = reader::CsvReader::new(buf);
        assert_eq!(
            vec!["1", "2", "3"],
            reader.next().unwrap().unwrap().into_vec()
        );
        assert_eq!(
            vec!["4", "5", ""],
            reader.next().unwrap().unwrap().into_vec()
        );
        assert!(reader.next().is_none())
    }

    #[test]
    fn read_bad_escaping() {
        let buf = "1,2,3\r\n4,\"5\"xyz,6\r\n\"7\",8,9".as_bytes();
        let mut reader = reader::CsvReader::new(buf);
        // println!("Line 1: {:?}", reader.next());
        // println!("Line 2: {:?}", reader.next());
        assert_eq!(
            vec!["1", "2", "3"],
            reader.next().unwrap().unwrap().into_vec()
        );
        assert!(reader.next().unwrap().is_err());
        assert_eq!(
            vec!["7", "8", "9"],
            reader.next().unwrap().unwrap().into_vec()
        )
    }
}
