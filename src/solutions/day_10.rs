use std::collections::VecDeque;

fn calc_score(s: &str) -> (usize, usize, bool) {
    let mut deque = VecDeque::<char>::new();
    for c in s.chars() {
        match c {
            '{' | '(' | '[' | '<' => deque.push_back(c),
            ')' | ']' | '}' | '>' => {
                let (target, score) = match c {
                    ')' => ('(', 3),
                    ']' => ('[', 57),
                    '}' => ('{', 1197),
                    '>' => ('<', 25137),
                    _ => unreachable!(),
                };
                if *deque.back().expect("err: deque is empty") != target {
                    return (score, 0, false);
                }
                deque.pop_back();
            }
            _ => unreachable!(),
        }
    }

    let part1 = 0;

    // filling in the rest for part 2
    let mut part2 = 0;
    while !deque.is_empty() {
        let c = deque.pop_back().expect("");
        part2 = part2 * 5
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            }
    }

    (part1, part2, true)
}

pub fn solve(contents: &str) -> (usize, usize) {
    let mut part1 = 0;
    let mut scores = Vec::new();

    for line in contents.lines() {
        let (score1, score2, incomplete) = calc_score(line);
        part1 += score1;
        if incomplete {
            scores.push(score2);
        }
    }

    scores.sort();
    let part2 = scores[scores.len() / 2];

    (part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {}
}
