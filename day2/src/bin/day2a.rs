fn main() {
  let lines = global::read_lines();

  let mut current = 5;

  for line in lines {
    for m in line.chars() {
      current = match m {
        'U' => { if current < 4 { current } else { current - 3} },
        'R' => { if current % 3 == 0 { current } else { current + 1 } },
        'D' => { if current > 6 { current } else { current + 3} },
        'L' => { if (current + 2) % 3 == 0 { current } else { current - 1} },
        _ => unreachable!()
      };
    }  
    print!("{}", current);
  }

  println!();
}