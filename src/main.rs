const BOARD_SIZE: usize = 15;

struct Board {
    board: [[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
    move_history: Vec<[usize; 2]>,
    to_move: Player,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Player {
    Player1,
    Player2,
}

impl Board {
    fn new(to_move: Player) -> Self {
        Board {
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            move_history: vec![],
            to_move: to_move,
        }
    }

    fn make_move(&mut self, row: usize, col: usize, player: Player) -> Result<(), &'static str> {
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return Err("Invalid move. Out of bounds.");
        }

        if self.board[row][col].is_some() {
            return Err("Invalid move. Field already occupied.");
        }

        self.board[row][col] = Some(player);
        self.move_history.push([row, col]);
        self.change_to_move();
        Ok(())
    }

    fn change_to_move(&mut self) {
        if self.to_move == Player::Player1 {self.to_move = Player::Player2;}
        else {self.to_move = Player::Player1;}
    }

    fn undo_move(&mut self) {
        let last_move = self.move_history.last().unwrap();
        self.board[last_move[0]][last_move[1]] = None;
        self.move_history.pop();
        self.change_to_move();
    }

    fn repr(&mut self) -> String{
        let mut s: String = String::new();
        for row in self.board {
            s.push('|');
            for field in row {
                match field {
                    None => { s.push_str(" |"); }
                    Some(Player::Player1) => { s.push_str("X|"); }
                    Some(Player::Player2) => { s.push_str("O|"); }
                }
            }
            s.push('\n');
        }
        return s;
    }

    fn check_for_winner(&mut self) {
        for row in self.board {
            for field in row {
                println!("{:?}", field);
            }
        }
    }
}

fn main() {
    let player1 = Player::Player1;
    let player2 = Player::Player2;
    let mut board = Board::new(player1);
    board.make_move(10, 5, board.to_move).expect("Move making failed: ");
    println!("{}", board.repr());
    board.make_move(11, 5, board.to_move).expect("Move making failed: ");
    println!("{}", board.repr());
    board.undo_move();
    println!("{}", board.repr());
}
