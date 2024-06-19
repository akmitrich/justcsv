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

pub fn escaped(dquote: char) -> impl FnMut(&str) -> ParseResult<&str> {
    move |src| {
        let rest = src.trim_start();
        let (rest, _) = tag(format!("{}", dquote).as_str())(rest)?;
        let mut char_indices = rest.char_indices().peekable();
        while let Some((i, c)) = char_indices.next() {
            if c == dquote {
                match char_indices.peek() {
                    Some((j, c)) if *c != dquote => {
                        return Ok((rest[*j..].trim_start(), &rest[..i]))
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
    nom::combinator::map(nom::branch::alt((escaped(dquote), textdata(stop))), |x| {
        x.replace("\"\"", "\"")
    })
}

pub fn record(src: &str) -> ParseResult<Vec<String>> {
    separated_list1(tag(","), field(',', '"'))(src)
}

#[cfg(test)]
mod tests {
    use std::io::BufRead;

    use super::*;

    #[test]
    fn it_works() {
        dbg!("1,2,3\r\n4, \"5\",6".as_bytes().lines().collect::<Vec<_>>());
    }

    #[test]
    fn parse_just_record() {
        let line = "мама,мыла,раму\r\n";
        assert_eq!(vec!["мама", "мыла", "раму"], record(line).unwrap().1);
    }

    #[test]
    fn parse_with_escaped() {
        let line = "мама, \"мыла\" ,раму";
        assert_eq!(vec!["мама", "мыла", "раму"], record(line).unwrap().1);
    }

    #[test]
    fn parse_multiline() {
        let line = "мама, \"мыла\ntwo times\"\t\t,раму";
        assert_eq!(vec!["мама", "мыла\ntwo times", "раму"], record(line).unwrap().1);
    }
}
