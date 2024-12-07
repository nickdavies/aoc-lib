use crate::parse::Parser;
#[derive(Debug, thiserror::Error)]

pub enum MidSplitError<E> {
    #[error("Failed to parse tuple element: {0}")]
    ParseError(E),
    #[error("Input string was not even length, can't split equally. Was {0}")]
    NotEven(usize),
    #[error("The mid point was an invalid unicode split")]
    InvalidMiddle,
}

pub struct SplitMiddle<P0, P1>(pub P0, pub P1);

impl<'a, E, P0, P1, T0, T1> Parser<&'a str, (T0, T1), MidSplitError<E>> for SplitMiddle<P0, P1>
where
    P0: Parser<&'a str, T0, E>,
    P1: Parser<&'a str, T1, E>,
{
    fn parse_section(&self, section: &'a str) -> Result<(T0, T1), MidSplitError<E>> {
        if section.len() % 2 != 0 {
            return Err(MidSplitError::NotEven(section.len()));
        }
        let (first, last) = section
            .split_at_checked(section.len() / 2)
            .ok_or(MidSplitError::InvalidMiddle)?;

        Ok((
            self.0
                .parse_section(first)
                .map_err(MidSplitError::ParseError)?,
            self.1
                .parse_section(last)
                .map_err(MidSplitError::ParseError)?,
        ))
    }
}
