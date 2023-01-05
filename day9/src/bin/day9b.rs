fn main() {
  let encoded = global::read_line();
  let decoded = decode(&encoded);
  println!("{}", decoded);
}

pub fn decode(encoded: &String) -> u64 {
  let mut ptr = 0;
  let mut inside_parens = false;
  let mut marker = "".to_string();
  let mut decoded = 0_u64;

  while ptr < encoded.len() {
    let c = encoded.chars().nth(ptr).unwrap();
    if inside_parens {
      if c == ')' {
        let parts: Vec<&str> = marker.split('x').collect();
        let size = parts[0].parse::<usize>().unwrap();
        let times = parts[1].parse::<u64>().unwrap();

        decoded += times * decode(&encoded[ptr + 1..ptr + 1 + size].to_string());

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
        decoded += 1;
      }
      ptr += 1;
    }
  }

  decoded
}