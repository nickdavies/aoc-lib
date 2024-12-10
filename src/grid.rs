#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn all() -> &'static [Direction; 4] {
        &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    pub fn invert(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    pub fn idx(&self) -> usize {
        match self {
            Self::North => 0,
            Self::East => 1,
            Self::South => 2,
            Self::West => 3,
        }
    }

    pub fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Self::North => 0,
            Self::East => 1,
            Self::South => 2,
            Self::West => 3,
        }
    }

    pub fn from_index(idx: usize) -> Option<Self> {
        Some(match idx {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd, Hash)]
pub struct UnboundLocation(pub i64, pub i64);

impl UnboundLocation {
    pub fn go_direction(&self, direction: &Direction, distance: usize) -> UnboundLocation {
        match direction {
            Direction::North => UnboundLocation(self.0 - distance as i64, self.1),
            Direction::East => UnboundLocation(self.0, self.1 + distance as i64),
            Direction::South => UnboundLocation(self.0 + distance as i64, self.1),
            Direction::West => UnboundLocation(self.0, self.1 - distance as i64),
        }
    }

    pub fn to_bounded<T>(self, map: &Map<T>) -> Option<Location> {
        map.get_location(self.0.try_into().ok()?, self.1.try_into().ok()?)
    }
}

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd, Hash)]
pub struct Location(pub usize, pub usize);

impl Location {
    pub fn manhattan_dist(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

pub type Grid<T> = Vec<Vec<T>>;

#[derive(Debug)]
pub struct Map<T>(pub Grid<T>);

impl<T> Map<T> {
    pub fn parse<E>(input: &str, parse_char: fn(char) -> Result<T, E>) -> Result<Map<T>, E> {
        let mut out = Vec::new();
        for line in input.lines() {
            let mut out_line = Vec::new();
            for char in line.chars() {
                out_line.push(parse_char(char)?);
            }
            out.push(out_line);
        }
        Ok(Map(out))
    }
    pub fn get(&self, location: &Location) -> &T {
        &self.0[location.0][location.1]
    }

    pub fn get_mut(&mut self, location: &Location) -> &mut T {
        &mut self.0[location.0][location.1]
    }

    pub fn get_location(&self, x: usize, y: usize) -> Option<Location> {
        self.0
            .get(x)
            .and_then(|row| row.get(y))
            .map(|_| Location(x, y))
    }

    pub fn go_direction(&self, current: &Location, direction: &Direction) -> Option<Location> {
        match direction {
            Direction::North => {
                if current.0 != 0 {
                    Some(Location(current.0 - 1, current.1))
                } else {
                    None
                }
            }
            Direction::East => self.get_location(current.0, current.1 + 1),
            Direction::South => self.get_location(current.0 + 1, current.1),
            Direction::West => {
                if current.1 != 0 {
                    Some(Location(current.0, current.1 - 1))
                } else {
                    None
                }
            }
        }
    }

    pub fn iter_direction(
        &self,
        start: Location,
        x_direction: Option<Direction>,
        y_direction: Option<Direction>,
    ) -> DirectionIterator<T> {
        DirectionIterator {
            map: self,
            current: start,
            x_direction,
            y_direction,
        }
    }

    pub fn get_edges(&self) -> Vec<(Location, Direction)> {
        let mut out = Vec::new();

        for col in 0..self.0[0].len() {
            out.push((Location(0, col), Direction::South));
            out.push((Location(self.0.len() - 1, col), Direction::North));
        }
        for (row_num, row) in self.0.iter().enumerate() {
            out.push((Location(row_num, 0), Direction::East));
            out.push((Location(row_num, row.len() - 1), Direction::West));
        }

        out
    }

    pub fn bottom_right(&self) -> Option<Location> {
        let row = self.0.last()?;
        Some(Location(self.0.len() - 1, row.len() - 1))
    }

    pub fn iter(&self) -> MapIterator<'_, T> {
        MapIterator {
            row_num: 0,
            map: self,
        }
    }

    pub fn print<F>(&self, to_char: F)
    where
        F: Fn(&T, Location) -> char,
    {
        for row in self.iter() {
            for (loc, col) in row {
                let out_char = to_char(col, loc);
                print!("{}", out_char);
            }
            println!();
        }
    }

    pub fn transform<N, F>(&self, transform_single: F) -> Map<N>
    where
        F: Fn(Location, &T) -> N,
    {
        let mut out = Vec::new();
        for row in self.iter() {
            let mut out_row = Vec::new();
            for (loc, col) in row {
                out_row.push(transform_single(loc, col));
            }
            out.push(out_row);
        }
        Map(out)
    }
}

pub struct MapIterator<'a, T> {
    row_num: usize,
    map: &'a Map<T>,
}

impl<'a, T> Iterator for MapIterator<'a, T> {
    type Item = RowIterator<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_num >= self.map.0.len() {
            None
        } else {
            let row_iter = RowIterator {
                row_num: self.row_num,
                col_num: 0,
                row: &self.map.0[self.row_num],
            };
            self.row_num += 1;
            Some(row_iter)
        }
    }
}

pub struct RowIterator<'a, T> {
    row_num: usize,
    col_num: usize,
    row: &'a Vec<T>,
}

impl<T> RowIterator<'_, T> {
    #[inline(always)]
    pub fn row_num(&self) -> usize {
        self.row_num
    }
}

impl<'a, T> Iterator for RowIterator<'a, T> {
    type Item = (Location, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.col_num >= self.row.len() {
            None
        } else {
            self.col_num += 1;
            Some((
                Location(self.row_num, self.col_num - 1),
                &self.row[self.col_num - 1],
            ))
        }
    }
}

pub struct DirectionIterator<'a, T> {
    current: Location,
    x_direction: Option<Direction>,
    y_direction: Option<Direction>,
    map: &'a Map<T>,
}

impl<'a, T> Iterator for DirectionIterator<'a, T> {
    type Item = (Location, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x_direction.is_none() && self.y_direction.is_none() {
            return None;
        }
        let mut new_location = self.current.clone();
        if let Some(direction) = &self.x_direction {
            new_location = self.map.go_direction(&new_location, direction)?;
        }
        if let Some(direction) = &self.y_direction {
            new_location = self.map.go_direction(&new_location, direction)?;
        }
        self.current = new_location.clone();
        Some((new_location.clone(), self.map.get(&new_location)))
    }
}

pub struct CountingMap(Map<bool>, usize);

impl CountingMap {
    pub fn mark(&mut self, l: &Location) {
        let v = self.0.get_mut(l);
        if !*v {
            self.1 += 1;
        }
        *v = true;
    }
    pub fn unique(&self) -> usize {
        self.1
    }
}

impl<T> From<&Map<T>> for CountingMap {
    fn from(other: &Map<T>) -> Self {
        Self(other.transform(|_, _| false), 0)
    }
}

impl<T, E> TryFrom<&str> for Map<T>
where
    char: TryInto<T, Error = E>,
{
    type Error = E;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::parse(input, |c| c.try_into())
    }
}
