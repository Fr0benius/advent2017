use crate::util::{parsing::Gather, recursive_function::{RecursiveFunction, Callable}};

pub const INPUT: &str = include_str!("../input/24.txt");

pub fn part1(input: &str) -> usize {
    let (g, edges) = read_graph(input);
    let mut seen = vec![false; edges.len()];
    let mut res = 0;
    let mut cur = 0;
    let mut dfs = RecursiveFunction::new(|f, v: usize| {
        res = res.max(cur);
        for &(i, w) in &g[v] {
            if seen[i] {
                continue;
            }
            seen[i] = true;
            cur += v + w;
            f.call(w);
            cur -= v + w;
            seen[i] = false;
        }
    });
    dfs.call(0);
    res
}

pub fn part2(input: &str) -> usize {
    let (g, edges) = read_graph(input);
    let mut seen = vec![false; edges.len()];
    let mut best = (0, 0);
    let mut len = 0;
    let mut cur = 0;
    let mut dfs = RecursiveFunction::new(|f, v: usize| {
        best = best.max((len, cur));
        for &(i, w) in &g[v] {
            if seen[i] {
                continue;
            }
            seen[i] = true;
            cur += v + w;
            len += 1;
            f.call(w);
            len -= 1;
            cur -= v + w;
            seen[i] = false;
        }
    });
    dfs.call(0);
    best.1
}

fn read_graph(input: &str) -> (Vec<Vec<(usize, usize)>>, Vec<(usize, usize)>) {
    let mut max = 0;
    let edges: Vec<_> = input
        .lines()
        .map(|line| {
            let (a, b): (usize, usize) = line.split('/').gather();
            max = max.max(a).max(b);
            (a, b)
        })
        .collect();
    let mut g = vec![vec![]; max + 1];
    for (i, &(a, b)) in edges.iter().enumerate() {
        g[a].push((i, b));
        g[b].push((i, a));
    }
    (g, edges)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1511);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1471);
    }
}
