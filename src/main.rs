fn main() {
    let mut game = Game::new();
    let success = game.play((1, 1));
    println!("success: {}", success);
    println!("{}", game.to_str())
}

#[derive(Copy, Clone)]
#[derive(Debug)]
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
#[derive(Debug)]
enum Player {
    Player1,
    Player2,
}

#[derive(Debug)]
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
    fn new() -> Game {
        Game {
            board: [[Field::Empty; 15]; 15],
            to_move: Player::Player1,
        }
    }
}
  