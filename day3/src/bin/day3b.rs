fn main() {
  let original = global::read_lines();
  let mut group = vec![];
  let mut lines = vec![];

  for line in original {
    let sides: Vec<u32> = line.split(' ').filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect();
    let (a, b, c) = (sides[0], sides[1], sides[2]);
    group.push((a, b, c));

    if group.len() == 3 {
      lines.push(format!{"{} {} {}", group[0].0 , group[1].0, group[2].0 });
      lines.push(format!{"{} {} {}", group[0].1 , group[1].1, group[2].1 });
      lines.push(format!{"{} {} {}", group[0].2 , group[1].2, group[2].2 });
      group.clear();
    }
  }

  let mut count = 0;

  for line in lines {
    let sides: Vec<u32> = line.split(' ').filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect();
    let (a, b, c) = (sides[0], sides[1], sides[2]);
    if a + b > c && a + c > b && b + c > a { count += 1 ; }
  }
  
  println!("{}", count);
}