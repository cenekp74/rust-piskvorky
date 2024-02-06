use std::io;

fn main() {
    play_on_one_device();
}

fn read_user_input() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().parse().expect("Input is not a valid number");
}

fn play_on_one_device() -> Player {
    let mut game = Game::new();
    loop {
        println!("Enter row number: ");
        let row: usize = read_user_input();
        println!("Enter col number: ");
        let col: usize = read_user_input();

        let success = game.play((row, col));
        if !success {
            println!("Move making failed");
            continue;
        }
        let winner = game.check_for_winner();
        match winner {
            None => {}
            Some(Player::Player1) => {println!("Player 1 won");return Player::Player1}
            Some(Player::Player2) => {println!("Player 2 won");return Player::Player2}
        }
        println!("{}", game.to_str());
    }
}

#[derive(Copy, Clone)]
enum Field {
    Player(Player),
    Empty,
}

impl Field {
    fn to_str(&self) -> &str {
        match self {
            Field::Player(Player::Player1) => {
                return "X";
            }
            Field::Player(Player::Player2) => {
                return "O";
            }
            Field::Empty =>  {
                return  " ";
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Player {
    Player1,
    Player2,
}

struct Game {
    board: [[Field; 15]; 15],
    to_move: Player,
}

impl Game {
    fn play(&mut self, position: (usize, usize)) -> bool {
        match &self.board[position.0][position.1] {
            Field::Player(_) => {
                return false;
            }
            Field::Empty => {
                self.board[position.0][position.1] = Field::Player(self.to_move.clone());
                match self.to_move {
                    Player::Player1 => {
                        self.to_move = Player::Player2;
                    }
                    Player::Player2 => {
                        self.to_move = Player::Player1;
                    }
                }
                return true;
            }
        }
    }
    fn to_str(&self) -> String {
        let mut s = String::new();
        for row in self.board {
            s += "|";
            for field in row {
                s += field.to_str();
                s += "|";
            }
            s += "\n";
        }
        s
    }
    fn check_for_winner(&self) -> Option<Player> {
        let mut p1_in_row: u8 = 0;
        let mut p2_in_row: u8 = 0;
        // horizontal
        for row in self.board {
            for field in row {
                match field {
                    Field::Player(Player::Player1) => {
                        p1_in_row += 1;
                        p2_in_row = 0;
                    }
                    Field::Player(Player::Player2) => {
                        p2_in_row += 1;
                        p1_in_row = 0;
                    }
                    Field::Empty => {
                        p1_in_row = 0;
                        p2_in_row = 0;
                    }
                }
                if p1_in_row >= 5 {
                    return Some(Player::Player1);
                } else if p2_in_row >=5 {
                    return Some(Player::Player2);
                }
            }
            p1_in_row = 0;
            p2_in_row = 0;
        }
        None
    }
    fn new() -> Game {
        Game {
            board: [[Field::Empty; 15]; 15],
            to_move: Player::Player1,
        }
    }
}