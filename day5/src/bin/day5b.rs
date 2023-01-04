fn main() {
  let line = global::read_line();

  let mut i = 0;
  let mut pwd = "________".to_string();

  loop {
    let s = format!("{}{}", line, i);
    let hash = format!("{:?}", md5::compute(s));
    if hash.starts_with("00000") {
      let pos = hash.chars().nth(5).unwrap();
      if pos.is_numeric() {
        let pos = pos.to_string().parse::<usize>().unwrap();
        if pos < 8 && pwd.chars().nth(pos).unwrap() == '_' {
          let c = hash.chars().nth(6).unwrap().to_string();
          pwd.replace_range(pos..pos+1, c.as_str());
        }
      }
      if !pwd.contains('_') { break; }
    }

    i += 1;
  }

  println!("{}", pwd);
}