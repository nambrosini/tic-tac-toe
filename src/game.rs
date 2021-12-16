use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct Board {
    pub board: [char; 9],
}

impl Board {
    pub fn new() -> Self {
        Self { board: [' '; 9] }
    }

    pub fn get_available(&self) -> Vec<usize> {
        self.board
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == ' ')
            .map(|(i, _)| i)
            .collect()
    }

    pub fn play_move(&mut self, position: usize, player: char) -> Result<(), BoardError> {
        if self.board[position] != ' ' {
            return Err(BoardError::PositionError(format!(
                "Position {} is occupied by {}",
                position, self.board[position]
            )));
        }

        self.board[position] = player;

        Ok(())
    }

    pub fn get_winner(&self) -> (bool, Option<char>) {
        for i in 0..3 {
            if self.board[i] == self.board[i + 3]
                && self.board[i] == self.board[i + 6]
                && self.board[i] != ' '
            {
                return (true, Some(self.board[i]));
            }

            if self.board[i * 3] == self.board[i * 3 + 1]
                && self.board[i * 3] == self.board[i * 3 + 2]
                && self.board[i * 3] != ' '
            {
                return (true, Some(self.board[i * 3]));
            }
        }

        if self.board[0] == self.board[4] && self.board[4] == self.board[8] && self.board[0] != ' '
        {
            return (true, Some(self.board[0]));
        }

        if self.board[2] == self.board[4] && self.board[4] == self.board[6] && self.board[2] != ' '
        {
            return (true, Some(self.board[2]));
        }

        if self.count_symbol(' ') == 0 {
            return (true, None);
        }

        (false, None)
    }

    pub fn get_turn(&self) -> char {
        let free = self.count_symbol(' ');
        if free % 2 > 0 {
            'X'
        } else {
            'O'
        }
    }

    fn count_symbol(&self, symbol: char) -> usize {
        self.board.iter().filter(|x| x == &&symbol).count()
    }

    pub fn get_hash(&self) -> String {
        self.board.iter().collect()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..9 {
            if i % 3 == 0 {
                writeln!(f)?;
                if i != 0 {
                    writeln!(f, "-----------")?;
                }
            } else {
                write!(f, "|")?;
            }
            write!(f, " {} ", self.board[i])?;
        }
        writeln!(f)
    }
}

pub enum BoardError {
    PositionError(String),
}

impl BoardError {
    pub fn value(&self) -> String {
        match self {
            BoardError::PositionError(s) => s.clone(),
        }
    }
}
