use std::collections::{HashMap, VecDeque};

pub const INPUT: &str = include_str!("../input/18.txt");

pub fn part1(input: &str) -> i64 {
    let insts: Vec<&str> = input.lines().collect();
    let mut prog = Program::new(insts, true);
    let mut last_sound = 0;
    loop {
        match prog.step() {
            Normal => continue,
            Snd(s) => last_sound = s,
            Rcv => {
                return last_sound;
            }
            Done => panic!("Program ended without recovering a sound!"),
        }
    }
}

pub fn part2(input: &str) -> i64 {
    let insts: Vec<&str> = input.trim().lines().collect();
    let mut progs = [
        Program::new(insts.clone(), false),
        Program::new(insts, false),
    ];
    for i in [0, 1] {
        progs[i].set("p", i as i64);
    }
    let mut res = 0;
    loop {
        let mut progress = false;
        for i in [0, 1] {
            loop {
                match progs[i].step() {
                    Normal => progress = true,
                    Snd(x) => {
                        progs[1 - i].send(x);
                        if i == 1 {
                            res += 1;
                        }
                        progress = true;
                    }
                    Rcv => break,
                    Done => {
                        break;
                    }
                }
            }
        }
        if !progress {
            break;
        }
    }
    res
}

#[derive(Debug)]
enum Status {
    Normal,
    Snd(i64),
    Rcv,
    Done,
}
use Status::*;

struct Program<'a> {
    reg: HashMap<&'a str, i64>,
    inst: i64,
    insts: Vec<&'a str>,
    sound_mode: bool,
    queue: VecDeque<i64>,
}

impl<'a> Program<'a> {
    fn new(insts: Vec<&'a str>, sound_mode: bool) -> Self {
        Self {
            reg: HashMap::new(),
            inst: 0,
            insts,
            sound_mode,
            queue: VecDeque::new(),
        }
    }
    fn step(&mut self) -> Status {
        if !(0..self.insts.len() as i64).contains(&self.inst) {
            return Done;
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
        let mut res = Normal;
        match op {
            "set" => {
                self.set(x, y_val);
            }
            "add" => {
                self.set(x, x_val + y_val);
            }
            "mul" => {
                self.set(x, x_val * y_val);
            }
            "mod" => {
                assert!(y_val > 0);
                self.set(x, x_val % y_val);
            }
            "jgz" => {
                if x_val > 0 {
                    self.inst += y_val;
                    return Normal;
                }
            }
            "snd" => {
                res = Snd(x_val);
            }
            "rcv" => {
                if self.sound_mode {
                    if x_val != 0 {
                        return Rcv;
                    }
                } else if let Some(s) = self.queue.pop_front() {
                    self.set(x, s);
                } else {
                    return Rcv;
                }
            }
            _ => unreachable!(),
        };
        self.inst += 1;
        res
    }

    fn set(&mut self, x: &'a str, y: i64) {
        self.reg.insert(x, y);
    }

    fn send(&mut self, x: i64) {
        self.queue.push_back(x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1187);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 5969);
    }
}
