use crate::util::parsing::Gather;

pub const INPUT: &str = include_str!("../input/02.txt");

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let v: Vec<i64> = line.split_whitespace().gather();
            v.iter().max().unwrap() - v.iter().min().unwrap()
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let v: Vec<i64> = line.split_whitespace().gather();
            for i in 0..v.len() {
                for j in 0..v.len() {
                    if i != j && v[i] % v[j] == 0 {
                        return v[i] / v[j];
                    }
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 45351);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 275);
    }
}
