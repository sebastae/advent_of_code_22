
use aoc::get_file_content_or_exit;

fn sum_priorities(types: u64) -> i32 {
    let mut sum = 0;
    for i in 1..=52 {
        sum += ((types & (1 << i)) != 0) as i32 * i;
    }
    sum
}

fn get_priority(c: &char) -> u8 {
    match c {
        'a'..='z' => *c as u8 - 'a' as u8 + 1,
        'A'..='Z' => *c as u8 - 'A' as u8 + 27,
        _ => 0,
    }
}

fn find_types(types: &str) -> u64 {
    let mut counter: u64 = 0;

    types.chars().for_each(|c| {
        let pri = get_priority(&c);
        counter |= 1 << pri;
    });

    counter
}

fn main() {
    let data = get_file_content_or_exit();

    // Part 1
    let sum_types = data
        .split('\n')
        .map(|line| {
            let comp_len = line.len() / 2;
            let comp = (&line[..comp_len], &line[comp_len..]);

            find_types(comp.0) & find_types(comp.1)
        })
        .fold(0, |acc, val| acc + sum_priorities(val));

    println!("Sum of priorities: {}", sum_types);

    // Part 2
    let mut badges: Vec<u64> = vec![];
    data.split('\n').enumerate().for_each(|(i, line)| {
        if badges.len() < (i / 3) + 1 {
            badges.push(find_types(line));
        }

        if let Some(t) = badges.get_mut(i / 3) {
            *t &= find_types(line);
        }
    });

    let sum = badges.iter().fold(0, |acc, val| acc + sum_priorities(*val));

    println!("Sum badge priorities: {}", sum);
}
