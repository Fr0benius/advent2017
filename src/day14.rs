use std::ops::BitXor;

use crate::util::{arr2::Arr2, dsu::DSU};

pub const INPUT: &str = include_str!("../input/14.txt");

const N: usize = 128;
pub fn part1(input: &str) -> usize {
    (0..N)
        .map(|i| {
            hash_binary_string(&format!("{}-{}", input.trim(), i))
                .bytes()
                .filter(|&b| b == b'1')
                .count()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut g = Arr2::new(N, N, false);
    for i in 0..N {
        for (j, c) in hash_binary_string(&format!("{}-{}", input.trim(), i))
            .bytes()
            .enumerate()
        {
            g[i][j] = c == b'1';
        }
    }
    let mut dsu = DSU::new(N * N);
    for i in 0..N {
        for j in 0..N {
            if !g[i][j] {
                continue;
            }
            let v = i * N + j;
            if i + 1 < N && g[i + 1][j] {
                dsu.join(v, v + N);
            }
            if j + 1 < N && g[i][j + 1] {
                dsu.join(v, v + 1);
            }
        }
    }
    dsu.comps() - g.into_iter().filter(|&x| !x).count()
}

fn hash_binary_string(source: &str) -> String {
    let lens: Vec<usize> = source
        .bytes()
        .chain([17, 31, 73, 47, 23])
        .map(|b| b as usize)
        .collect();
    let n = lens.len();
    let mut v: Vec<u8> = (0..=255).collect();
    let mut pos = 0;
    for (skip, l) in lens.into_iter().cycle().take(64 * n).enumerate() {
        v[0..l].reverse();
        v.rotate_left((l + skip) % 256);
        pos += l + skip;
    }
    v.rotate_right(pos % 256);
    let res: String = v
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, BitXor::bitxor))
        .map(|b| format!("{:08b}", b))
        .collect();
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 8140);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1182);
    }
}
