use std::{fmt::Display, ops::AddAssign};

#[derive(Copy, Clone)]
pub struct Results {
    pub playing_x: Stat,
    pub playing_o: Stat,
}

impl Results {
    pub fn new() -> Self {
        Self {
            playing_x: Stat::new(),
            playing_o: Stat::new(),
        }
    }
}

impl Display for Results {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\t\tWins\tDraws\tLost")?;
        writeln!(
            f,
            "\tX\t{}\t{}\t{}\t{}",
            self.playing_x.wins,
            self.playing_x.draws,
            self.playing_x.losts,
            self.playing_x.sum()
        )?;
        writeln!(
            f,
            "\tO\t{}\t{}\t{}\t{}",
            self.playing_o.wins,
            self.playing_o.draws,
            self.playing_o.losts,
            self.playing_o.sum()
        )
    }
}

#[derive(Copy, Clone)]
pub struct Stat {
    wins: usize,
    draws: usize,
    losts: usize,
}

impl Stat {
    fn new() -> Self {
        Self {
            wins: 0,
            draws: 0,
            losts: 0,
        }
    }
}

impl Stat {
    pub fn sum(&self) -> usize {
        self.wins + self.draws + self.losts
    }
}

impl AddAssign<(usize, usize, usize)> for Stat {
    fn add_assign(&mut self, rhs: (usize, usize, usize)) {
        self.wins += rhs.0;
        self.draws += rhs.1;
        self.losts += rhs.2;
    }
}