use aoc::get_file_content_or_exit;
use std::cmp;

#[derive(Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}

struct Pair {
    left: Range,
    right: Range,
}

enum Intersection {
    Disjoint,
    Partial(i32, i32),
    Contain(i32, i32),
}

fn compute_intersection(pair: &Pair) -> Intersection {
    if pair.left.start < pair.right.start {
        if pair.left.end < pair.right.start {
            Intersection::Disjoint
        } else if pair.left.end >= pair.right.end {
            Intersection::Contain(pair.right.start, pair.right.end)
        } else {
            Intersection::Partial(pair.right.start, pair.left.end)
        }
    } else if pair.right.start < pair.left.start {
        if pair.right.end < pair.left.start {
            Intersection::Disjoint
        } else if pair.right.end >= pair.left.end {
            Intersection::Contain(pair.left.start, pair.left.end)
        } else {
            Intersection::Partial(pair.left.start, pair.right.end)
        }
    } else {
        Intersection::Contain(pair.left.start, cmp::min(pair.left.end, pair.right.end))
    }
}

fn main() {
    let data = get_file_content_or_exit();
    let cleaned = data.replace('\r', "");
    let pairs = cleaned.split('\n').map(|line| {
        if line.len() == 0 {
            return None;
        }
        let pair: Vec<Range> = line
            .split(',')
            .map(|range| {
                let mut sides = range.split('-').map(|num| num.parse::<i32>());
                let start = sides.next().unwrap_or(Ok(0)).unwrap_or(0);
                let end = sides.next().unwrap_or(Ok(0)).unwrap_or(0);

                Range { start, end }
            })
            .collect();

        Some(Pair {
            left: pair.get(0).unwrap().to_owned(),
            right: pair.get(1).unwrap().to_owned(),
        })
    });

    // Part 1
    let total_full_intersections = pairs
        .clone()
        .filter(|p| {
            if let Some(pair) = p {
                if let Intersection::Contain(..) = compute_intersection(pair) {
                    return true;
                }
            }
            false
        })
        .count();

    println!("Total complete intersections: {}", total_full_intersections);

    // Part 2
    let total_intersections = pairs
        .clone()
        .filter(|p| {
            if let Some(pair) = p {
                match compute_intersection(pair) {
                    Intersection::Contain(..) | Intersection::Partial(..) => true,
                    _ => false,
                }
            } else {
                false
            }
        })
        .count();

    println!("Total intersections: {}", total_intersections);
}
