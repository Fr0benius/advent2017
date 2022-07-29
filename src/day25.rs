use std::collections::HashMap;

use crate::util::parsing::Gather;

pub const INPUT: &str = include_str!("../input/25.txt");

pub fn part1(input: &str) -> usize {
    let blobs: Vec<&str> = input.split("\n\n").collect();
    let (mut state, times) = {
        let mut lines = blobs[0].lines();
        let state = state_idx(last_item(lines.next().unwrap()));
        let times: usize = lines
            .next()
            .unwrap()
            .strip_prefix("Perform a diagnostic checksum after ")
            .unwrap()
            .split_whitespace()
            .gather();
        (state, times)
    };
    let mut tran: Vec<[(bool, i64, usize); 2]> = vec![Default::default(); blobs.len() - 1];
    for &blob in &blobs[1..] {
        let mut lines = blob.lines();
        let state = state_idx(last_item(lines.next().unwrap()));
        for _ in 0..2 {
            let items: Vec<&str> = (0..4).map(|_| last_item(lines.next().unwrap())).collect();
            let cur_bit = bit(items[0]);
            let next_bit = bit(items[1]);
            let dir = if items[2] == "right" { 1 } else { -1 };
            let next_state = state_idx(items[3]);
            tran[state][cur_bit as usize] = (next_bit, dir, next_state);
        }
    }
    let mut tape: HashMap<i64, bool> = HashMap::new();
    let mut pos = 0;
    for _ in 0..times {
        let &bit = tape.get(&pos).unwrap_or(&false);
        let (next_bit, dir, next_state) = tran[state][bit as usize];
        tape.insert(pos, next_bit);
        pos += dir;
        state = next_state;
    }
    tape.into_values().filter(|&b| b).count()
}

fn last_item(s: &str) -> &str {
    s[..s.len() - 1].split_whitespace().last().unwrap()
}

fn state_idx(s: &str) -> usize {
    assert_eq!(s.len(), 1);
    (s.as_bytes()[0] - b'A') as usize
}
fn bit(s: &str) -> bool {
    assert_eq!(s.len(), 1);
    (s.as_bytes()[0] - b'0') == 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 0);
    }
}
