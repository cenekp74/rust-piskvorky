fn main() {
    let mut game = Game::new();
    let success = game.play((1, 1));
    println!("success: {}", success);
    println!("Board: \n {:?}", game.board)
}

#[derive(Copy, Clone)]
#[derive(Debug)]
enum Field {
    Player(Player),
    Empty,
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
    fn new() -> Game {
        Game {
            board: [[Field::Empty; 15]; 15],
            to_move: Player::Player1,
        }
    }
}
  