use std::io;
use std::process;

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

    fn check_for_winner(&mut self) -> Option<Player>{
        let mut oneinrow: u8 = 0;
        let mut twoinrow: u8 = 0;

        //check horizontal
        for row in self.board {  
            for field in row {
                match field {
                    None => {
                        oneinrow = 0;
                        twoinrow = 0;
                    }
                    Some(Player::Player1) => {
                        oneinrow += 1;
                        twoinrow = 0;
                    }
                    Some(Player::Player2) => {
                        oneinrow = 0;
                        twoinrow += 1;
                    }
                }
                if oneinrow >= 5 {return Some(Player::Player1);}
                if twoinrow >= 5 {return Some(Player::Player2);}
            }
            oneinrow = 0;
            twoinrow = 0;
        }
        
        return None
    }
}

fn read_user_input() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().parse().expect("Input is not a valid number");
}

fn play_on_one_device() {
    let mut board = Board::new(Player::Player1);
    loop {
        println!("Enter row number: ");
        let row: usize = read_user_input();
        println!("Enter col number: ");
        let col: usize = read_user_input();

        board.make_move(row, col, board.to_move).expect("Move making failed: ");
        let winner = board.check_for_winner();
        match winner {
            None => {}
            Some(Player::Player1) => {println!("Player 1 won");process::exit(1)}
            Some(Player::Player2) => {println!("Player 2 won");process::exit(0)}
        }
        println!("{}", board.repr());
    }
}

fn main() {
    play_on_one_device();
}
