use crate::parse::Parser;

pub mod chars;
pub mod tuples;

pub struct Identity;
impl<T> Parser<T, T, std::convert::Infallible> for Identity {
    fn parse_section(&self, section: T) -> Result<T, std::convert::Infallible> {
        Ok(section)
    }
}

pub struct ParseVec<T>(pub T);

impl<E, S, I, INNER, T> Parser<I, Vec<T>, E> for ParseVec<INNER>
where
    I: Iterator<Item = S>,
    INNER: Parser<S, T, E>,
{
    fn parse_section(&self, section: I) -> Result<Vec<T>, E> {
        let mut out = Vec::new();
        for item in section {
            out.push(self.0.parse_section(item)?);
        }
        Ok(out)
    }
}
