use std::io;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Player {
    X,
    O,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    X,
    O,
}

struct Board {
    cells: [[Cell; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [[Cell::Empty; 3]; 3],
        }
    }

    fn print(&self) {
        for row in &self.cells {
            for cell in row {
                match cell {
                    Cell::Empty => print!("[ ] "),
                    Cell::X => print!("[X] "),
                    Cell::O => print!("[O] "),
                }
            }
            println!();
        }
    }

    fn place_marker(&mut self, row: usize, col: usize, player: Player) -> bool {
        if row < 3 && col < 3 && self.cells[row][col] == Cell::Empty {
            self.cells[row][col] = match player {
                Player::X => Cell::X,
                Player::O => Cell::O,
            };
            true
        } else {
            false
        }
    }

    fn check_winner(&self) -> Option<Player> {
        for i in 0..3 {
            if self.cells[i][0] != Cell::Empty
                && self.cells[i][0] == self.cells[i][1]
                && self.cells[i][1] == self.cells[i][2]
            {
                return match self.cells[i][0] {
                    Cell::X => Some(Player::X),
                    Cell::O => Some(Player::O),
                    _ => None,
                };
            }

            if self.cells[0][i] != Cell::Empty
                && self.cells[0][i] == self.cells[1][i]
                && self.cells[1][i] == self.cells[2][i]
            {
                return match self.cells[0][i] {
                    Cell::X => Some(Player::X),
                    Cell::O => Some(Player::O),
                    _ => None,
                };
            }
        }

        if self.cells[0][0] != Cell::Empty
            && self.cells[0][0] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][2]
        {
            return match self.cells[0][0] {
                Cell::X => Some(Player::X),
                Cell::O => Some(Player::O),
                _ => None,
            };
        }

        if self.cells[0][2] != Cell::Empty
            && self.cells[0][2] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][0]
        {
            return match self.cells[0][2] {
                Cell::X => Some(Player::X),
                Cell::O => Some(Player::O),
                _ => None,
            };
        }

        None
    }

    fn is_full(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|&cell| cell != Cell::Empty))
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = Player::X;

    loop {
        board.print();
        let mut input = String::new();
        println!(
            "Player {:?}, enter your move (row and column) separated by space:",
            current_player
        );

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let row: usize = match iter.next().unwrap_or("0").parse() {
            Ok(val) => val,
            Err(_) => continue,
        };
        let col: usize = match iter.next().unwrap_or("0").parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        if !board.place_marker(row, col, current_player) {
            println!("Invalid move, try again.");
            continue;
        }

        if let Some(winner) = board.check_winner() {
            board.print();
            println!("Player {:?} wins!", winner);
            break;
        }

        if board.is_full() {
            board.print();
            println!("The game is a draw!");
            break;
        }

        current_player = match current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }
}
