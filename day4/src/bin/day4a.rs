use day4::lines_to_rooms;

fn main() {
  let lines = global::read_lines();
  let rooms = lines_to_rooms(&lines);

  let sum: u32 = rooms.into_iter().filter(|r| r.is_real()).map(|r| r.id).sum();

  println!("{:?}", sum);
}