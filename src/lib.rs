mod agent;
mod game;
mod mmagent;
mod stats;

use std::{collections::HashMap, io};

use agent::Agent;
use game::Board;
use stats::{Results};

use crate::mmagent::minimax_search;

use rand::prelude::*;

static MINIMAX: &str = "minimax";

pub fn train(name: &str, cycles: usize) {
    let mut p1 = Agent::new(name);
    let mut p2 = Agent::new("");

    for i in 0..cycles {
        if i % 1000 == 0 {
            println!("{} cycles", i);
        }

        if i == cycles / 2 {
            println!("Switching symbols");
            let tmp = p1.clone();
            p1 = p2.clone();
            p2 = tmp;
        }

        let mut board = Board::new();

        loop {
            let turn = board.get_turn();

            let action = if turn == b'X' {
                p1.get_best_action(&board, turn)
            } else {
                p2.get_best_action(&board, turn)
            };

            if let Err(e) = board.play_move(action, turn) {
                eprintln!("{}", e.value());
                continue;
            }

            if turn == b'X' {
                p1.add_state(&board)
            } else {
                p2.add_state(&board)
            };

            let (is_finished, winner) = board.get_winner();

            if is_finished {
                if let Some(winner) = winner {
                    if winner == b'X' {
                        p1.feed_reward(1.0);
                        p2.feed_reward(-1.0);
                    } else {
                        p1.feed_reward(-1.0);
                        p2.feed_reward(1.0);
                    }
                } else {
                    p2.feed_reward(0.5);
                }
                break;
            };
        }

        p1.reset();
        p2.reset();
    }
    if let Err(e) = p2.save_model() {
        eprintln!("{}", e);
    }
}

pub fn train_with_minimax(name: &str, cycles: usize) {
    let mut p1 = Agent::new(name);

    let mut agent = b'X';
    let mut minimax = b'O';

    for i in 0..cycles {
        if i % 1000 == 0 {
            println!("{} cycles", i);
        }

        if i == cycles / 2 {
            println!("Switching symbols");
            agent = b'O';
            minimax = b'X';
        }


        let mut board = Board::new();

        loop {
            let turn = board.get_turn();

            let action = if turn == agent {
                p1.get_best_action(&board, agent)
            } else {
                mmagent::minimax_search(&board, minimax)
            };

            if let Err(e) = board.play_move(action, turn) {
                eprintln!("{}", e.value());
                continue;
            }

            if turn == agent {
                p1.add_state(&board)
            };

            let (is_finished, winner) = board.get_winner();

            if is_finished {
                if let Some(winner) = winner {
                    if winner == agent {
                        p1.feed_reward(1.0);
                    } else {
                        p1.feed_reward(-1.0);
                    }
                } else if agent == b'O' {
                    p1.feed_reward(0.5);
                }
                break;
            };
        }

        p1.reset();
    }
    if let Err(e) = p1.save_model() {
        eprintln!("{}", e);
    }
}

