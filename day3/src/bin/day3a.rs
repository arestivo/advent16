fn main() {
  let lines = global::read_lines();

  let mut count = 0;

  for line in lines {
    let sides: Vec<u32> = line.split(' ').filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect();
    let (a, b, c) = (sides[0], sides[1], sides[2]);

    if a + b > c && a + c > b && b + c > a { count += 1 ; }
  }
  
  println!("{}", count);
}