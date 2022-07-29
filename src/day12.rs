use crate::util::{parsing::Gather, dsu::DSU};

pub const INPUT: &str = include_str!("../input/12.txt");

pub fn part1(input: &str) -> usize {
    let mut dsu = gen_dsu(input);
    (0..dsu.len()).filter(|&k| dsu.root(k) == dsu.root(0)).count()
}

pub fn part2(input: &str) -> usize {
    let dsu = gen_dsu(input);
    dsu.comps()
}

fn gen_dsu(input: &str) -> DSU {
    let n = input.lines().count();
    let mut dsu = DSU::new(n);
    for line in input.lines() {
        let (v, ws): (usize, &str) = line.split(" <-> ").gather();
        for w in ws.split(", ") {
            dsu.join(v, w.parse().unwrap());
        }
    }
    dsu
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 169);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 179);
    }
}
