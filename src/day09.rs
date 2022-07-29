pub const INPUT: &str = include_str!("../input/09.txt");

pub fn part1(input: &str) -> i64 {
    score(&parse_item(&mut input.trim().bytes().peekable()), 1)
}

pub fn part2(input: &str) -> i64 {
    count_garbage(&parse_item(&mut input.trim().bytes().peekable()))
}


#[derive(Debug)]
enum Item {
    Group(Vec<Item>),
    Garbage(Vec<u8>),
}
use std::iter::Peekable;

use Item::*;

fn score(item: &Item, k: i64) -> i64 {
    match item {
        Group(v) => k + v.iter().map(|x| score(x, k + 1)).sum::<i64>(),
        Garbage(_) => 0,
    }
}

fn count_garbage(item: &Item) -> i64 {
    match item {
        Group(v) => v.iter().map(count_garbage).sum::<i64>(),
        Garbage(g) => g.len() as i64,
    }
}
fn parse_item<Iter: Iterator<Item = u8>>(iter: &mut Peekable<Iter>) -> Item {
    let c = iter.next().unwrap();
    match c {
        b'{' => {
            if iter.peek() == Some(&b'}') {
                iter.next();
                return Group(vec![]);
            }
            let mut res = vec![];
            loop {
                res.push(parse_item(iter));
                if iter.next() == Some(b'}') {
                    break;
                }
            }
            Group(res)
        }
        b'<' => {
            let mut res = vec![];
            while let Some(g) = iter.next() {
                match g {
                    b'!' => {
                        iter.next();
                    }
                    b'>' => break,
                    _ => res.push(g),
                }
            }
            Garbage(res)
        }
        _ =>  unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 11089);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 5288);
    }
}
