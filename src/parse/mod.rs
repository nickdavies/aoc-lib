pub mod parsers;
pub mod preamble;
pub mod sections;

pub trait Sections<'a, T> {
    fn to_sections(&self, input: &'a str) -> impl Iterator<Item = T>;
}

pub trait Parser<S, T, E> {
    fn parse_section(&self, section: S) -> Result<T, E>;
}
