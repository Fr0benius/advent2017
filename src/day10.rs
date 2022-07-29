use std::ops::BitXor;

use crate::util::parsing::Gather;

pub const INPUT: &str = include_str!("../input/10.txt");

pub fn part1(input: &str) -> i64 {
    let lens: Vec<usize> = input.trim().split(',').gather();
    let mut v: Vec<u8> = (0..=255).collect();
    let mut pos = 0;
    for (skip, l) in lens.into_iter().enumerate() {
        v[0..l].reverse();
        v.rotate_left((l + skip) % 256);
        pos += l + skip;
    }
    v.rotate_right(pos % 256);
    v[0] as i64 * v[1] as i64
}

pub fn part2(input: &str) -> String {
    let lens: Vec<usize> = input
        .trim()
        .bytes()
        .chain([17, 31, 73, 47, 23])
        .map(|b| b as usize)
        .collect();
    let n = lens.len();
    let mut v: Vec<u8> = (0..=255).collect();
    let mut pos = 0;
    for (skip, l) in lens.into_iter().cycle().take(64 * n).enumerate() {
        v[0..l].reverse();
        v.rotate_left((l + skip) % 256);
        pos += l + skip;
    }
    v.rotate_right(pos % 256);
    v.chunks(16)
        .map(|chunk| chunk.iter().fold(0, BitXor::bitxor))
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 7888);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "decdf7d377879877173b7f2fb131cf1b");
    }
}
