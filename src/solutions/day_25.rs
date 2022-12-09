fn advance(board: &mut Vec<Vec<char>>) -> bool {
    let mut moved = false;
    let r = board.len();
    let c = board[0].len();

    // check for moving right
    let mut move_r = vec![vec![false; c]; r];
    for i in 0..r {
        for j in 0..c {
            move_r[i][j] = board[i][j] == '>' && board[i][(j + 1) % c] == '.';
        }
    }
    for i in 0..r {
        for j in 0..c {
            if move_r[i][j] {
                moved = true;
                board[i][j] = '.';
                board[i][(j + 1) % c] = '>';
            }
        }
    }

    // check for moving down
    let mut move_d = vec![vec![false; c]; r];
    for i in 0..r {
        for j in 0..c {
            move_d[i][j] = board[i][j] == 'v' && board[(i + 1) % r][j] == '.';
        }
    }
    for i in 0..r {
        for j in 0..c {
            if move_d[i][j] {
                moved = true;
                board[i][j] = '.';
                board[(i + 1) % r][j] = 'v';
            }
        }
    }

    moved
}

pub fn solve(contents: &str) -> (usize, usize) {
    let mut board = contents
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut part1 = 0;
    loop {
        part1 += 1;
        if !advance(&mut board) {
            break;
        }
    }

    let part2 = 0;

    (part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {}
}
