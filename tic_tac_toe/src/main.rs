use std::io;

const BOARD_SIZE: usize = 3;

struct Game {
    board: [[Option<char>; BOARD_SIZE]; BOARD_SIZE],
    player: char,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            player: 'X',
        }
    }

    fn switch_player(&mut self) {
        self.player = match self.player {
            'X' => 'O',
            'O' => 'X',
            _ => unreachable!(),
        };
    }

    fn display_board(&self) {
        println!("\n");
        for row in &self.board {
            for cell in row {
                match cell {
                    Some(player) => print!(" {} ", player),
                    None => print!(" - "),
                }
            }
            println!("\n");
        }
    }

    fn make_move(&mut self) {
        let (row, col) = self.read_move();
        self.board[row][col] = Some(self.player);
    }

    fn read_move(&self) -> (usize, usize) {
        loop {
            println!("Player {} turn:", self.player);
            println!("Enter row and column (e.g. '1 2'):");

            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read line");

            let mut parts = input.trim().split_whitespace();

            let row = match parts.next().unwrap().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let col = match parts.next().unwrap().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if row >= BOARD_SIZE || col >= BOARD_SIZE {
                println!("Invalid input: row and column must be between 0 and {}", BOARD_SIZE - 1);
                continue;
            }

            // if let Some(_player) = self.board[row][col] {
            //     println!("Cell is already occupied");
            //     continue;
            // }

            return (row, col);
        }
    }

    fn game_over(&self) -> bool {
        for row in 0..BOARD_SIZE {
            if self.board[row][0] == Some(self.player) && self.board[row][1] == Some(self.player) && self.board[row][2] == Some(self.player) {
                return true;
            }
        }

        for col in 0..BOARD_SIZE {
            if self.board[0][col] == Some(self.player) && self.board[1][col] == Some(self.player) && self.board[2][col] == Some(self.player) {
                return true;
            }
        }

        if self.board[0][0] == Some(self.player) && self.board[1][1] == Some(self.player) && self.board[2][2] == Some(self.player) {
            return true;
        }

        if self.board[0][2] == Some(self.player) && self.board[1][1] == Some(self.player) && self.board[2][0] == Some(self.player) {
            return true;
        }

        for row in &self.board {
            for cell in row {
                if cell.is_none() {
                    return false;
                }
            }
        }

        return true;
    }
}

fn main() {
    let mut game = Game::new();

    while !game.game_over() {
        game.display_board();
        game.make_move();
        game.switch_player();
    }

    game.display_board();
    println!("Player {} wins!", game.player);
}

