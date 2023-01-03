use std::collections::HashSet;

use day1::{Direction, Santa, Location, Step};

fn main() {
  let line = global::read_line();
  let steps = day1::line_to_steps(&line);

  println!("{}", closest_crossing(&steps))
}

fn closest_crossing(steps: &[Step]) -> i32 {
  let mut santa = Santa { location: Location { row: 0, col: 0 }, direction: Direction::North }; 
  let mut visited = HashSet::new();

  visited.insert(santa.location.clone());

  for s in steps {
    let locations = santa.step_locations(&s);

    for location in locations {
      if visited.contains(&location) {
        return location.row.abs() + location.col.abs();
      }
      visited.insert(location.clone());
    }
  };

  unreachable!()
}