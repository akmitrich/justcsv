# Just CSV reader/writer
Inspired by [`csv_simple`](https://github.com/daramos/simple_csv) crate.

Conforms the [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180). Please be aware of that RFC. According to RFC CSV-file is ASCII-only, this crate treats CSV-files as UTF-8.

## Reader API
```rust
impl<R: BufRead> reader::CsvReader<R> {
	pub fn new(reader: R) -> Self {...}
	pub fn with_config(source: R, config: Config) -> Self {...}
	pub fn headers(&self) -> Option<&[String]> {...}
}

impl<R: BufRead> Iterator for reader::CsvReader<R> {
	type Item = crate::Result<Box<[String]>>;
	...
}

struct Config {
  pub has_headers: bool,
  pub separator: char,
  pub escape: char,
}
```

Config struct exported as `CsvReaderConfig` in `lib.rs` implements builder pattern and Default trait.

`CsvReader::headers(...)` returns `Some` if `config.has_headers == true` and parsing first record was successful. If you passed `config.has_headers == true` but get `None` from `CsvReader::headers(...)` means parsing first record failed. Note that iterating through such a reader is undefined behaviour.

## Writer API
Is not as convinient as Reader API. It just helps you to escape special characters while writing to the `dest`. 

Headers have no special treatment, they are just another row. Only if you try to write them after records you get an error.

```rust
impl<W: Write> CsvWriter<W> {
	pub fn new(dest: W) -> Self {...}
	pub fn with_config(dest: W, config: CsvWriterConfig) -> Self {...}
	pub fn write_row(&mut self, row: &[impl AsRef<str>]) -> crate::Result<()> {...}
	pub fn headers(&mut self, headers: &[impl AsRef<str>]) -> crate::Result<()> {...}
	pub fn write_doc(&mut self, doc: &[&[impl AsRef<str>]]) -> crate::Result<()> {...}
}

// as CsvWriterConfig
pub struct Config {
	pub separator: char,
  	pub escape: char,
  	pub newline: NewLine,
}

pub enum NewLine {
  	Rfc,
  	Unix,
  	Custom(String),
}
```
Config struct exported as `CsvWriterConfig` in `lib.rs` implements builder pattern and Default trait.

# License
Due to [nom](https://github.com/rust-bakery/nom) the crate has [MIT License](LICENSE).