use nom::{bytes::complete::tag, multi::separated_list1};

pub type ParseResult<'a, T> = nom::IResult<&'a str, T>;

pub fn textdata<F>(stop: F) -> impl FnMut(&str) -> ParseResult<&str>
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

pub fn escaped(comma: char, dquote: char) -> impl FnMut(&str) -> ParseResult<&str> {
    move |src| {
        let rest = src.trim_start();
        let (rest, _) = tag(format!("{}", dquote).as_str())(rest)?;
        let mut char_indices = rest.char_indices().peekable();
        while let Some((i, c)) = char_indices.next() {
            if c == dquote {
                match char_indices.peek().copied() {
                    Some((j, c)) if c != dquote => {
                        let remainder = rest[j..].trim_start();
                        let next_byte = remainder.as_bytes().first().copied();
                        if remainder.starts_with(comma) || next_byte.unwrap_or_default() < 0x20 {
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

pub fn field<'a>(comma: char, dquote: char) -> impl FnMut(&'a str) -> ParseResult<String> {
    let stop = move |c| (c < ' ' || c == comma || c == dquote);
    nom::combinator::map(
        nom::branch::alt((escaped(comma, dquote), textdata(stop))),
        |field| field.replace("\"\"", "\""),
    )
}

pub fn record(src: &str, comma: char, dquote: char) -> ParseResult<Vec<String>> {
    separated_list1(tag(format!("{}", comma).as_str()), field(comma, dquote))(src)
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
}
