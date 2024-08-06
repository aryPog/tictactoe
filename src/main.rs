use colored::Colorize;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const CROSS_SYMBOL: char = 'X';
const NOUGHT_SYMBOL: char = '0';

const USER_MARK: i8 = 1;
const AI_MARK: i8 = -1;

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
    if board.board[y as usize][x as usize] != 0 {
      panic!("Invalid play, already taken spot");
    }
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

  fn is_winner(&self, board: &Board) -> bool {
    let board_size = board.size();

    let mark = match self.symbol {
      CROSS_SYMBOL => USER_MARK,
      NOUGHT_SYMBOL => AI_MARK,
      _ => panic!("Invalid player symbol"),
    };

    for row in 0..board_size {
      if board.board[row as usize].iter().all(|&cell| cell == mark) {
        return true;
      }
    }

    for col in 0..board_size {
      if (0..board_size).all(|row| board.board[row as usize][col as usize] == mark) {
        return true;
      }
    }

    if (0..board_size).all(|i| board.board[i as usize][i as usize] == mark) {
      return true;
    }

    if (0..board_size).all(|i| board.board[i as usize][(board_size - 1 - i) as usize] == mark) {
      return true;
    }

    false
  }
}

// AI PLAYER
#[derive(Debug)]
struct AI {
  ai: Player,
}

impl AI {
  fn new(symbol: char) -> Self {
    AI {
      ai: Player::new(symbol),
    }
  }

  fn play(&mut self, board: &mut Board, player: &Player) {
    let mut best_move = (0_u8, 0_u8);
    let mut best_value = -100;

    let x_length = board.size();
    let y_length = board.size();

    for i in 0..x_length {
      for j in 0..y_length {
        if board.board[i as usize][j as usize] == 0 {
          board.board[i as usize][j as usize] = AI_MARK;
          let value = board.minimax(false, 0, player, &self.ai);
          board.board[i as usize][j as usize] = 0;
          if value > best_value {
            best_value = value;
            best_move = (j, i);
          }
        }
      }
    }

    self.ai.play(board, best_move.0, best_move.1);
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
      CROSS_SYMBOL => USER_MARK,
      NOUGHT_SYMBOL => AI_MARK,
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
        if self.board[y as usize][x as usize] == 0 {
          return false;
        }
      }
    }

    true
  }

  fn minimax(&mut self, is_maximizing: bool, depth: isize, player: &Player, ai: &Player) -> isize {
    if player.is_winner(self) {
      return -100 + depth;
    } else if ai.is_winner(self) {
      return 100 - depth;
    } else if self.terminal() {
      return 0;
    }

    let x_length = self.size();
    let y_length = self.size();

    if is_maximizing {
      let mut best_score = -100;
      for i in 0..x_length {
        for j in 0..y_length {
          if self.board[i as usize][j as usize] == 0 {
            self.board[i as usize][j as usize] = AI_MARK;
            let score = self.minimax(false, depth + 1, player, ai);
            self.board[i as usize][j as usize] = 0;
            if score > best_score {
              best_score = score;
            }
          }
        }
      }
      best_score
    } else {
      let mut best_score = 100;
      for i in 0..x_length {
        for j in 0..y_length {
          if self.board[i as usize][j as usize] == 0 {
            self.board[i as usize][j as usize] = USER_MARK;
            let score = self.minimax(true, depth + 1, player, ai);
            self.board[i as usize][j as usize] = 0;
            if score < best_score {
              best_score = score;
            }
          }
        }
      }
      best_score
    }
  }
}

// MAIN
fn main() -> io::Result<()> {
  let board_path = Path::new("src/board");
  let mut board = Board::new(board_path);

  let mut current_player;

  clear_terminal();
  let mut input = String::new();
  println!("First to Move: (p)layer (c)omputer");
  io::stdin()
    .read_line(&mut input)
    .expect("Error getting user input");
  let input = input.trim();
  match input {
    "p" => current_player = USER_MARK,
    "c" => current_player = AI_MARK,
    _ => panic!("Invalid first to move"),
  }

  let mut cross = Player::new(CROSS_SYMBOL);
  let mut nought = AI::new(NOUGHT_SYMBOL);

  clear_terminal();
  let mut round_count = 0;
  while !board.terminal() && !cross.is_winner(&board) && !nought.ai.is_winner(&board) {
    board.draw();

    if current_player == USER_MARK {
      if !board.terminal() && !cross.is_winner(&board) && !nought.ai.is_winner(&board) {
        let mut input = String::new();
        io::stdin()
          .read_line(&mut input)
          .expect("Error getting user input");
        let input = input.trim();
        let (x, y) = convert_input(input);
        if y >= board.size() || x >= board.size() {
          panic!("x or y out of bounds");
        }

        cross.play(&mut board, x, y);
        round_count += 1;
      }
      current_player = AI_MARK;
    } else if current_player == AI_MARK {
      if !board.terminal() && !cross.is_winner(&board) && !nought.ai.is_winner(&board) {
        nought.play(&mut board, &cross);
        round_count += 1;
      }
      current_player = USER_MARK;
    }
    clear_terminal();
  }
  board.draw();
  println!("GAME ENDED IN {}", round_count);

  Ok(())
}
