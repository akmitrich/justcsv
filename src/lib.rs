mod error;
mod parse;
mod reader;
mod writer;

pub use error::{Error, Result};
pub use reader::{CsvReader, CsvReaderConfig};
pub use writer::{CsvWriter, CsvWriterConfig, NewLine};

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

    #[test]
    fn read_headers() {
        let buf = "Col 1,Col 2, \"Col 3\"\r\n1,2,3\r\n4,\"5\",6".as_bytes();
        let mut reader = reader::CsvReader::with_config(
            buf,
            CsvReaderConfig {
                has_headers: true,
                ..Default::default()
            },
        );
        assert_eq!(
            vec!["Col 1", "Col 2", "Col 3"],
            reader.headers().unwrap().to_vec()
        );
        assert_eq!(
            vec!["1", "2", "3"],
            reader.next().unwrap().unwrap().into_vec()
        );
        assert_eq!(
            vec!["4", "5", "6"],
            reader.next().unwrap().unwrap().into_vec()
        );
    }

    #[test]
    fn writer_works() {
        let mut buf = Vec::new();
        let row = vec!["1", "2", "3"];
        let mut writer = CsvWriter::new(&mut buf);
        writer.write_row(&row).unwrap();
        writer.write_row(["4", "5\"abc\"X", "6\n\tagain"]).unwrap();
        assert_eq!(
            b"1,2,3\r\n4,\"5\"\"abc\"\"X\",\"6\n\tagain\"",
            buf.as_slice()
        );
        println!("CSV: {}", String::from_utf8(buf).unwrap());
    }
}
// println!("Line 1: {:?}", reader.next());
// println!("Line 2: {:?}", reader.next());
