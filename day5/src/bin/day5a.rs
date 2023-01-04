fn main() {
  let line = global::read_line();

  let mut i = 0;
  let mut pwd = "".to_string();

  loop {
    let s = format!("{}{}", line, i);
    let hash = format!("{:?}", md5::compute(s));
    if hash.starts_with("00000") {
      pwd.push(hash.chars().nth(5).unwrap());
      if pwd.len() == 8 { break; }
    }

    i += 1;
  }

  println!("{}", pwd);
}