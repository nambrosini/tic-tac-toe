use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct Board {
    pub board: [u8; 9],
}

impl Board {
    pub fn new() -> Self {
        Self { board: [b' '; 9] }
    }

    pub fn get_available(&self) -> Vec<usize> {
        self.board
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == b' ')
            .map(|(i, _)| i)
            .collect()
    }

    pub fn play_move(&mut self, position: usize, player: u8) -> Result<(), BoardError> {
        if self.board[position] != b' ' {
            return Err(BoardError::PositionError(format!(
                "Position {} is occupied by {}",
                position, self.board[position]
            )));
        }

        self.board[position] = player;

        Ok(())
    }

    pub fn get_winner(&self) -> (bool, Option<u8>) {
        for i in 0..3 {
            if self.board[i] == self.board[i + 3]
                && self.board[i] == self.board[i + 6]
                && self.board[i] != b' '
            {
                return (true, Some(self.board[i]));
            }

            if self.board[i * 3] == self.board[i * 3 + 1]
                && self.board[i * 3] == self.board[i * 3 + 2]
                && self.board[i * 3] != b' '
            {
                return (true, Some(self.board[i * 3]));
            }
        }

        if self.board[0] == self.board[4] && self.board[4] == self.board[8] && self.board[0] != b' '
        {
            return (true, Some(self.board[0]));
        }

        if self.board[2] == self.board[4] && self.board[4] == self.board[6] && self.board[2] != b' '
        {
            return (true, Some(self.board[2]));
        }

        if self.count_symbol(b' ') == 0 {
            return (true, None);
        }

        (false, None)
    }

    pub fn get_turn(&self) -> u8 {
        let free = self.count_symbol(b' ');
        if free % 2 > 0 {
            b'X'
        } else {
            b'O'
        }
    }

    fn count_symbol(&self, symbol: u8) -> usize {
        self.board.iter().filter(|x| x == &&symbol).count()
    }

    pub fn get_hash(&self) -> String {
        self.board.iter().map(|c| *c as char).collect()
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

