use crate::parse::Parser;

#[derive(Debug, thiserror::Error)]
pub enum TupleError<E, T> {
    #[error("Failed to parse tuple element: {0}")]
    ParseError(E),
    #[error("Not enough elements found: {0}")]
    Missing(usize),
    #[error("Additional values remaining. Next: {0:?}")]
    Extra(T),
}

pub struct ParseTuple2<'a, P0, P1>(pub P0, pub P1, pub &'a str);

impl<'a, E, P0, P1, T0, T1> Parser<&'a str, (T0, T1), TupleError<E, String>>
    for ParseTuple2<'a, P0, P1>
where
    P0: Parser<&'a str, T0, E>,
    P1: Parser<&'a str, T1, E>,
{
    fn parse_section(&self, section: &'a str) -> Result<(T0, T1), TupleError<E, String>> {
        let mut parts = section.split(self.2);
        let out = (
            self.0
                .parse_section(parts.next().ok_or_else(|| TupleError::Missing(0))?)
                .map_err(TupleError::ParseError)?,
            self.1
                .parse_section(parts.next().ok_or_else(|| TupleError::Missing(1))?)
                .map_err(TupleError::ParseError)?,
        );
        let next = parts.next();
        if let Some(extra) = next {
            Err(TupleError::Extra(extra.to_string()))
        } else {
            Ok(out)
        }
    }
}

pub struct ParseTuple3<'a, P0, P1, P2>(P0, P1, P2, &'a str);

impl<'a, E, P0, P1, P2, T0, T1, T2> Parser<&'a str, (T0, T1, T2), TupleError<E, &'a str>>
    for ParseTuple3<'a, P0, P1, P2>
where
    P0: Parser<&'a str, T0, E>,
    P1: Parser<&'a str, T1, E>,
    P2: Parser<&'a str, T2, E>,
{
    fn parse_section(&self, section: &'a str) -> Result<(T0, T1, T2), TupleError<E, &'a str>> {
        let mut parts = section.split(self.3);
        let out = (
            self.0
                .parse_section(parts.next().ok_or_else(|| TupleError::Missing(0))?)
                .map_err(TupleError::ParseError)?,
            self.1
                .parse_section(parts.next().ok_or_else(|| TupleError::Missing(1))?)
                .map_err(TupleError::ParseError)?,
            self.2
                .parse_section(parts.next().ok_or_else(|| TupleError::Missing(2))?)
                .map_err(TupleError::ParseError)?,
        );
        let next = parts.next();
        if let Some(extra) = next {
            Err(TupleError::Extra(extra))
        } else {
            Ok(out)
        }
    }
}
