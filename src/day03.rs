use std::collections::HashMap;

pub const INPUT: &str = include_str!("../input/03.txt");

pub fn part1(input: &str) -> i64 {
    let (mut x, mut y) = (0i64, 0i64);
    let mut z: i64 = input.trim().parse().unwrap();
    let mut try_move = |dx, dy| {
        if z == 1 {
            false
        } else {
            z -= 1;
            x += dx;
            y += dy;
            true
        }
    };
    'outer: for k in 0.. {
        let rad = 2 * k + 1;
        for _ in 0..rad {
            if !try_move(1, 0) {
                break 'outer;
            }
        }
        for _ in 0..rad {
            if !try_move(0, 1) {
                break 'outer;
            }
        }
        for _ in 0..rad + 1 {
            if !try_move(-1, 0) {
                break 'outer;
            }
        }
        for _ in 0..rad + 1 {
            if !try_move(0, -1) {
                break 'outer;
            }
        }
    }
    x.abs() + y.abs()
}

pub fn part2(input: &str) -> i64 {
    let (mut x, mut y) = (0i64, 0i64);
    let z: i64 = input.trim().parse().unwrap();
    let mut last_item = 0;
    let mut map = HashMap::from([((0, 0), 1)]);
    let mut do_move = |dx, dy| {
        x += dx;
        y += dy;
        last_item = 0;
        for x1 in x - 1..=x + 1 {
            for y1 in y - 1..=y + 1 {
                if (x, y) == (x1, y1) {
                    continue;
                }
                last_item += map.get(&(x1, y1)).unwrap_or(&0);
            }
        }
        map.insert((x, y), last_item);
        last_item <= z
    };
    'outer: for k in 0.. {
        let rad = 2 * k + 1;
        for _ in 0..rad {
            if !do_move(1, 0) {
                break 'outer;
            }
        }
        for _ in 0..rad {
            if !do_move(0, 1) {
                break 'outer;
            }
        }
        for _ in 0..rad + 1 {
            if !do_move(-1, 0) {
                break 'outer;
            }
        }
        for _ in 0..rad + 1 {
            if !do_move(0, -1) {
                break 'outer;
            }
        }
    }
    last_item
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 371);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 369_601);
    }
}
