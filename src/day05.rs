use crate::util::parsing::Gather;

pub const INPUT: &str = include_str!("../input/05.txt");

pub fn part1(input: &str) -> i64 {
    let mut v: Vec<i64> = input.lines().gather();
    let n = v.len();
    let mut i = 0i64;
    for k in 0.. {
        if !(0..n as i64).contains(&i) {
            return k;
        }
        let nxt = i + v[i as usize];
        v[i as usize] += 1;
        i = nxt;
    }
    0
}

pub fn part2(input: &str) -> i64 {
    let mut v: Vec<i64> = input.lines().gather();
    let n = v.len();
    let mut i = 0i64;
    for k in 0.. {
        if !(0..n as i64).contains(&i) {
            return k;
        }
        let nxt = i + v[i as usize];
        v[i as usize] += if v[i as usize] >= 3 { -1 } else { 1 };
        i = nxt;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 358_309);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 28_178_177);
    }
}
