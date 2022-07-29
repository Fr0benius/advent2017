pub const INPUT: &str = include_str!("../input/01.txt");

pub fn part1(input: &str) -> i64 {
    let v: Vec<u8> = input.trim().bytes().collect();
    let n = v.len();
    (0..n).filter_map(|i| if v[i] == v[(i + 1) % n] {
        Some((v[i] - b'0') as i64)
    } else {
        None
    }).sum()
}

pub fn part2(input: &str) -> i64 {
    let v: Vec<u8> = input.trim().bytes().collect();
    let n = v.len();
    (0..n).filter_map(|i| if v[i] == v[(i + n/2) % n] {
        Some((v[i] - b'0') as i64)
    } else {
        None
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1253);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1278);
    }
}
