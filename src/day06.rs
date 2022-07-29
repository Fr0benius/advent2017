use std::{collections::{HashSet, HashMap}, cmp::Reverse};

use crate::util::parsing::Gather;

pub const INPUT: &str = include_str!("../input/06.txt");

pub fn part1(input: &str) -> usize {
    let mut v: Vec<i64> = input.split_whitespace().gather();
    let n = v.len();
    let mut set = HashSet::new();
    loop {
        if !set.insert(v.clone()) {
            return set.len();
        }
        let mut j = (0..n).max_by_key(|&i| (v[i], Reverse(i))).unwrap();
        let mut x = v[j];
        v[j] = 0;
        while x > 0 {
            j = (j + 1) % n;
            v[j] += 1;
            x -= 1;
        }
    }
}

pub fn part2(input: &str) -> usize {
    let mut v: Vec<i64> = input.split_whitespace().gather();
    let n = v.len();
    let mut map = HashMap::new();
    for gen in 0.. {
        if let Some(prev) = map.insert(v.clone(), gen) {
            return gen - prev
        }
        let mut j = (0..n).max_by_key(|&i| (v[i], Reverse(i))).unwrap();
        let mut x = v[j];
        v[j] = 0;
        while x > 0 {
            j = (j + 1) % n;
            v[j] += 1;
            x -= 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3156);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1610);
    }
}
