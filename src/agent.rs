use crate::game::Board;
use rand::{prelude::*, seq::SliceRandom};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub struct Agent {
    name: String,
    states: Vec<String>,
    lr: f32,
    pub states_values: HashMap<String, f32>,
    exp_rate: f32,
}

impl Agent {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            states: vec![],
            lr: 0.2,
            states_values: HashMap::new(),
            exp_rate: 0.1,
        }
    }

    pub fn save_model(&self) -> Result<(), String> {
        let filename = format!("data/policy_{}", self.name);
        let serialized_string = match serde_json::to_string(&self.states_values) {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };

        let mut file = match File::create(filename.clone()) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        match file.write_all(serialized_string.as_bytes()) {
            Ok(_) => println!("Successfully saved to {}", filename),
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn load_model(&mut self) -> Result<(), String> {
        let filename = format!("data/policy_{}", self.name);

        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                return Err(format!(
                    "Cannot load agent {} file. Are you sure you have trained one first?",
                    self.name
                ))
            }
        };

        let mut serialized_string = String::new();

        match file.read_to_string(&mut serialized_string) {
            Ok(_) => {}
            Err(e) => return Err(e.to_string()),
        }

        let deserialized: HashMap<String, f32> = match serde_json::from_str(&serialized_string) {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };

        self.states_values = deserialized;

        Ok(())
    }

    pub fn get_best_action(&self, board: &Board, turn: u8) -> usize {
        let mut action: usize = 0;
        let positions = board.get_available();
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() <= self.exp_rate {
            action = *positions.choose(&mut rng).unwrap();
        } else {
            let mut max_value = f32::MIN;
            for p in positions.iter() {
                let mut next_board = board.clone();
                next_board.board[*p] = turn;
                let saved_state = self.states_values.get(&next_board.get_hash());
                let value = if let Some(v) = saved_state { *v } else { 0.0 };
                if value > max_value {
                    max_value = value;
                    action = *p;
                }
            }
        }

        action
    }

    pub fn feed_reward(&mut self, reward: f32) {
        let mut reward = reward;
        for s in self.states.iter().rev() {
            let entry = self.states_values.entry(s.clone()).or_insert(0.0);
            *entry += self.lr * (reward - *entry);
            reward = *entry;
        }
    }

    pub fn reset(&mut self) {
        self.states = vec![];
    }

    pub fn add_state(&mut self, board: &Board) {
        self.states.push(board.get_hash());
    }
}
