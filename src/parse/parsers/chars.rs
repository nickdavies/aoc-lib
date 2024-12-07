use crate::parse::Parser;

pub struct TryFromChar;

impl<E, T: TryFrom<char, Error = E>> Parser<char, T, E> for TryFromChar {
    fn parse_section(&self, section: char) -> Result<T, E> {
        section.try_into()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CharError<E> {
    #[error("Expected single character found {0}")]
    InvalidLen(usize),
    #[error("Failed to parse character: {0}")]
    ParseError(E),
}

pub struct SingleChar<P>(pub P);
impl<'a, P, T, E> Parser<&'a str, T, CharError<E>> for SingleChar<P>
where
    P: Parser<char, T, E>,
{
    fn parse_section(&self, section: &'a str) -> Result<T, CharError<E>> {
        let mut chars = section.chars();
        let out = chars.next().ok_or(CharError::InvalidLen(0))?;
        let count = chars.count() + 1;
        if count == 1 {
            self.0.parse_section(out).map_err(CharError::ParseError)
        } else {
            Err(CharError::InvalidLen(count))
        }
    }
}
