use crate::game::Board;
use cached::proc_macro::cached;

pub fn minimax_search(board: &Board, my_sign: u8) -> usize {
    let board: &'static [u8; 9] = &board.board;
    let (_, action) = max_value(&board, i32::MIN, i32::MAX, my_sign);
    action.unwrap()
}

#[cached]
fn max_value(board: &'static [u8; 9], alpha: i32, beta: i32, my_sign: u8) -> (i32, Option<usize>) {
    let winner = check_winner(board, my_sign);
    if let Some(result) = winner {
        return (result, None);
    }

    let mut alpha = alpha;

    let mut max = i32::MIN;
    let mut action = None;

    for p in get_availables(board) {
        let mut new_board = *board;
        new_board[p] = my_sign;

        let (v, _) = min_value(&new_board, alpha, beta, my_sign);
        if v > max {
            max = v;
            action = Some(p);
        }

        alpha = alpha.max(v);

        if beta <= alpha {
            return (max, action);
        }
    }

    (max, action)
}

#[cached]
fn min_value(board: &[u8; 9], alpha: i32, beta: i32, my_sign: u8) -> (i32, Option<usize>) {
    let winner = check_winner(board, my_sign);
    if let Some(result) = winner {
        return (result, None);
    }

    let mut beta = beta;

    let mut min = i32::MAX;
    let mut action: Option<usize> = None;

    for p in get_availables(board) {
        let mut new_board = *board;
        new_board[p] = if my_sign == b'X' { b'O' } else { b'X' };

        let (v, _) = max_value(&new_board, alpha, beta, my_sign);
        if v < min {
            min = v;
            action = Some(p);
        }

        beta = beta.min(v);

        if beta <= alpha {
            return (min, action);
        }
    }

    (min, action)
}

fn check_winner(board: &[u8; 9], my_sign: u8) -> Option<i32> {
    for i in 0..3 {
        if equals_3(board[i], board[i + 3], board[i + 6]) {
            if board[i] == my_sign {
                return Some(10);
            } else {
                return Some(-10);
            }
        }

        if equals_3(board[i * 3], board[i * 3 + 1], board[i * 3 + 2]) {
            if board[i * 3] == my_sign {
                return Some(10);
            } else {
                return Some(-10);
            }
        }
    }

    if equals_3(board[0], board[4], board[8]) {
        if board[0] == my_sign {
            return Some(10);
        } else {
            return Some(-10);
        }
    }

    if equals_3(board[2], board[4], board[6]) {
        if board[2] == my_sign {
            return Some(10);
        } else {
            return Some(-10);
        }
    }

    if board.iter().filter(|&&c| c == b' ').count() == 0 {
        return Some(0);
    }

    None
}

fn equals_3(a: u8, b: u8, c: u8) -> bool {
    a == b && b == c && a != b' '
}

fn get_availables(board: &[u8; 9]) -> Vec<usize> {
    board
        .iter()
        .enumerate()
        .filter(|(_, &e)| e == b' ')
        .map(|(i, _)| i)
        .collect()
}
