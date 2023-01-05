fn main() {
  let encoded = global::read_line();
  let decoded = decode(&encoded);
  println!("{}", decoded.len());
}

pub fn decode(encoded: &String) -> String {
  let mut ptr = 0;
  let mut inside_parens = false;
  let mut marker = "".to_string();
  let mut decoded = "".to_string();

  while ptr < encoded.len() {
    let c = encoded.chars().nth(ptr).unwrap();
    if inside_parens {
      if c == ')' {
        let parts: Vec<&str> = marker.split('x').collect();
        let size = parts[0].parse::<usize>().unwrap();
        let times = parts[1].parse::<u32>().unwrap();

        for _ in 0..times {
          decoded.push_str(&encoded[ptr + 1..ptr + 1 + size]);
        }

        inside_parens = false;
        marker = "".to_string();
        ptr += size;
      } else {
        marker.push(c);
      }
      ptr += 1
    } else {      
      if c == '(' {
        inside_parens = true;
      } else {
        decoded.push(c);
      }
      ptr += 1;
    }
  }

  decoded
}