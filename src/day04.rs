use std::collections::HashSet;

pub const INPUT: &str = include_str!("../input/04.txt");

pub fn part1(input: &str) -> usize {
    input.lines()
        .filter(|ln| {
            let n = ln.split_whitespace().count();
            let m = ln.split_whitespace().collect::<HashSet<_>>().len();
            n == m
        }).count()
}

pub fn part2(input: &str) -> usize {
    input.lines()
        .filter(|ln| {
            let n = ln.split_whitespace().count();
            let m = ln.split_whitespace()
                .map(|w| {
                    let mut s = w.to_owned().into_bytes();
                    s.sort_unstable();
                    s
                })
                .collect::<HashSet<_>>().len();
            n == m
        }).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 337);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 231);
    }
}
