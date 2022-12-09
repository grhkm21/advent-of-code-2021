#[allow(unused)]
pub fn solve(contents: &str) -> (isize, isize) {
    // part 1
    let mut xpos_1 = 0;
    let mut ypos_1 = 0;

    // part 2
    let mut xpos_2 = 0;
    let mut ypos_2 = 0;
    let mut aim_2 = 0;

    for line in contents.lines() {
        let (ins, arg) = line.split_once(" ").expect("err: can't parse line");
        let arg = arg.parse::<isize>().expect("err: can't parse int");
        match ins {
            "down" => {
                ypos_1 += arg;
                aim_2 += arg
            }
            "up" => {
                ypos_1 -= arg;
                aim_2 -= arg
            }
            "forward" => {
                xpos_1 += arg;
                xpos_2 += arg;
                ypos_2 += aim_2 * arg;
            }
            _ => unreachable!(),
        }
    }

    (xpos_1 * ypos_1, xpos_2 * ypos_2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {}
}
