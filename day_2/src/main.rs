use aoc::get_file_content_or_exit;

#[derive(PartialEq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

fn get_shape_value(shape: &Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    }
}

#[derive(Debug)]
enum MatchResult {
    Lose,
    Draw,
    Win,
}

fn get_result_value(result: &MatchResult) -> i32 {
    match result {
        MatchResult::Lose => 0,
        MatchResult::Draw => 3,
        MatchResult::Win => 6,
    }
}

fn parse_input(input: &char) -> Option<Shape> {
    match input {
        'A' | 'X' => Some(Shape::Rock),
        'B' | 'Y' => Some(Shape::Paper),
        'C' | 'Z' => Some(Shape::Scissor),
        _ => None,
    }
}

fn check_result(opponent: &Shape, you: &Shape) -> MatchResult {
    if opponent == you {
        MatchResult::Draw
    } else if provoke_result(opponent, MatchResult::Lose) == you {
        MatchResult::Lose
    } else {
        MatchResult::Win
    }
}

fn parse_required_result(result: &char) -> Option<MatchResult> {
    match result {
        'X' => Some(MatchResult::Lose),
        'Y' => Some(MatchResult::Draw),
        'Z' => Some(MatchResult::Win),
        _ => None,
    }
}

fn provoke_result(opponent: &Shape, result: MatchResult) -> &Shape {
    match result {
        MatchResult::Draw => opponent,
        MatchResult::Lose => match *opponent {
            Shape::Rock => &Shape::Scissor,
            Shape::Paper => &Shape::Rock,
            Shape::Scissor => &Shape::Paper,
        },
        MatchResult::Win => match *opponent {
            Shape::Rock => &Shape::Paper,
            Shape::Paper => &Shape::Scissor,
            Shape::Scissor => &Shape::Rock,
        },
    }
}

fn main() {
    let data = get_file_content_or_exit();
    let cleaned = data.replace(' ', "");

    // Part 1
    let total: i32 = cleaned
        .split('\n')
        .map(|line| {
            if line.len() > 0 {
                let mut chars = line.chars();
                let opponent = parse_input(&chars.next().unwrap()).unwrap();
                let you = parse_input(&chars.next().unwrap()).unwrap();
                return get_result_value(&check_result(&opponent, &you)) + get_shape_value(&you);
            }
            0
        })
        .sum();

    println!("Total points: {}", total);

    let provoked_total: i32 = cleaned
        .split('\n')
        .map(|line| {
            let mut chars = line.chars();
            if line.len() > 0 {
                let opponent = parse_input(&chars.next().unwrap()).unwrap();
                let desired_result = parse_required_result(&chars.next().unwrap()).unwrap();
                
                let shape = provoke_result(&opponent, desired_result);
                let match_result = check_result(&opponent, shape);

                get_result_value(&match_result) + get_shape_value(shape)
            } else {0}
        })
        .sum();

    println!("Provoked point total: {}", provoked_total);
}
