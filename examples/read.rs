fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::PathBuf::from(std::env::args().nth(1).unwrap());
    let input = std::fs::File::open(path)?;
    let buf = std::io::BufReader::new(input);
    let reader = justcsv::CsvReader::new(buf);
    for (line, record) in reader.enumerate() {
        let record = record?;
        println!("{:4}. {:?}", line, record);
    }
    Ok(())
}
