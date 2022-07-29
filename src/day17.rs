pub const INPUT: &str = include_str!("../input/17.txt");

pub fn part1(input: &str) -> usize {
    let step: usize = input.trim().parse().unwrap();
    let mut nxt = vec![0];
    let mut cur = 0;
    for k in 0..2017 {
        for _ in 0..step {
            cur = nxt[cur];
        }
        nxt.push(nxt[cur]);
        nxt[cur] = k + 1;
        cur = k + 1;
    }
    nxt[2017]
}

pub fn part2(input: &str) -> usize {
    let step: usize = input.trim().parse().unwrap();
    let mut cur = 0;
    let mut res = 0;
    for k in 1..=500_000_000 {
        cur = (cur + step) % k;
        if cur == 0 {
            res = k;
        }
        cur += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 204);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 28_954_211);
    }
}
