fn main() {
  let lines = global::read_lines();

  let mut count = 0;

  for address in lines {
    let parts: Vec<String> = address.split(&['[', ']'][..]).map(|s| s.to_string()).collect();

    let mut supernets = vec![];
    let mut hypernets = vec![];

    for (i, p) in parts.iter().enumerate() {
      if i % 2 == 0 { supernets.push(p.clone()); }
      if i % 2 == 1 { hypernets.push(p.clone()); }
    }

    if supports_ssl(&supernets, &hypernets) { count += 1; }
  }
  
  println!("{}", count);
}

fn supports_ssl(supernets: &Vec<String>, hypernets: &Vec<String>) -> bool {
  for sn in supernets {
    for hn in hypernets {
      if contains_aba_bab(sn, hn) { return true }
    }
  }

  false
}

fn contains_aba_bab(sn: &str, hn: &str) -> bool {
  for i in 0..sn.len() - 2 {
    let l1 = sn.chars().nth(i).unwrap();
    let l2 = sn.chars().nth(i + 1).unwrap();
    let l3 = sn.chars().nth(i + 2).unwrap();

    if l1 == l3 && l1 != l2 && hn.contains(&format!("{}{}{}", l2, l1, l2)) { return true; }
  }

  false
}