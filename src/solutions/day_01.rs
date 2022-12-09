fn count_increase(data: &Vec<usize>, step: usize) -> usize {
    let mut ans = 0;
    for i in 0..data.len() - step {
        if data[i + step] > data[i] {
            ans += 1;
        }
    }
    ans
}

pub fn solve(contents: &str) -> (usize, usize) {
    let data: Vec<usize> = contents
        .lines()
        .map(|c| c.parse().expect("err: failed to parse int"))
        .collect();

    let part1 = count_increase(&data, 1);
    let part2 = count_increase(&data, 3);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {}
}
