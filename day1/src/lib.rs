#[derive(Debug)]
pub enum Step {
  Right(i32),
  Left(i32),
}

#[derive(Debug)]
pub enum Direction {
  North,
  East,
  South,
  West,
}

#[derive(Debug)]
pub struct Santa {
  pub location: Location,
  pub direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location {
  pub row: i32,
  pub col: i32,
}

impl Direction {
  fn left(&self) -> Direction {
    match self {
        Direction::North => Self::West,
        Direction::East => Self::North,
        Direction::South => Self::East,
        Direction::West => Self::South,
    }
  }

  fn right(&self) -> Direction {
    match self {
        Direction::North => Self::East,
        Direction::East => Self::South,
        Direction::South => Self::West,
        Direction::West => Self::North,
    }
  }
}

impl Step {
  pub fn distance(&self) -> i32 {
    match self {
        Step::Right(d) => *d,
        Step::Left(d) => *d,
    }
  }
}

impl Santa {
  pub fn step(&mut self, s: &Step) {
    self.direction = match s {
      Step::Left(_) => self.direction.left(),
      Step::Right(_) => self.direction.right(),
    };

    self.walk(s.distance());
  }

  pub fn step_locations(&mut self, s: &Step) -> Vec<Location> {
    let mut locations = vec![];

    self.direction = match s {
      Step::Left(_) => self.direction.left(),
      Step::Right(_) => self.direction.right(),
    };

    for _ in 0..s.distance() {
      self.walk(1);
      locations.push(self.location.clone());
    };

    locations
  }

  pub fn walk(&mut self, d: i32) {
    match self.direction {
        Direction::North => self.location.row -= d,
        Direction::East => self.location.col += d,
        Direction::South => self.location.row += d,
        Direction::West => self.location.col -= d,
    }
  }
}

pub fn line_to_steps(line: &str) -> Vec<Step> {
  line
    .split(", ")
    .map(|s| match s.chars().next() {
      Some('R') => Step::Right(s[1..s.len()].parse().unwrap()),
      Some('L') => Step::Left(s[1..s.len()].parse().unwrap()),
      _ => unreachable!()
    })
    .collect()
}