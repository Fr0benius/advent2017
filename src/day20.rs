use std::collections::{HashMap, HashSet};

use crate::util::parsing::{re_parser, Gather};

pub const INPUT: &str = include_str!("../input/20.txt");

pub fn part1(input: &str) -> usize {
    let parse = re_parser(r"p=<(.*)>, v=<(.*)>, a=<(.*)>");
    input
        .lines()
        .enumerate()
        .min_by_key(|&(_, line)| {
            let (sp, sv, sa): (&str, &str, &str) = parse(line);
            let p: Vec<i64> = sp.split(',').gather();
            let v: Vec<i64> = sv.split(',').gather();
            let a: Vec<i64> = sa.split(',').gather();
            (0..3).fold((0, 0, 0), |p1, i| add(p1, sig(p[i], v[i], a[i])))
        })
        .unwrap()
        .0
}

pub fn part2(input: &str) -> usize {
    let mut p: Vec<Vec<i64>> = vec![];
    let mut v: Vec<Vec<i64>> = vec![];
    let mut a: Vec<Vec<i64>> = vec![];
    let parse = re_parser(r"p=<(.*)>, v=<(.*)>, a=<(.*)>");
    for line in input.lines() {
        let (sp, sv, sa): (&str, &str, &str) = parse(line);
        p.push(sp.split(',').gather());
        v.push(sv.split(',').gather());
        a.push(sa.split(',').gather());
    }
    let n = p.len();
    let mut coll_map: HashMap<i64, Vec<usize>> = HashMap::new();
    for i in 0..n {
        for j in i + 1..n {
            let colls: Vec<_> = (0..3)
                .filter_map(|k| collisions((p[i][k], v[i][k], a[i][k]), (p[j][k], v[j][k], a[j][k])))
                .collect();
            for &x in &colls[0] {
                if x >= 0 && (1..colls.len()).all(|k| colls[k].contains(&x)) {
                    coll_map.entry(x).or_default().extend([i, j]);
                }
            }
        }
    }
    let mut alive: HashSet<usize> = (0..n).collect();
    let mut times: Vec<_> = coll_map.keys().copied().collect();
    times.sort_unstable();
    for t in times {
        let boom: HashSet<_> = coll_map[&t]
            .iter()
            .filter(|&&j| alive.contains(&j))
            .collect();
        if boom.len() >= 2 {
            for b in boom {
                alive.remove(b);
            }
        }
    }
    alive.len()
}

type P3 = (i64, i64, i64);
fn sig(mut p: i64, mut v: i64, mut a: i64) -> P3 {
    if a < 0 {
        a = -a;
        v = -v;
        p = -p;
    }
    (a, a + 2 * v, p)
}

fn add((a1, a2, a3): P3, (b1, b2, b3): P3) -> P3 {
    (a1 + b1, a2 + b2, a3 + b3)
}

fn collisions((p1, v1, a1): P3, (p2, v2, a2): P3) -> Option<Vec<i64>> {
    let a = a1 - a2;
    let b = a + 2 * (v1 - v2);
    let c = 2 * (p1 - p2);
    quad_roots(a, b, c)
}

fn quad_roots(a: i64, b: i64, c: i64) -> Option<Vec<i64>> {
    match (a, b, c) {
        (0, 0, 0) => None,
        (0, 0, _) => Some(vec![]),
        (0, _, _) => {
            if c % b == 0 {
                Some(vec![-c / b])
            } else {
                Some(vec![])
            }
        }
        _ => Some(isqrt(b * b - 4 * a * c).map_or_else(Vec::new, |d| {
            [(-b - d, 2 * a), (-b + d, 2 * a)]
                .into_iter()
                .filter_map(|(x, y)| if x % y == 0 { Some(x / y) } else { None })
                .collect()
        })),
    }
}

fn isqrt(x: i64) -> Option<i64> {
    if x >= 0 {
        Some((x as f64).sqrt() as i64)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 170);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 571);
    }
    #[test]
    fn test_quad_roots() {
        assert_eq!(quad_roots(0, 0, 0), None);
        assert_eq!(quad_roots(0, 3, 6), Some(vec![-2]));
        assert_eq!(quad_roots(0, 3, 4), Some(vec![]));
        assert_eq!(quad_roots(1, 3, 4), Some(vec![]));
        assert_eq!(quad_roots(1, 2, 1), Some(vec![-1, -1]));
        assert_eq!(quad_roots(1, -3, 2), Some(vec![1, 2]));
        assert_eq!(quad_roots(-20, 60, -40), Some(vec![2, 1]));
    }
}
