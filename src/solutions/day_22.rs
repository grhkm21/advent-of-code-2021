use std::cmp::*;

struct Cube {
    is_on: bool,
    x1: isize,
    x2: isize,
    y1: isize,
    y2: isize,
    z1: isize,
    z2: isize,
}

impl Cube {
    pub fn intersect(&self, c2: &Cube) -> Option<Cube> {
        let x1 = max(self.x1, c2.x1);
        let x2 = min(self.x2, c2.x2);
        let y1 = max(self.y1, c2.y1);
        let y2 = min(self.y2, c2.y2);
        let z1 = max(self.z1, c2.z1);
        let z2 = min(self.z2, c2.z2);

        if x1 > x2 || y1 > y2 || z1 > z2 {
            return None;
        }

        let new_state;
        if self.is_on && c2.is_on {
            new_state = false;
        } else if !self.is_on && !c2.is_on {
            new_state = true;
        } else {
            new_state = c2.is_on;
        }

        let new_cube = Cube {
            is_on: new_state,
            x1: x1,
            x2: x2,
            y1: y1,
            y2: y2,
            z1: z1,
            z2: z2,
        };
        return Some(new_cube);
    }
}

fn make_tuple(s: &str) -> (isize, isize) {
    let (s1, s2) = s.split_once("..").unwrap();
    (s1.parse::<isize>().unwrap(), s2.parse::<isize>().unwrap())
}

fn solve_with_bounds(contents: &str, bound: isize) -> isize {
    let mut cubes = Vec::<Cube>::new();
    for line in contents.lines() {
        // process on / off
        let (set_str, ranges) = line.split_once(" ").unwrap();
        let set = (set_str == "on") as bool;

        // process ranges
        let (x, y, z) = if let [x, y, z] = ranges
            .split(",")
            .map(|c| make_tuple(&c[2..]))
            .collect::<Vec<(isize, isize)>>()[..]
        {
            (x, y, z)
        } else {
            panic!("err: invalid input")
        };

        let cur_cube = Cube {
            is_on: set,
            x1: x.0,
            y1: y.0,
            z1: z.0,
            x2: x.1,
            y2: y.1,
            z2: z.1,
        };

        // check bounds
        if cur_cube.x1.abs() > bound
            || cur_cube.x2.abs() > bound
            || cur_cube.y1.abs() > bound
            || cur_cube.y2.abs() > bound
            || cur_cube.z1.abs() > bound
            || cur_cube.z2.abs() > bound
        {
            continue;
        }

        let mut add_queue = Vec::<Cube>::new();

        for cube in &cubes {
            let intersection = cube.intersect(&cur_cube);
            match intersection {
                Some(c) => add_queue.push(c),
                None => {}
            }
        }

        if set {
            add_queue.push(cur_cube);
        }

        cubes.extend(add_queue);
    }

    let mut total_vol = 0;
    for cube in cubes {
        let vol = (cube.x2 - cube.x1 + 1) * (cube.y2 - cube.y1 + 1) * (cube.z2 - cube.z1 + 1);
        if cube.is_on {
            total_vol += vol;
        } else {
            total_vol -= vol;
        }
    }

    total_vol
}

pub fn solve(contents: &str) -> (isize, isize) {
    let part1 = solve_with_bounds(contents, 50);
    let part2 = solve_with_bounds(contents, 1000000);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {}
}
