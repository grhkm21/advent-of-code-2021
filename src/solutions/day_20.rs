use std::collections::HashMap;

fn vec_to_int(vec: &Vec<usize>) -> usize {
    // convert bitvector into usize
    let mut res = 0;
    for v in vec {
        res = res * 2 + v;
    }
    res
}

fn extend(pixels: &mut Vec<Vec<usize>>, r: usize, c: usize) {
    // add left and right border
    for i in 0..r {
        pixels[i].insert(0, 0);
        pixels[i].push(0);
    }

    // add top and bottom border
    pixels.insert(0, vec![0; c + 2]);
    pixels.push(vec![0; c + 2]);
}

fn enhance(pixels: &mut Vec<Vec<usize>>, r: usize, c: usize, encoding: HashMap<usize, usize>) {
    // this function ignores all borders, call extend before enhance
    // compute res[i][j] for non-borders
    let mut res = vec![vec![0; c]; r];
    for i in 1..r - 1 {
        for j in 1..c - 1 {
            // consider 3x3 neighbor
            let mut vec = Vec::<usize>::new();
            for ni in i - 1..i + 2 {
                for nj in j - 1..j + 2 {
                    vec.push(pixels[ni][nj]);
                }
            }
            res[i][j] = encoding[&vec_to_int(&vec)];
        }
    }
    *pixels = res.clone();
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<usize>>, r: usize, c: usize) {
    for i in 0..r {
        for j in 0..c {
            print!("{}", if grid[i][j] == 1 { '#' } else { '.' });
        }
        println!();
    }
    println!();
}

fn enhance_iter(k: usize, contents: &str) -> usize {
    // process content from file
    let (encodings, grid) = contents.split_once("\n\n").unwrap();

    // process bitstring into hashmap
    let mut encoding: HashMap<usize, usize> = HashMap::new();
    for (i, c) in encodings.replace(" ", "").chars().enumerate() {
        encoding.insert(i, (c == '#') as usize);
    }

    // process grid into Vec<Vec<usize>>
    let mut grid: Vec<Vec<usize>> = grid
        .trim()
        .split("\n")
        .map(|r| r.chars().map(|x| (x == '#') as usize).collect())
        .collect();

    let mut r = grid.len();
    let mut c = grid[0].len();

    for _ in 0..k * 2 {
        extend(&mut grid, r, c);
        r += 2;
        c += 2;
    }

    // run enhance program k times
    for _ in 0..k {
        enhance(&mut grid, r, c, encoding.clone());
    }

    // count pixels
    let mut cnt = 0;
    for i in k..r - k {
        for j in k..c - k {
            cnt += grid[i][j];
        }
    }

    cnt
}

pub fn solve(contents: &str) -> (usize, usize) {
    let part1 = enhance_iter(2, contents);
    let part2 = enhance_iter(50, contents);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {}
}
