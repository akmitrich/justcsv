use nom::{Parser, bytes::complete::tag, multi::separated_list1};

pub type ParseResult<'a, T> = nom::IResult<&'a str, T>;

fn textdata<F>(stop: F) -> impl FnMut(&str) -> ParseResult<&str>
where
    F: Fn(char) -> bool,
{
    move |src| {
        for (i, c) in src.char_indices() {
            if stop(c) {
                return Ok((&src[i..], &src[..i]));
            }
        }
        Ok(("", src))
    }
}

fn escaped(comma: char, dquote: char) -> impl FnMut(&str) -> ParseResult<&str> {
    move |src| {
        let trimmed = src.trim_start();
        let (rest, _) = tag(format!("{}", dquote).as_str())(trimmed)?;
        println!("Escaped: {:?}; trimmed={:?}; rest={:?}", src, trimmed, rest);
        let mut char_indices = rest.char_indices().peekable();
        while let Some((i, c)) = char_indices.next() {
            if c == dquote {
                match char_indices.peek().copied() {
                    Some((j, c)) if c != dquote => {
                        let remainder = rest[j..].trim_start();
                        let next_byte = remainder.as_bytes().first().copied().unwrap_or_default();
                        if remainder.starts_with(comma) || next_byte < 0x20 {
                            return Ok((remainder, &rest[..i]));
                        } else {
                            return Err(nom::Err::Failure(nom::error::make_error(
                                src,
                                nom::error::ErrorKind::Fail,
                            )));
                        }
                    }
                    None => return Ok(("", &rest[..i])),
                    _ => {
                        let _ = char_indices.next();
                    }
                }
            }
        }
        Err(nom::Err::Incomplete(nom::Needed::Unknown))
    }
}

fn field<'a>(
    comma: char,
    dquote: char,
) -> impl Parser<&'a str, Output = String, Error = nom::error::Error<&'a str>> {
    let stop = move |c| (c < ' ' || c == comma || c == dquote);
    nom::combinator::map(
        nom::branch::alt((escaped(comma, dquote), textdata(stop))),
        move |field| {
            field.replace(
                format!("{}{}", dquote, dquote).as_str(),
                format!("{}", dquote).as_str(),
            )
        },
    )
}

pub fn record(src: &str, comma: char, dquote: char) -> ParseResult<Vec<String>> {
    separated_list1(tag(format!("{}", comma).as_str()), field(comma, dquote)).parse(src)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_just_record() {
        let line = "мама,мыла,раму\r\n";
        assert_eq!(
            vec!["мама", "мыла", "раму"],
            record(line, ',', '"').unwrap().1
        );
    }

    #[test]
    fn parse_with_escaped() {
        let line = "мама, \"мыла\",раму";
        assert_eq!(
            vec!["мама", "мыла", "раму"],
            record(line, ',', '"').unwrap().1
        );
    }

    #[test]
    fn parse_multiline() {
        let line = "мама, \"мыла\ntwo times\"\t\t,раму";
        assert_eq!(
            vec!["мама", "мыла\ntwo times", "раму"],
            record(line, ',', '"').unwrap().1
        );
    }

    #[test]
    fn fail_after_dquote() {
        let comma = ',';
        let dquote = '"';
        println!("{:?}", record("мама,мыла, \"раму\"abc", comma, dquote));
        assert!(record("мама,мыла, \"раму\"abc", comma, dquote).is_err());
        assert_eq!(
            vec!["мама", "мыла", "раму"],
            record("мама,\"мыла\", \"раму\" ", comma, dquote).unwrap().1
        );
    }

    #[test]
    fn escaped_dquote() {
        let line = "мама, \"мыла\n\"\"two times\"\"\"\t\t,раму";
        assert_eq!(
            vec!["мама", "мыла\n\"two times\"", "раму"],
            record(line, ',', '"').unwrap().1
        );
    }
}
