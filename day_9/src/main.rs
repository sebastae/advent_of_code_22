use std::collections::HashSet;

type Vec2 = (i32, i32);

fn add_vec((ax, ay): Vec2, (bx, by): Vec2) -> Vec2 {
    (ax + bx, ay + by)
}

fn max_vec(a: Vec2, b: Vec2) -> Vec2 {
    if (a.0 * a.0 + a.1 * a.1) > (b.0 * b.0 + b.1 * b.1) {
        a
    } else {
        b
    }
}

fn sign(n: i32) -> i32 {
    if n == 0 {
        0
    } else {
        n.signum()
    }
}

fn normalize((x, y): Vec2) -> Vec2 {
    (sign(x), sign(y))
}

fn parse_line(line: &str) -> Option<Vec2> {
    let mut tokens = line.trim().split_ascii_whitespace();
    if let Some(dir) = tokens.next() {
        if let Some(length_str) = tokens.next() {
            if let Ok(length) = length_str.parse::<i32>() {
                return match dir {
                    "U" => Some((0, -length)),
                    "D" => Some((0, length)),
                    "L" => Some((-length, 0)),
                    "R" => Some((length, 0)),
                    _ => None,
                };
            }
        }
    }

    None
}

fn follow(v: &mut Vec2, t: &Vec2) {
    let dx = t.0 - v.0;
    let dy = t.1 - v.1;

    if f32::sqrt((dx * dx + dy * dy) as f32) > 1.5 {
        // length of diagonal adjacent (1,1) is 1.41...
        let (ax, ay) = max_vec((dx, 0), (0, dy));
        *v = add_vec(*t, (-sign(ax), -sign(ay)));
    }
}

fn move_rope_head(rope: &mut Vec<Vec2>, dir: Vec2) {
    if rope.len() > 0 {
        rope[0] = add_vec(rope[0], dir);
        for i in 1..rope.len() {
            let target = rope.get(i - 1).unwrap().to_owned();
            follow(rope.get_mut(i).unwrap(), &target);
        }
    }
}

fn find_unique_positions(input: &str, rope_len: usize) -> usize {
    let mut pos: HashSet<Vec2> = HashSet::new();

    if rope_len == 0 {
        return 0;
    }

    let mut rope = vec![(0, 0); rope_len];

    pos.insert(rope.last().unwrap().to_owned());

    input.trim().lines().for_each(|line| {
        if let Some(dir) = parse_line(line) {
            for _ in 0..dir.0.abs().max(dir.1.abs()) {
                move_rope_head(&mut rope, normalize(dir));
                let tail = rope.last().unwrap().to_owned();
                pos.insert(tail);
            }
        }
    });

    pos.len()
}

fn main() {
    let input = include_str!("./input.txt");
    println!(
        "Unique tail positions, 2 segments: {}",
        find_unique_positions(input, 2)
    );

    // Gives correct answer for both tests and part 1 with full input, but too high for part 2... >:(
    println!(
        "Unique tail positions, 10 segments: {}",
        find_unique_positions(input, 10)
    );
}

#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn p1_input() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
        assert_eq!(find_unique_positions(input, 2), 13);
    }

    #[test]
    fn p2_input() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        assert_eq!(find_unique_positions(input, 10), 36);
    }
}
