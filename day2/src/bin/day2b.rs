fn main() {
  let lines = global::read_lines();

  let keyboard = vec![
    vec![' ', ' ', '1', ' ', ' '],
    vec![' ', '2', '3', '4', ' '],
    vec!['5', '6', '7', '8', '9'],
    vec![' ', 'A', 'B', 'C', ' '],
    vec![' ', ' ', 'D', ' ', ' '],
  ];

  let mut row = 2; let mut col = 0;

  for line in lines {
    for m in line.chars() {
      let next_row = match m {
        'U' => { if row == 0 { row } else { row - 1 } },
        'D' => { if row == keyboard.len() - 1 { row } else { row + 1 } },
        _ => { row }
      };

      let next_col = match m {
        'L' => { if col == 0 { col } else { col - 1 } },
        'R' => { if col == keyboard[0].len() - 1 { col } else { col + 1 } },
        _ => { col }
      };    
      
      (row, col) = if keyboard[next_row][next_col] == ' ' {(row, col)} else {(next_row, next_col)};
    }  
    print!("{}", keyboard[row][col]);
  }

  println!();
}