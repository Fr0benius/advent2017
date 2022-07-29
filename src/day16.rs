use crate::util::parsing::Gather;

pub const INPUT: &str = include_str!("../input/16.txt");

const K: usize = (b'p' - b'a' + 1) as usize;
pub fn part1(input: &str) -> String {
    let (iperm, lperm) = get_perms(input);
    apply(&iperm, &lperm)
}

pub fn part2(input: &str) -> String {
    let (iperm, lperm) = get_perms(input);
    let times = 1_000_000_000;
    apply(&pow(&iperm, times), &pow(&lperm, times))
}

fn get_perms(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut iperm: Vec<usize> = (0..K).collect();
    let mut inv: Vec<usize> = (0..K).collect();
    for s in input.trim().split(',') {
        let (op, args) = s.split_at(1);
        match op.as_bytes()[0] {
            b's' => {
                let k: usize = args.parse().unwrap();
                assert!(k < K);
                iperm.rotate_right(k);
            }
            b'x' => {
                let (a, b): (usize, usize) = args.split('/').gather();
                iperm.swap(a, b);
            }
            b'p' => {
                let i = (args.as_bytes()[0] - b'a') as usize;
                let j = (args.as_bytes()[2] - b'a') as usize;
                inv.swap(i, j);
            }
            _ => unreachable!(),
        }
    }
    let mut lperm = vec![0; K];
    for (a, b) in inv.into_iter().enumerate() {
        lperm[b] = a;
    }
    (iperm, lperm)
}

fn apply(iperm: &[usize], lperm: &[usize]) -> String {
    iperm
        .iter()
        .map(|&i| (lperm[i] as u8 + b'a') as char)
        .collect()
}

fn pow(perm: &[usize], k: usize) -> Vec<usize> {
    let mut seen = vec![false; K];
    let mut res = vec![0; K];
    for i in 0..K {
        if seen[i] {
            continue;
        }
        let mut cyc = vec![];
        let mut j = i;
        while !seen[j] {
            seen[j] = true;
            cyc.push(j);
            j = perm[j];
        }
        let m = cyc.len();
        for ix in 0..m {
            res[cyc[ix]] = cyc[(ix + k) % m];
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "fnloekigdmpajchb");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "amkjepdhifolgncb");
    }
}
