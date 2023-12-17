// tictactoe.rs
pub(crate) mod tictactoe {


#[derive(Debug, Clone)]
pub(crate) struct TicTacToe {
    pub board: Vec<Vec<char>>
}

#[derive(Debug)]
pub(crate) enum GameError {
    InvalidMove(String),
    OutOfBounds,
    GameOver
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::InvalidMove(msg) => write!(f, "Invalid move: {}", msg),
            GameError::OutOfBounds => write!(f, "Out of bounds"),
            GameError::GameOver => write!(f, "Game over"),
        }
    }
}
impl std::error::Error for GameError {}

impl TicTacToe {
    pub(crate) fn new() -> Self
    { TicTacToe {
            board: vec![vec!['_'; 3]; 3],
        } }

    pub fn make_move(&mut self, row: usize, col: usize, player: char) -> Result<(), GameError> {
        if row < 3 && col < 3 && self.board[row][col] == '_' {
            self.board[row][col] = player;
            Ok(())
        } else
            { Err(GameError::InvalidMove(String::from("Cell already occupied or out of bounds"))) }
    }

    pub fn is_over(&self) -> Option<char> {
        // Check rows and columns
        for i in 0..3 {
            if self.board[i][0] != '_' && self.board[i][0] == self.board[i][1] && self.board[i][1] == self.board[i][2]
                { return Some(self.board[i][0]); }
            if self.board[0][i] != '_' && self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i]
                { return Some(self.board[0][i]); }
        }

        // Check diagonals
        if self.board[0][0] != '_' && self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2]
            { return Some(self.board[0][0]); }
        if self.board[0][2] != '_' && self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0]
            { return Some(self.board[0][2]); }

        // Check for a draw
        if self.board.iter().all(|row| row.iter().all(|&cell| cell != '_'))
            { return Some('D'); }

        None
    }
}

}
