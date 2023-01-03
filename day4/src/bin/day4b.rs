use day4::{lines_to_rooms, Room};

fn main() {
  let lines = global::read_lines();
  let rooms = lines_to_rooms(&lines);

  let rooms: Vec<Room> = rooms.into_iter().filter(|r| r.is_real()).collect();

  for room in rooms {
    if room.decrypted() == "northpole object storage" { println!("{}", room.id); }
  }

}