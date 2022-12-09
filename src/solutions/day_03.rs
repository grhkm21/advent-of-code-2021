use std::collections::BTreeMap;

pub fn solve(contents: &str) -> (usize, usize) {
    let mut cnt: usize = 0;
    let mut freq = BTreeMap::<usize, usize>::new();

    for line in contents.lines().map(|l| l.chars()) {
        cnt += 1;
        for (i, c) in line.enumerate() {
            let ptr: &mut usize = freq.entry(i).or_insert(0);
            if c == '1' {
                *ptr += 1;
            }
        }
    }

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;

    for (_, val) in freq {
        let common_bit = (2 * val >= cnt) as usize;
        gamma = 2 * gamma + common_bit;
        epsilon = 2 * epsilon + (1 - common_bit);
    }

    let mut ratings = 1;
    for mode in [0, 1] {
        let mut lines: Vec<Vec<usize>> = contents
            .lines()
            .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
            .collect();

        let mut i = 0;
        while lines.len() > 1 && i < lines[0].len() {
            // count which bit is more frequent
            let column: Vec<usize> = lines.iter().map(|l| l[i]).collect();
            let cnt1: usize = column.iter().sum();
            let keep_bit = if lines.len() - cnt1 > cnt1 {
                mode
            } else {
                1 - mode
            };

            // filter out
            lines = lines
                .iter()
                .filter(|&line| line[i] == keep_bit)
                .cloned()
                .collect();
            i += 1;
        }
        let mut rating = 0;
        for bit in &lines[0] {
            rating = rating * 2 + bit;
        }
        ratings *= rating;
    }

    (gamma * epsilon, ratings)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {}
}
