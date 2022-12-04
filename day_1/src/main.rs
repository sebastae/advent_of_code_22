use aoc::get_file_content_or_exit;

fn main() {
    let data = get_file_content_or_exit();
    let cleaned = data.replace('\r', "");

    // Part 1
    let mut elves: Vec<i32> = cleaned
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|line| line.parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        })
        .collect();

    if let Some(n) = elves.iter().max() {
        println!("Max calories: {}", n);
    }

    // Part 2
    elves.sort();
    println!("Sum of top 3: {}", elves.iter().rev().take(3).sum::<i32>());
}
