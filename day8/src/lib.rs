use std::fmt;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Debug)]
pub enum Instruction {
  #[display("rect {width}x{height}")]
  Rect {width: usize, height: usize},
  #[display("rotate row y={row} by {pixels}")]
  RotateRow {row: usize, pixels: usize},
  #[display("rotate column x={col} by {pixels}")]
  RotateCol {col: usize, pixels: usize}
}

#[derive(Debug)]
pub struct Screen {
  pub pixels: Vec<Vec<bool>>
}

impl Screen {
  pub fn new(width: usize, height: usize) -> Screen {
    let mut pixels = vec![];

    for _ in 0..height {
      let mut row = vec![];
      for _ in 0..width { row.push(false) }
      pixels.push(row);
    }

    Screen { pixels }
  }

  pub fn execute(&mut self, instruction: &Instruction) {
    match instruction {
        Instruction::Rect { width, height } => {
          for r in 0..*height {
            for c in 0..*width {
              self.pixels[r][c] = true;
            }
          }
        },
        Instruction::RotateRow { row, pixels } => {
          let mut to_append = vec![];
          let width = self.pixels[*row].len();
          for col in width-pixels..width {
            to_append.push(self.pixels[*row][col]);
          }

          for col in (0..width-*pixels).rev() {
            self.pixels[*row][col + pixels] = self.pixels[*row][col];
          }
          for (col, p) in to_append.iter().enumerate() {
            self.pixels[*row][col] = *p;
          }
        },
        Instruction::RotateCol { col, pixels } => {
          let mut to_append = vec![];
          let height = self.pixels.len();
          for row in height-pixels..height {
            to_append.push(self.pixels[row][*col]);
          }

          for row in (0..height-*pixels).rev() {
            self.pixels[row + pixels][*col] = self.pixels[row][*col];
          }
          for (row, p) in to_append.iter().enumerate() {
            self.pixels[row][*col] = *p;
          }
        },
    }
  }
}

impl fmt::Display for Screen {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for row in &self.pixels {
      for pixel in row {
        write!(f, "{}", if *pixel {'#'} else {'.'})?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}


pub fn lines_to_instructions(lines: &[String]) -> Vec<Instruction> {
  lines
    .iter()
    .map(|l| l.parse::<Instruction>())
    .map(Result::unwrap)
    .collect()
}