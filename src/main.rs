use colored::Colorize;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Player {
    symbol: char,
    x: u8,
    y: u8,
}

impl Player {
    fn new(symbol: char) -> Self {
        Player {
            symbol,
            x: 0,
            y: 0,
        }
    }
}

fn read_file_to_matrix(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    reader.lines()
        .map(|line| line.map(|l| l.chars().collect()))
        .collect()
}

fn print_board(ch: char, cross: char, nought: char) {
    if ch.is_ascii_alphanumeric() && ch != cross && ch != nought {
        print!("{}", ch.to_string().yellow());
    } else if ch != cross && ch != nought {
        print!("{}", ch.to_string().bright_white().bold());
    }
}

fn print_players(ch: char, cross: &Player, nought: &Player) {
    if ch == cross.symbol {
        print!("{}", ch.to_string().red());
    } else if ch == nought.symbol {
        print!("{}", ch.to_string().blue());
    }
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn convert_input(input: &str) -> (u8, u8) {
    let x = input.chars().nth(1).unwrap().to_uppercase().next().unwrap() as u8 - 64;
    let y = input.chars().nth(0).unwrap() as u8 - 48;
    (x, y)
}

fn main() -> io::Result<()> {
    let board_path = "src/board";
    let mut board = read_file_to_matrix(board_path)?;

    let mut cross = Player::new('X');
    let mut nought = Player::new('0');

    clear_terminal();
    loop {
        for chrow in &board {
            for &ch in chrow {
                print_players(ch, &cross, &nought);
                print_board(ch, cross.symbol, nought.symbol);
            }
            println!();
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error getting user input");
        let input = input.trim();

        if !input.is_empty() {
            let (x, y) = convert_input(input);

            // Update cross position
            cross.x = match x {
                1 => 3,
                2 => 9,
                3 => 15,
                _ => panic!("Invalid x position"),
            };
            cross.y = match y {
                1 => 2,
                2 => 5,
                3 => 8,
                _ => panic!("Invalid y position"),
            };

            board[cross.y as usize][cross.x as usize] = cross.symbol;
        }

        clear_terminal();
    }

    Ok(())
}