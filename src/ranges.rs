#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct InclusiveRange<A, B = A> {
    pub start: A,
    pub end: B,
}

impl<A, B> InclusiveRange<A, B> {
    pub fn includes<C: PartialOrd<A> + PartialOrd<B>>(&self, target: C) -> bool {
        target >= self.start && target <= self.end
    }
}

pub fn merge_inclusive_ranges<A: Ord + PartialOrd<B> + Ord + PartialOrd<A>, B: Ord>(
    mut ranges: Vec<InclusiveRange<A, B>>,
) -> Vec<InclusiveRange<A, B>> {
    ranges.sort();

    let mut out: Vec<InclusiveRange<A, B>> = Vec::new();
    let mut current: Option<(A, B)> = None;
    for range in ranges {
        current = if let Some((current_start, current_end)) = current {
            if range.start <= current_end {
                Some((current_start, std::cmp::max(current_end, range.end)))
            } else {
                out.push(InclusiveRange {
                    start: current_start,
                    end: current_end,
                });
                Some((range.start, range.end))
            }
        } else {
            Some((range.start, range.end))
        };
    }

    if let Some((current_start, current_end)) = current {
        out.push(InclusiveRange {
            start: current_start,
            end: current_end,
        });
    }

    out
}
