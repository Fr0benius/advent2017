use std::collections::HashMap;

use crate::util::parsing::re_parser;

pub const INPUT: &str = include_str!("../input/08.txt");

pub fn solve(input: &str) -> (i64, i64) {
    let parse = re_parser(r"(.*) (.*) (.*) if (.*) (.*) (.*)");
    let mut regs: HashMap<&str, i64> = HashMap::new();
    let mut overall = 0;
    for line in input.lines() {
        let (r1, op, v1, r2, comp, v2): (&str, &str, i64, &str, &str, i64) = parse(line);
        let cond = {
            let r2 = *regs.get(&r2).unwrap_or(&0);
            match comp {
                ">" => r2 > v2,
                "<" => r2 < v2,
                ">=" => r2 >= v2,
                "<=" => r2 <= v2,
                "==" => r2 == v2,
                "!=" => r2 != v2,
                _ => unreachable!(),
            }
        };
        if cond {
            match op {
                "inc" => {
                    *regs.entry(r1).or_default() += v1;
                }
                "dec" => {
                    *regs.entry(r1).or_default() -= v1;
                }
                _ => unreachable!(),
            }
            overall = overall.max(regs[&r1]);
        }
    }
    (regs.into_values().max().unwrap(), overall)
}

pub fn part1(input: &str) -> i64 {
    solve(input).0
}

pub fn part2(input: &str) -> i64 {
    solve(input).1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 6012);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 6369);
    }
}