pub fn play(agent_name: &str) {
    let mut p1 = Agent::new(agent_name);
    if let Err(e) = p1.load_model() {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    let mut board = Board::new();
    let mut winner = board.get_winner();

    let agent_begins = rand::thread_rng().gen::<bool>();

    if !agent_begins {
        println!("Starting board");
        println!("{}", board);
    }

    while !winner.0 {
        let turn = board.get_turn();

        let field = if agent_begins {
            if turn == b'X' {
                p1.get_best_action(&board, turn)
            } else {
                read_human_input()
            }
        } else if turn == b'O' {
            read_human_input()
        } else {
            p1.get_best_action(&board, turn)
        };

        if let Err(e) = board.play_move(field, turn) {
            eprintln!("{}", e.value());
            continue;
        }

        if agent_begins && turn == b'X' || !agent_begins && turn == b'O' {
            p1.add_state(&board);
        }

        println!("Game after {}'s move", turn);
        println!("{}", board);

        winner = board.get_winner();
    }

    let winner: String = if let Some(winner) = winner.1 {
        if winner == b'X' && agent_begins || winner == b'O' && !agent_begins {
            p1.feed_reward(1.0);
        } else {
            p1.feed_reward(-1.0);
        }
        winner.to_string()
    } else {
        if !agent_begins {
            p1.feed_reward(0.5);
        }
        "nobody".to_string()
    };

    println!("The winner is {}.", winner);
    p1.save_model().unwrap_or_else(|e| eprintln!("{}", e));
}

fn read_human_input() -> usize {
    loop {
        println!("Which field to set?");

        let mut position = String::new();

        if let Err(e) = io::stdin().read_line(&mut position) {
            eprintln!("{}", e.to_string());
            continue;
        };

        match position.trim().parse::<usize>() {
            Ok(v) => {
                if (1..=9).contains(&v) {
                    return v - 1;
                } else {
                    eprintln!("Number not between 1 and 9!");
                    continue;
                }
            }
            Err(e) => {
                eprintln!("{}", e.to_string());
                continue;
            }
        };
    }
}

pub fn play_minimax() {
    let mut board = Board::new();
    let mut winner = board.get_winner();

    let human_begins = rand::thread_rng().gen::<bool>();

    if human_begins {
        println!("Starting board");
        println!("{}", board);
    }

    while !winner.0 {
        let turn = if board.get_turn() == b'X' { b'O' } else { b'X' };

        let field = if human_begins {
            if turn == b'X' {
                read_human_input()
            } else {
                mmagent::minimax_search(&board, b'O')
            }
        } else if turn == b'X' {
            mmagent::minimax_search(&board, b'X')
        } else {
            read_human_input()
        };

        if let Err(e) = board.play_move(field, turn) {
            eprintln!("{}", e.value());
            continue;
        }

        println!("Game after {}'s move", turn);
        println!("{}", board);

        winner = board.get_winner();
    }

    let winner: String = if let Some(winner) = winner.1 {
        winner.to_string()
    } else {
        "nobody".to_string()
    };

    println!("The winner is {}.", winner);
}

pub fn play_agent_vs_minimax(agent_name: &str) {
    let mut wins: HashMap<String, Results> = HashMap::new();
    let mut p1 = Agent::new(agent_name);
    if let Err(e) = p1.load_model() {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    println!();
    for i in 0..100 {
        let mut board = Board::new();
        let mut winner = board.get_winner();

        let agent_begins = i < 50;

        while !winner.0 {
            let turn = board.get_turn();

            let field = if agent_begins {
                if turn == b'X' {
                    p1.get_best_action(&board, turn)
                } else {
                    minimax_search(&board, b'O')
                }
            } else if turn == b'X' {
                minimax_search(&board, b'X')
            } else {
                p1.get_best_action(&board, turn)
            };

            if let Err(e) = board.play_move(field, turn) {
                eprintln!("{}", e.value());
                continue;
            }

            if agent_begins && turn == b'X' || !agent_begins && turn == b'O' {
                p1.add_state(&board);
            }

            winner = board.get_winner();
        }

        if let Some(winner) = winner.1 {
            if agent_begins {
                if winner == b'X' {
                    let entry = wins
                        .entry(agent_name.to_string())
                        .or_insert_with(Results::new);
                    entry.playing_x += (1, 0, 0);
                    let entry = wins.entry(MINIMAX.to_string()).or_insert_with(Results::new);
                    entry.playing_o += (0, 0, 1);
                } else {
                    let entry = wins.entry(MINIMAX.to_string()).or_insert_with(Results::new);
                    entry.playing_x += (1, 0, 0);
                    let entry = wins
                        .entry(agent_name.to_string())
                        .or_insert_with(Results::new);
                    entry.playing_o += (0, 0, 1);
                }
            } else if winner == b'X' {
                let entry = wins.entry(MINIMAX.to_string()).or_insert_with(Results::new);
                entry.playing_x += (1, 0, 0);
                let entry = wins
                    .entry(agent_name.to_string())
                    .or_insert_with(Results::new);
                entry.playing_o += (0, 0, 1);
            } else {
                let entry = wins
                    .entry(agent_name.to_string())
                    .or_insert_with(Results::new);
                entry.playing_x += (1, 0, 0);
                let entry = wins.entry(MINIMAX.to_string()).or_insert_with(Results::new);
                entry.playing_o += (0, 0, 1);
            }

            if winner == b'X' && agent_begins || winner == b'O' && !agent_begins {
                p1.feed_reward(1.0);
            } else {
                p1.feed_reward(-1.0);
            }
            winner.to_string()
        } else {

            if agent_begins {
                let entry = wins
                    .entry(agent_name.to_string())
                    .or_insert_with(Results::new);
                entry.playing_x += (0, 1, 0);
                let entry = wins.entry(MINIMAX.to_string()).or_insert_with(Results::new);
                entry.playing_o += (0, 1, 0);
            } else {
                let entry = wins.entry(MINIMAX.to_string()).or_insert_with(Results::new);
                entry.playing_x += (0, 1, 0);
                let entry = wins
                    .entry(agent_name.to_string())
                    .or_insert_with(Results::new);
                entry.playing_o += (0, 1, 0);
            }

            if !agent_begins {
                p1.feed_reward(0.5);
            }
            "nobody".to_string()
        };

        p1.reset();
    }

    let agent = wins[agent_name];
    let minimax = wins[MINIMAX];

    println!();
    println!("Results:");
    println!("Agent");
    println!("{}", agent);
    println!("Minimax");
    println!("{}", minimax);

    p1.save_model().unwrap_or_else(|e| eprintln!("{}", e));
}

pub fn play_multiple(a1: &str, a2: &str) {
    let mut wins: HashMap<String, Results> = HashMap::new();
    let mut p1 = Agent::new(a1);
    if let Err(e) = p1.load_model() {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    let mut p2 = Agent::new(a1);
    if let Err(e) = p1.load_model() {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    let mut rng = rand::thread_rng();

    println!();
    for _ in 0..100 {
        let mut board = Board::new();
        let mut winner = board.get_winner();

        let agent1_begins = rng.gen::<bool>();

        while !winner.0 {
            let turn = board.get_turn();

            let field = if agent1_begins {
                if turn == b'X' {
                    p1.get_best_action(&board, turn)
                } else {
                    p2.get_best_action(&board, turn)
                }
            } else if turn == b'X' {
                p2.get_best_action(&board, turn)
            } else {
                p1.get_best_action(&board, turn)
            };

            if let Err(e) = board.play_move(field, turn) {
                eprintln!("{}", e.value());
                continue;
            }

            if agent1_begins && turn == b'X' || !agent1_begins && turn == b'O' {
                p1.add_state(&board)
            } else {
                p2.add_state(&board);
            }

            winner = board.get_winner();
        }

        if let Some(winner) = winner.1 {
            if agent1_begins {
                if winner == b'X' {
                    let entry = wins
                        .entry(a1.to_string())
                        .or_insert_with(Results::new);
                    entry.playing_x += (1, 0, 0);
                    let entry = wins.entry(a2.to_string()).or_insert_with(Results::new);
                    entry.playing_o += (0, 0, 1);
                } else {
                    let entry = wins.entry(a2.to_string()).or_insert_with(Results::new);
                    entry.playing_x += (1, 0, 0);
                    let entry = wins
                        .entry(a1.to_string())
                        .or_insert_with(Results::new);
                    entry.playing_o += (0, 0, 1);
                }
            } else if winner == b'X' {
                let entry = wins.entry(a2.to_string()).or_insert_with(Results::new);
                entry.playing_x += (1, 0, 0);
                let entry = wins
                    .entry(a1.to_string())
                    .or_insert_with(Results::new);
                entry.playing_o += (0, 0, 1);
            } else {
                let entry = wins
                    .entry(a1.to_string())
                    .or_insert_with(Results::new);
                entry.playing_x += (1, 0, 0);
                let entry = wins.entry(a2.to_string()).or_insert_with(Results::new);
                entry.playing_o += (0, 0, 1);
            }

            if winner == b'X' && agent1_begins || winner == b'O' && !agent1_begins {
                p1.feed_reward(1.0);
                p2.feed_reward(-1.0);
            } else {
                p1.feed_reward(-1.0);
                p2.feed_reward(1.0);
            }
            winner.to_string()
        } else {

            if agent1_begins {
                let entry = wins
                    .entry(a1.to_string())
                    .or_insert_with(Results::new);
                entry.playing_x += (0, 1, 0);
                let entry = wins.entry(a2.to_string()).or_insert_with(Results::new);
                entry.playing_o += (0, 1, 0);
            } else {
                let entry = wins.entry(a2.to_string()).or_insert_with(Results::new);
                entry.playing_x += (0, 1, 0);
                let entry = wins
                    .entry(a1.to_string())
                    .or_insert_with(Results::new);
                entry.playing_o += (0, 1, 0);
            }

            if !agent1_begins {
                p1.feed_reward(0.5);
            } else {
                p2.feed_reward(0.5);
            }
            "nobody".to_string()
        };

        p1.reset();
        p2.reset()
    }

    let agent = wins[a1];
    let minimax = wins[a2];

    println!();
    println!("Results:");
    println!("Agent {}", a1);
    println!("{}", agent);
    println!("Agent {}", a2);
    println!("{}", minimax);

    p1.save_model().unwrap_or_else(|e| eprintln!("{}", e));
    p2.save_model().unwrap_or_else(|e| eprintln!("{}", e));
}