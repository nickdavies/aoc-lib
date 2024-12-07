use crate::parse::Sections;
use std::iter::Peekable;

pub struct LineSplitter;
impl<'a> Sections<'a, &'a str> for LineSplitter {
    fn to_sections(&self, input: &'a str) -> impl Iterator<Item = &'a str> {
        input.lines()
    }
}

pub struct LineBlocksIterator<'a> {
    lines: Peekable<std::str::Lines<'a>>,
    delim_fn: fn(&'a str) -> bool,
}

impl<'a> Iterator for LineBlocksIterator<'a> {
    type Item = std::vec::IntoIter<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.peek()?;
        let next: Vec<&str> = (&mut self.lines)
            .take_while(|line| !(self.delim_fn)(line))
            .collect();

        Some(next.into_iter())
    }
}

pub struct LineGroupSplitter(fn(&str) -> bool);
impl LineGroupSplitter {
    pub fn new(delim_fn: fn(&str) -> bool) -> Self {
        Self(delim_fn)
    }

    pub fn blankline() -> Self {
        LineGroupSplitter::new(|l| l.trim().is_empty())
    }
}

impl<'a> Sections<'a, std::vec::IntoIter<&'a str>> for LineGroupSplitter {
    fn to_sections(&self, input: &'a str) -> impl Iterator<Item = std::vec::IntoIter<&'a str>> {
        LineBlocksIterator {
            lines: input.lines().peekable(),
            delim_fn: self.0,
        }
    }
}
