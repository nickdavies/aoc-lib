use crate::parse::Parser;

#[derive(Debug, thiserror::Error)]
pub enum TupleError<E0, E1, E2, T> {
    #[error("Failed to parse tuple element 0: {0}")]
    ParseError0(E0),
    #[error("Failed to parse tuple element 1: {0}")]
    ParseError1(E1),
    #[error("Failed to parse tuple element 2: {0}")]
    ParseError2(E2),
    #[error("Not enough elements found: {0}")]
    Missing(usize),
    #[error("Additional values remaining. Next: {0:?}")]
    Extra(T),
}

pub struct ParseTuple2<'a, P0, P1>(pub P0, pub P1, pub &'a str);

impl<'a, E0, E1, P0, P1, T0, T1>
    Parser<&'a str, (T0, T1), TupleError<E0, E1, std::convert::Infallible, String>>
    for ParseTuple2<'a, P0, P1>
where
    P0: Parser<&'a str, T0, E0>,
    P1: Parser<&'a str, T1, E1>,
{
    fn parse_section(
        &self,
        section: &'a str,
    ) -> Result<(T0, T1), TupleError<E0, E1, std::convert::Infallible, String>> {
        let mut parts = section.split(self.2);
        let out = (
            self.0
                .parse_section(parts.next().ok_or_else(|| TupleError::Missing(0))?)
                .map_err(TupleError::ParseError0)?,
            self.1
                .parse_section(parts.next().ok_or_else(|| TupleError::Missing(1))?)
                .map_err(TupleError::ParseError1)?,
        );
        let next = parts.next();
        if let Some(extra) = next {
            Err(TupleError::Extra(extra.to_string()))
        } else {
            Ok(out)
        }
    }
}

pub struct ParseTuple3<'a, P0, P1, P2>(pub P0, pub P1, pub P2, pub &'a str);

impl<'a, E0, E1, E2, P0, P1, P2, T0, T1, T2>
    Parser<&'a str, (T0, T1, T2), TupleError<E0, E1, E2, String>> for ParseTuple3<'a, P0, P1, P2>
where
    P0: Parser<&'a str, T0, E0>,
    P1: Parser<&'a str, T1, E1>,
    P2: Parser<&'a str, T2, E2>,
{
    fn parse_section(
        &self,
        section: &'a str,
    ) -> Result<(T0, T1, T2), TupleError<E0, E1, E2, String>> {
        let mut parts = section.split(self.3);
        let out = (
            self.0
                .parse_section(parts.next().ok_or_else(|| TupleError::Missing(0))?)
                .map_err(TupleError::ParseError0)?,
            self.1
                .parse_section(parts.next().ok_or_else(|| TupleError::Missing(1))?)
                .map_err(TupleError::ParseError1)?,
            self.2
                .parse_section(parts.next().ok_or_else(|| TupleError::Missing(2))?)
                .map_err(TupleError::ParseError2)?,
        );
        let next = parts.next();
        if let Some(extra) = next {
            Err(TupleError::Extra(extra.to_string()))
        } else {
            Ok(out)
        }
    }
}

pub struct ParseSectionTuple2<P0, P1>(pub P0, pub P1);

impl<'a, E0, E1, P0, P1, T0, T1>
    Parser<
        std::vec::IntoIter<&'a str>,
        (T0, T1),
        TupleError<E0, E1, std::convert::Infallible, String>,
    > for ParseSectionTuple2<P0, P1>
where
    P0: Parser<&'a str, T0, E0>,
    P1: Parser<&'a str, T1, E1>,
{
    fn parse_section(
        &self,
        mut section: std::vec::IntoIter<&'a str>,
    ) -> Result<(T0, T1), TupleError<E0, E1, std::convert::Infallible, String>> {
        let out = (
            self.0
                .parse_section(section.next().ok_or_else(|| TupleError::Missing(0))?)
                .map_err(TupleError::ParseError0)?,
            self.1
                .parse_section(section.next().ok_or_else(|| TupleError::Missing(1))?)
                .map_err(TupleError::ParseError1)?,
        );
        let next = section.next();
        if let Some(extra) = next {
            Err(TupleError::Extra(extra.to_string()))
        } else {
            Ok(out)
        }
    }
}
