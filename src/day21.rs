use std::collections::HashMap;

use crate::util::{arr2::Arr2, parsing::Gather};

pub const INPUT: &str = include_str!("../input/21.txt");

type Mat = Arr2<bool>;

pub fn solve(input: &str, times: usize) -> usize {
    let table: HashMap<Mat, Mat> = input
        .lines()
        .flat_map(|line| {
            let (a, b): (&str, &str) = line.split(" => ").gather();
            let mut v = vec![];
            let mut mat_a = from_str(a);
            let mat_b = from_str(b);
            for _ in 0..4 {
                v.push((mat_a.clone(), mat_b.clone()));
                v.push((flip(&mat_a), mat_b.clone()));
                mat_a = rot(&mat_a);
            }
            v
        })
        .collect();
    let mut pic = from_str(".#./..#/###");
    for _ in 0..times {
        let k = if pic.dims().0 % 2 == 0 { 2 } else { 3 };
        let l = if k == 2 { 3 } else { 4 };
        pic = transform(&pic, &table, k, l);
    }
    pic.into_iter().filter(|&x| x).count()
}

pub fn part1(input: &str) -> usize {
    solve(input, 5)
}

pub fn part2(input: &str) -> usize {
    solve(input, 18)
}

fn from_str(s: &str) -> Mat {
    let v: Vec<_> = s.split('/').collect();
    Arr2::from_fn(v.len(), v[0].len(), |i, j| v[i].as_bytes()[j] == b'#')
}

fn rot(a: &Mat) -> Mat {
    let (n, m) = a.dims();
    Arr2::from_fn(m, n, |j, i| a[i][m - 1 - j])
}

fn flip(a: &Mat) -> Mat {
    let (n, m) = a.dims();
    Arr2::from_fn(n, m, |i, j| a[i][m - 1 - j])
}

fn transform(a: &Mat, table: &HashMap<Mat, Mat>, k: usize, l: usize) -> Mat {
    let n = a.dims().0 / k;
    let m = a.dims().1 / k;
    let mut res = Arr2::new(n * l, m * l, false);
    for r in 0..n {
        for c in 0..m {
            let p = Arr2::from_fn(k, k, |i, j| a[r * k + i][c * k + j]);
            let q = &table[&p];
            for i in 0..l {
                for j in 0..l {
                    res[r * l + i][c * l + j] = q[i][j];
                }
            }
        }
    }
    res
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 155);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2_449_665);
    }
}
