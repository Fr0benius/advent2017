use std::collections::{HashSet, HashMap};

pub const INPUT: &str = include_str!("../input/22.txt");

pub fn part1(input: &str) -> i64 {
    let mut g = HashSet::new();
    let n = input.lines().count() as i64;
    let m = input.lines().next().unwrap().len() as i64;
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.bytes().enumerate() {
            if c == b'#' {
                g.insert((i as i64, j as i64));
            }
        }
    }
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut pos = ((n - 1) / 2, (m - 1) / 2);
    let mut d = 0;
    let mut res = 0;
    for _ in 0..10_000 {
        if g.contains(&pos) {
            d = (d + 1) % 4;
            g.remove(&pos);
        } else {
            d = (d + 3) % 4;
            g.insert(pos);
            res += 1;
        }
        pos = (pos.0 + dirs[d].0, pos.1 + dirs[d].1);
    }
    res
}

pub fn part2(input: &str) -> i64 {
    let mut g = HashMap::new();
    let n = input.lines().count() as i64;
    let m = input.lines().next().unwrap().len() as i64;
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.bytes().enumerate() {
            if c == b'#' {
                g.insert((i as i64, j as i64), 2);
            }
        }
    }
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut pos = ((n - 1) / 2, (m - 1) / 2);
    let mut d = 0;
    let mut res = 0;
    for _ in 0..10_000_000 {
        let state = g.get(&pos).copied().unwrap_or(0);
        g.insert(pos, (state + 1) % 4);
        if state == 1 {
            res += 1;
        }
        d = (d + state + 3) % 4;
        pos = (pos.0 + dirs[d].0, pos.1 + dirs[d].1);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 5176);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2_512_017);
    }
}
