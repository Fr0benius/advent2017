use crate::util::parsing::Gather;

pub const INPUT: &str = include_str!("../input/13.txt");

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (layer, sz): (i64, i64) = line.split(": ").gather();
            if layer % (2 * (sz - 1)) == 0 {
                layer * sz
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let mods: Vec<(i64, i64)> = input
        .lines()
        .map(|line| {
            let (layer, sz): (i64, i64) = line.split(": ").gather();
            (layer, 2 * (sz - 1))
        })
        .collect();
    for k in 0.. {
        if mods.iter().all(|&(a, b)| (a + k) % b != 0) {
            return k;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1900);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 3_966_414);
    }
}
