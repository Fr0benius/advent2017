use std::collections::{HashMap, HashSet};

use crate::util::{
    counter::Counter,
    parsing::{Gather, re_parser},
    recursive_function::{Callable, RecursiveFunction},
};

pub const INPUT: &str = include_str!("../input/07.txt");

pub fn part1(input: &str) -> String {
    let mut all = HashSet::new();
    let mut ch = HashSet::new();
    for line in input.lines() {
        let data: Vec<&str> = line.split(" -> ").gather();
        all.insert(data[0].split_whitespace().next().unwrap());
        if data.len() == 2 {
            for x in data[1].split(", ") {
                all.insert(x);
                ch.insert(x);
            }
        }
    }
    for x in all {
        if !ch.contains(&x) {
            return x.into();
        }
    }
    unreachable!()
}

pub fn part2(input: &str) -> i64 {
    let mut g = HashMap::new();
    let mut par = HashMap::new();
    let mut wt = HashMap::new();
    let parser = re_parser(r"(.*) \((.*)\)");
    for line in input.lines() {
        let data: Vec<&str> = line.split(" -> ").gather();
        let (v, weight): (&str, i64) = parser(data[0]);
        wt.insert(v, weight);
        let ch = g.entry(v).or_insert(vec![]);
        if data.len() == 2 {
            for x in data[1].split(", ") {
                ch.push(x);
                par.insert(x, v);
            }
        }
    }
    let root = {
        let mut root = "";
        for &v in g.keys() {
            if !par.contains_key(&v) {
                root = v;
                break;
            }
        }
        root
    };
    let mut sub = HashMap::new();
    {
        let mut compute_subtrees = RecursiveFunction::new(|f, v: &str| {
            let mut sum = wt[&v];
            for &w in &g[v] {
                f.call(w);
                sum += sub[&w];
            }
            sub.insert(v, sum);
        });
        compute_subtrees.call(root);
    }
    let mut node = root;
    loop {
        match find_unbalanced(&g, &sub, node) {
            (1, Some(nxt)) => {
                node = nxt;
            },
            (2, None) => {
                unreachable!()
            },
            (0, None) => {
                let p = par[&node];
                let target = g[&p].iter()
                    .map(|&w| sub[w])
                    .find(|&x| x != sub[node])
                    .unwrap();
                return target - sub[&node] + wt[&node];
            },
            _ => unreachable!()
        }
    }
}

fn find_unbalanced<'a>(
    g: &'a HashMap<&str, Vec<&'a str>>,
    sub: &'a HashMap<&'a str, i64>,
    v: &'a str,
) -> (usize, Option<&'a str>) {
    let cnt = g[v].iter().map(|w| sub[w]).counter();
    assert!(cnt.len() <= 2);
    if cnt.len() == 1 {
        return (0, None);
    }
    if cnt.values().all(|&c| c == 1) {
        println!("Ambiguity!");
        return (2, None);
    }
    if let Some((sz, _)) = cnt.into_iter().find(|&(_, k)| k == 1) {
        let w = *g[v].iter().find(|&w| sub[w] == sz).unwrap();
        return (1, Some(w));
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "aapssr");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1458);
    }
}
