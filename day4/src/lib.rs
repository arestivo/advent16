use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
pub struct Room {
  pub name: Vec<String>,
  pub id: u32,
  pub checksum: String
}

impl Room {
  pub fn is_real(&self) -> bool {
    let mut letters = HashMap::new();

    for part in self.name.clone() {
      for c in part.chars() {
        if let Some(n) = letters.get(&c) { letters.insert(c, n + 1); }
        else { letters.insert(c, 1_u32); }
      }
    }

    let mut tuples: Vec<(char, u32)> = letters.into_iter().collect();
    tuples.sort_unstable_by_key(|t| (u32::MAX - t.1, t.0));

    let selected = tuples[0..5].to_vec().iter().map(|t| t.0.to_string()).collect::<Vec<String>>().join("");

    selected == self.checksum
  }

  pub fn decrypted(&self) -> String {
    let mut name = "".to_string();

    for part in &self.name {
      for c in part.chars() {
        name.push((((c as u32 - 'a' as u32 + self.id) % ('z' as u32 - 'a' as u32 + 1)) as u8 + b'a') as char);
      }
      name.push(' ');
    }
    
    name.trim().to_string()
  }
}

pub fn lines_to_rooms(lines: &[String]) -> Vec<Room> {
  let re = Regex::new(r"([a-z-]+)(\d+)\[([a-z]+)\]").unwrap();
  let mut rooms = vec![];

  for line in lines {
    let cap = re.captures_iter(line).next().unwrap();
    let name = cap[1][0..cap[1].len()-1] .split('-').map(|s| s.to_string()).collect();
    let id = cap[2].parse().unwrap();
    let checksum = cap[3].to_string();

    rooms.push(Room { name, id, checksum })
  };

  rooms
}