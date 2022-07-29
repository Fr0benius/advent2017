use crate::util::arr2::Arr2;

pub const INPUT: &str = include_str!("../input/19.txt");

fn solve(input: &str) -> (String, usize) {
    let n = input.lines().count();
    let m = input.lines().next().unwrap().len();
    let g = Arr2::from_raw(n, m, input.bytes().filter(|&b| b != b'\n').collect());
    let mut pos = (0i64, g.row(0).position(|&c| c == b'|').unwrap() as i64);
    let dirs = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut d = 0;
    let mut s = vec![];
    let mut cnt = 0;
    loop {
        let (r, c) = pos;
        let b = g[r as usize][c as usize];
        if b.is_ascii_uppercase() {
            s.push(b as char);
        }
        if b == b' ' {
            break;
        }
        cnt += 1;
        if b == b'+' {
            for j in [(d + 1) % 4, (d + 3) % 4] {
                let rn = r + dirs[j].0;
                let cn = c + dirs[j].1;
                if g[rn as usize][cn as usize] != b' ' {
                    pos = (rn, cn);
                    d = j;
                    break;
                }
            }
        } else {
            pos = (r + dirs[d].0, c + dirs[d].1);
        }
    }
    (s.into_iter().collect(), cnt)
}

pub fn part1(input: &str) -> String {
    solve(input).0
}

pub fn part2(input: &str) -> usize {
    solve(input).1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "RUEDAHWKSM");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 17264);
    }
}
