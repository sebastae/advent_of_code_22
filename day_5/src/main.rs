use aoc::get_file_content_or_exit;

fn parse_init_line(line: &str) -> Vec<Option<String>> {
    let mut items: Vec<Option<String>> = Vec::new();
    let chunks = line.as_bytes().chunks(4);

    chunks.for_each(|c| {
        let name = (c[1] as char).to_string();
        if name.ne(" ") {
            items.push(Some(name));
        } else {
            items.push(None);
        }
    });

    items
}

fn make_stacks(state: &str) -> Vec<String> {
    let labels = state.split('\n').last().expect("label line");
    let num_stacks: usize = labels.trim().split(' ').filter(|l| l.len() != 0).count();

    let mut stacks = Vec::new();

    for _ in 0..num_stacks {
        stacks.push(String::from(""));
    }

    let initial_state = state
        .split('\n')
        .take(state.split('\n').count() - 1)
        .collect::<Vec<&str>>();
    initial_state.iter().rev().for_each(|line| {
        parse_init_line(*line)
            .iter()
            .enumerate()
            .for_each(|(i, opt_crate)| {
                if let Some(crt) = opt_crate {
                    if let Some(stack) = stacks.get_mut(i) {
                        stack.push_str(crt);
                    }
                }
            });
    });

    stacks
}

struct Operation {
    number: usize,
    from: usize,
    to: usize,
}

fn parse_instruction_line(line: &str) -> Option<Operation> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 6 {
        return None;
    }

    if let Ok(number) = fields.get(1).unwrap().parse() {
        if let Ok(from) = fields.get(3).unwrap().parse::<usize>() {
            if let Ok(to) = fields.get(5).unwrap().parse::<usize>() {
                return Some(Operation {
                    number,
                    from: from - 1,
                    to: to - 1,
                });
            }
        }
    }

    None
}

fn apply_operation(stacks: &mut Vec<String>, op: &Operation) {
    for _ in 0..op.number {
        let item = if let Some(from) = stacks.get_mut(op.from) {
            from.pop()
        } else {
            None
        };

        if let Some(item) = item {
            if let Some(to) = stacks.get_mut(op.to) {
                to.push(item);
            }
        }
    }
}

fn apply_take_operation(stacks: &mut Vec<String>, op: &Operation) {
    let mut tmp_str = String::from("");

    for _ in 0..op.number {
        if let Some(from) = stacks.get_mut(op.from) {
            if let Some(ch) = from.pop() {
                tmp_str.push(ch);
            }
        }
    }

    if let Some(to) = stacks.get_mut(op.to) {
        to.push_str(&tmp_str.chars().rev().collect::<String>());
    }
}

fn get_top_layer(stacks: &Vec<String>) -> String {
    stacks
        .iter()
        .filter(|s| s.len() > 0)
        .map(|s| s.get((s.len() - 1)..s.len()).unwrap().to_owned())
        .collect::<Vec<String>>()
        .join("")
}

fn main() {
    let data = get_file_content_or_exit().replace('\r', "");
    let sections: Vec<&str> = data.split("\n\n").collect();

    let initial_state_str = sections.get(0).expect("initial state").to_string();
    let rearrangement_procedure = sections
        .get(1)
        .expect("rearrangement procedure")
        .to_string();

    // Part 1

    let mut stacks = make_stacks(&initial_state_str);

    let instructions = rearrangement_procedure
        .split('\n')
        .filter(|line| line.len() != 0)
        .map(|line| parse_instruction_line(line))
        .collect::<Vec<Option<Operation>>>();

    instructions.iter().for_each(|operation| {
        if let Some(op) = operation {
            apply_operation(&mut stacks, op);
        }
    });

    println!("Top layer: {}", get_top_layer(&stacks));

    // Part 2

    let mut p2_stacks = make_stacks(&initial_state_str);

    instructions.iter().for_each(|operation| {
        if let Some(op) = operation {
            apply_take_operation(&mut p2_stacks, op);
        }
    });

    println!("Top layer p2: {}", get_top_layer(&p2_stacks));
}
