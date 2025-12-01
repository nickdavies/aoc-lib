use crate::parse::Parser;
use crate::parse::Sections;

use std::str::FromStr;

pub struct ParseFromStr;

impl<E, T: FromStr<Err = E>> Parser<&str, T, E> for ParseFromStr {
    fn parse_section(&self, section: &str) -> Result<T, E> {
        section.parse()
    }
}

pub fn parse_input<'a, SPLIT, PARSE, E, S, FT>(
    make_sections: SPLIT,
    section_parser: PARSE,
    input: &'a str,
) -> Result<Vec<FT>, E>
where
    SPLIT: Sections<'a, S>,
    PARSE: Parser<S, FT, E>,
{
    let mut out = Vec::new();
    for section in make_sections.to_sections(input) {
        out.push(section_parser.parse_section(section)?);
    }
    Ok(out)
}

#[derive(Debug, thiserror::Error)]
pub enum MidSplitError<E0, E1> {
    #[error("Failed to parse left value: {0}")]
    ParseError0(E0),
    #[error("Failed to parse right value: {0}")]
    ParseError1(E1),
    #[error("Input string was not even length, can't split equally. Was {0}")]
    NotEven(usize),
    #[error("The mid point was an invalid unicode split")]
    InvalidMiddle,
}

pub struct SplitMiddle<P0, P1>(pub P0, pub P1);

impl<'a, E0, E1, P0, P1, T0, T1> Parser<&'a str, (T0, T1), MidSplitError<E0, E1>>
    for SplitMiddle<P0, P1>
where
    P0: Parser<&'a str, T0, E0>,
    P1: Parser<&'a str, T1, E1>,
{
    fn parse_section(&self, section: &'a str) -> Result<(T0, T1), MidSplitError<E0, E1>> {
        if !section.len().is_multiple_of(2) {
            return Err(MidSplitError::NotEven(section.len()));
        }
        let (first, last) = section
            .split_at_checked(section.len() / 2)
            .ok_or(MidSplitError::InvalidMiddle)?;

        Ok((
            self.0
                .parse_section(first)
                .map_err(MidSplitError::ParseError0)?,
            self.1
                .parse_section(last)
                .map_err(MidSplitError::ParseError1)?,
        ))
    }
}

pub struct SplitDelim<'a, T>(pub T, pub &'a str);

impl<'a, E, P, T> Parser<&'a str, Vec<T>, E> for SplitDelim<'_, P>
where
    P: Parser<&'a str, T, E>,
{
    fn parse_section(&self, section: &'a str) -> Result<Vec<T>, E> {
        let parts = section.split(self.1);
        let mut out = Vec::new();
        for item in parts {
            out.push(self.0.parse_section(item)?);
        }
        Ok(out)
    }
}

pub struct Trim<T>(pub T);
impl<'a, T, P, E> Parser<&'a str, T, E> for Trim<P>
where
    P: Parser<&'a str, T, E>,
{
    fn parse_section(&self, section: &'a str) -> Result<T, E> {
        self.0.parse_section(section.trim())
    }
}

pub struct Chars<T>(pub T);
impl<'a, T, P, E> Parser<&'a str, Vec<T>, E> for Chars<P>
where
    P: Parser<char, T, E>,
{
    fn parse_section(&self, section: &'a str) -> Result<Vec<T>, E> {
        let mut out = Vec::with_capacity(section.len());
        for c in section.chars() {
            out.push(self.0.parse_section(c)?);
        }
        Ok(out)
    }
}
