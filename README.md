# Just CSV reader/writer
Inspired by [`csv_simple`](https://github.com/daramos/simple_csv) crate.

Conforms the [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180). Please be aware of that RFC.

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