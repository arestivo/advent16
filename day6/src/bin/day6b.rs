use std::collections::HashMap;

fn main() {
  let lines = global::read_lines();

  let mut counts: [HashMap<char, i32>; 8] = [();8].map(|_| HashMap::new());

  for line in &lines {
    for (i, c) in line.char_indices() {
      match counts[i].entry(c) {
          std::collections::hash_map::Entry::Occupied(mut e) => { e.insert(e.get() + 1); }
          std::collections::hash_map::Entry::Vacant(e) => { e.insert(1); }
      }
    }
  }

  let mut data = "".to_string();

  (0..lines[0].len()).for_each(|i| {
    let mut pairs: Vec<(char, i32)> = counts[i].iter().map(|(a, b)| (*a,*b)).collect();
    pairs.sort_unstable_by_key(|p| (p.1, p.0));
    data.push(pairs[0].0);    
  });

  println!("{}", data);
}