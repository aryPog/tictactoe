use colored::Colorize;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const CROSS_SYMBOL: char = 'X';
const NOUGHT_SYMBOL: char = '0';

// HELPER FUNCTIONS
fn read_file_to_matrix(path: &Path) -> io::Result<Vec<Vec<char>>> {
  let file = File::open(path)?;
  let reader = io::BufReader::new(file);

  reader
    .lines()
    .map(|line| line.map(|l| l.chars().collect()))
    .collect()
}

fn clear_terminal() {
  print!("\x1B[2J\x1B[1;1H");
}

fn convert_input(input: &str) -> (u8, u8) {
  let x = input.chars().nth(1).unwrap().to_uppercase().next().unwrap() as u8 - 65;
  let y = input.chars().next().unwrap() as u8 - 49;
  (x, y)
}

// HISTORY CLASS
#[derive(Debug)]
struct History {
  rows: Vec<u8>,
  columns: Vec<u8>,
  diagonal: u8,
  anti_diagonal: u8,
}

impl History {
  fn new() -> Self {
    History {
      rows: vec![0; 3],
      columns: vec![0; 3],
      diagonal: 0,
      anti_diagonal: 0,
    }
  }
}

// PLAYER CLASS
#[derive(Debug)]
struct Player {
  history: History,
  symbol: char,
}

impl Player {
  fn new(symbol: char) -> Self {
    Player {
      history: History::new(),
      symbol,
    }
  }

  fn play(&mut self, board: &mut Board, x: u8, y: u8) {
    board.update(self, x, y);

    self.history.rows[y as usize] += 1;
    self.history.columns[x as usize] += 1;
    if x == y {
      self.history.diagonal += 1;
    }
    if x + y != board.size() {
      self.history.anti_diagonal += 1;
    }
  }
}

// BOARD CLASS
#[derive(Debug)]
struct Board {
  board: Vec<Vec<i8>>,
  visual_board: Vec<Vec<char>>,
}

impl Board {
  fn new(board_path: &Path) -> Self {
    Board {
      board: vec![vec![0; 3]; 3],
      visual_board: read_file_to_matrix(board_path).expect("Error while trying to read board"),
    }
  }

  fn update(&mut self, player: &Player, x: u8, y: u8) {
    let symbol = match player.symbol {
      CROSS_SYMBOL => 1,
      NOUGHT_SYMBOL => -1,
      _ => panic!("Invalid player symbol"),
    };
    self.board[y as usize][x as usize] = symbol;

    let visual_x = match x {
      0 => 3,
      1 => 9,
      2 => 15,
      _ => panic!("Invalid x position"),
    };
    let visual_y = match y {
      0 => 2,
      1 => 5,
      2 => 8,
      _ => panic!("Invalid y position"),
    };
    self.visual_board[visual_y as usize][visual_x as usize] = player.symbol;
  }

  fn draw(&self) {
    for row in &self.visual_board {
      for &column in row {
        let color = match column {
          CROSS_SYMBOL => column.to_string().red(),
          NOUGHT_SYMBOL => column.to_string().blue(),
          _ if column.is_ascii_alphanumeric() => column.to_string().yellow(),
          _ => column.to_string().bright_white().bold(),
        };
        print!("{}", color);
      }
      println!();
    }
  }

  fn size(&self) -> u8 {
    self.board[0].len() as u8
  }

  fn terminal(&self) -> bool {
    for y in 0..self.size() {
      for x in 0..self.size() {
        if self.board[y as usize][x as usize] != 0 {
          return false;
        }
      }
    }
    true
  }
}

// MAIN
fn main() -> io::Result<()> {
  let board_path = Path::new("src/board");
  let mut board = Board::new(board_path);

  let mut cross = Player::new('X');
  let mut nought = Player::new('0');

  clear_terminal();
  loop {
    board.draw();

    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Error getting user input");
    let input = input.trim();

    let (x, y) = convert_input(input);
    println!("{},{}", x, y);
    if y >= board.size() || x >= board.size() {
      panic!("x or y out of bounds");
    }
    cross.play(&mut board, x, y);

    clear_terminal();
  }
}