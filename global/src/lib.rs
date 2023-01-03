use std::{io, io::prelude::*};

pub fn read_lines() -> Vec<String> {
  io::stdin().lock().lines().map(|x| x.unwrap()).collect()
}


pub fn read_line() -> String {
  io::stdin().lock().lines().next().unwrap().unwrap()
}
