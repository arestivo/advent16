use day1::{Direction, Santa, Location};

fn main() {
  let line = global::read_line();
  let steps = day1::line_to_steps(&line);

  let mut santa = Santa { location: Location { row: 0, col: 0 }, direction: Direction::North }; 

  for s in steps { santa.step(&s) }

  println!("{}", santa.location.row.abs() + santa.location.col.abs());
}
