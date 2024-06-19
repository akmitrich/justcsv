pub type ParseResult<'a, T> = nom::IResult<&'a str, T>;

pub fn textdata<F>(src: &str, stop: F) -> ParseResult<&str>
where
    F: Fn(char) -> bool,
{
    for (i, c) in src.char_indices() {
        if stop(c) {
            return Ok((&src[i..], &src[..i]));
        }
    }
    Ok(("", src))
}

pub fn record(src: &str) -> ParseResult<Vec<String>> {
    let stop = |c| (c < ' ' || c == ',' || c == '"');
    let (mut rest, mut field) = textdata(src, stop)?;
    let mut result = vec![];
    while !field.is_empty() {
        result.push(String::from(field));
        if !rest.is_empty() {
            rest = rest.trim_start_matches(',');
            (rest, field) = textdata(rest, stop)?;
        } else {
            break;
        }
    }
    Ok((rest, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_record() {
        let line = "мама,мыла,раму\r\n";
        println!("{:?}", record(line));
    }
}
