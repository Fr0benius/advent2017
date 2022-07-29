use std::collections::HashMap;

pub const INPUT: &str = include_str!("../input/23.txt");

pub fn part1(input: &str) -> usize {
    let mut prog = Program::new(input.lines().collect(), false);
    while prog.step() {}
    prog.muls
}

pub fn part2(input: &str) -> i64 {
    let mut prog = Program::new(input.lines().collect(), true);
    prog.set("a", 1);
    while prog.step() {}
    *prog.reg.get("h").unwrap()
}

struct Program<'a> {
    reg: HashMap<&'a str, i64>,
    inst: i64,
    insts: Vec<&'a str>,
    muls: usize,
    cheating: bool,
}

impl<'a> Program<'a> {
    fn new(insts: Vec<&'a str>, cheating: bool) -> Self {
        Self {
            reg: HashMap::new(),
            inst: 0,
            insts,
            muls: 0,
            cheating,
        }
    }
    fn step(&mut self) -> bool {
        if self.cheating && self.inst == 8 {
            self.set("f", is_prime(self.reg[&"b"]) as i64);
            self.inst = 24;
            return true;
        }
        if !(0..self.insts.len() as i64).contains(&self.inst) {
            return false;
        }
        let mut iter = self.insts[self.inst as usize].split_whitespace();
        let op = iter.next().unwrap();
        let val = |r: &str| {
            r.parse()
                .ok()
                .or_else(|| self.reg.get(&r).copied())
                .unwrap_or(0)
        };
        let x = iter.next().unwrap();
        let x_val = val(x);
        let y = iter.next();
        let y_val: i64 = y.map_or(0, val);
        match op {
            "set" => {
                self.set(x, y_val);
            }
            "sub" => {
                self.set(x, x_val - y_val);
            }
            "mul" => {
                self.set(x, x_val * y_val);
                self.muls += 1;
            }
            "jnz" => {
                if x_val != 0 {
                    self.inst += y_val;
                    return true;
                }
            }
            _ => unreachable!(),
        };
        self.inst += 1;
        true
    }

    fn set(&mut self, x: &'a str, y: i64) {
        self.reg.insert(x, y);
    }
}

fn is_prime(x: i64) -> bool {
    (2..=x).take_while(|&k| k * k <= x).all(|k| x % k != 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 6241);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 909);
    }
}
