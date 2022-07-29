use crate::util::parsing::Gather;

pub const INPUT: &str = include_str!("../input/15.txt");

const A: i64 = 16807;
const B: i64 = 48271;
const MOD: i64 = 2_147_483_647;
pub fn part1(input: &str) -> usize {
    let (mut a, mut b): (i64, i64) = input
        .lines()
        .map(|line| line.split("starts with ").nth(1).unwrap())
        .gather();
    let mut count = 0;
    for _ in 0..40_000_000 {
        a = a * A % MOD;
        b = b * B % MOD;
        if a & ((1 << 16) - 1) == b & ((1 << 16) - 1) {
            count += 1;
        }
    }
    count
}

pub fn part2(input: &str) -> i64 {
    let (mut a, mut b): (i64, i64) = input
        .lines()
        .map(|line| line.split("starts with ").nth(1).unwrap())
        .gather();
    let mut count = 0;
    for _ in 0..5_000_000 {
        loop {
            a = a * A % MOD;
            if a % 4 == 0 {
                break;
            }
        }
        loop {
            b = b * B % MOD;
            if b % 8 == 0 {
                break;
            }
        }
        if a & ((1 << 16) - 1) == b & ((1 << 16) - 1) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 626);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 306);
    }
}
