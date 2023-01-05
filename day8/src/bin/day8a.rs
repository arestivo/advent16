use day8::{lines_to_instructions, Screen};

fn main() {
  let lines = global::read_lines();
  let instructions = lines_to_instructions(&lines);

  let mut screen = if instructions.len() == 4 { Screen::new(7, 3) } else { Screen::new(50, 6) };

  for instruction in instructions {
    screen.execute(&instruction);
  }

  let mut count = 0;

  for row in 0..screen.pixels.len() {
    for col in 0..screen.pixels[row].len() {
      if screen.pixels[row][col] { count += 1 }    
    }      
  }

  println!("{}", count);
}