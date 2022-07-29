use std::cmp::max;

pub const INPUT: &str = include_str!("../input/11.txt");

pub fn part1(input: &str) -> i64 {
    norm(input.trim().split(',').fold((0, 0), step))
}

pub fn part2(input: &str) -> i64 {
    input
        .trim()
        .split(',')
        .scan((0, 0), |pt, op| {
            *pt = step(*pt, op);
            Some(norm(*pt))
        })
        .max()
        .unwrap()
}

fn step((x, y): (i64, i64), op: &str) -> (i64, i64) {
    match op {
        "se" => (x + 1, y),
        "ne" => (x + 1, y + 1),
        "n" => (x, y + 1),
        "nw" => (x - 1, y),
        "sw" => (x - 1, y - 1),
        "s" => (x, y - 1),
        _ => unreachable!(),
    }
}
fn norm((x, y): (i64, i64)) -> i64 {
    if x.signum() == y.signum() {
        max(x.abs(), y.abs())
    } else {
        (x - y).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 747);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1544);
    }
}
