# Just CSV reader/writer
Inspired by [`csv_simple`](https://github.com/daramos/simple_csv) crate.

Conforms the [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180)

## Reader API
```rust
impl<R: BufRead> Iterator for CsvReader<R> {
    type Item = crate::Result<Vec<String>>;

}
```