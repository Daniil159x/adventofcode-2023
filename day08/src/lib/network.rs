use std::collections::HashMap;

#[derive(Debug)]
pub struct Network<'a> {
    pub path: &'a str,
    pub map: HashMap<&'a str, (&'a str, &'a str)>,
}

#[derive(Debug)]
pub enum NetworkParseError {
    NoLineWithPath(),
    WrongFormat(),
    WrongNone(),
}

fn parse_node(line: &str) -> Option<(&str, (&str, &str))> {
    let (name, left_right_str) = line.split_once('=')?;
    let (left, right) = left_right_str.split_once(',')?;

    let to_trim: &[_] = &[' ', '(', ')'];

    let name = name.trim();
    let left = left.trim_matches(to_trim);
    let right = right.trim_matches(to_trim);
    Some((name, (left, right)))
}

impl<'a> TryFrom<&'a str> for Network<'a> {
    type Error = NetworkParseError;

    fn try_from(content: &'a str) -> Result<Self, Self::Error> {
        let mut lines = content.lines();

        let path = lines.next().ok_or(NetworkParseError::NoLineWithPath())?;

        let padding_line = lines.next().ok_or(NetworkParseError::WrongFormat())?.trim();
        if !padding_line.is_empty() {
            return Err(NetworkParseError::WrongFormat());
        }

        let map = lines
            .map(|x| parse_node(x).ok_or(NetworkParseError::WrongNone()))
            .collect::<Result<_, _>>()?;

        let network = Network { path, map };
        Ok(network)
    }
}
