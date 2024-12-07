use std::str::FromStr;

pub mod parsers;
pub mod preamble;
pub mod sections;

pub trait Sections<'a, T> {
    fn to_sections(&self, input: &'a str) -> impl Iterator<Item = T>;
}

pub trait Parser<S, T, E> {
    fn parse_section(&self, section: S) -> Result<T, E>;
}

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
