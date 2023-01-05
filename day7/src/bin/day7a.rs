use fancy_regex::Regex;

fn main() {
  let lines = global::read_lines();

  let hypernet_regex = Regex::new(r"\[.*?\]").unwrap();
  let abba_regex = Regex::new(r"(\w)(\w)\2\1").unwrap();

  let count = lines.into_iter().filter(|address| {
    let hypernets = hypernet_regex.captures_iter(address);

    for hn in hypernets {
      let hn = hn.unwrap().get(0).unwrap().as_str().to_string();
      if contains_abba(&hn, &abba_regex) { return false; }
    }

    let address = hypernet_regex.replace_all(address, " ").to_string();
    contains_abba(&address, &abba_regex)
  }).count();

  println!("{}", count);
}

fn contains_abba(s: &str, regex: &Regex) -> bool {
  for candidate in regex.captures_iter(s) {
    let candidate = candidate.unwrap();
    let l1 = candidate.get(1).unwrap().as_str().to_string();
    let l2 = candidate.get(2).unwrap().as_str().to_string();
    
    if  l1 != l2 { return true }
  }

  false
}