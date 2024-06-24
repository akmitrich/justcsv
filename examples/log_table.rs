fn main() -> Result<(), Box<dyn std::error::Error>> {
    let headers = ["x", "log(x)"];
    let mut table = vec![];
    let mut writer = justcsv::CsvWriter::new(&mut table);
    writer.headers(&headers)?;
    if std::env::args().nth(1).is_some() {
        log2(&mut writer)?;
    } else {
        natural_log(&mut writer)?;
    }
    println!("{table}", table = String::from_utf8(table)?);
    Ok(())
}

fn natural_log<W: std::io::Write>(writer: &mut justcsv::CsvWriter<W>) -> justcsv::Result<()> {
    for x in 0..u16::MAX {
        writer.write_row(&[format!("{x}"), format!("{log}", log = (x as f64).ln())])?;
    }
    Ok(())
}

fn log2<W: std::io::Write>(writer: &mut justcsv::CsvWriter<W>) -> justcsv::Result<()> {
    let data = (0..u16::MAX)
        .map(|x| vec![format!("{x}"), format!("{log}", log = (x as f64).log2())])
        .collect::<Vec<_>>();
    writer.write_document(data.as_slice())
}
