fn char_to_index(c: &char) -> u8 {
    match c {
        'A'..='Z' => *c as u8 - 'A' as u8 + 1,
        'a'..='z' => *c as u8 - 'a' as u8 + 27,
        _ => 0,
    }
}

fn is_all_unique(chars: &str) -> bool {
    let mut cmp: u64 = 0;
    for c in chars.chars() {
        if cmp & (1 << char_to_index(&c)) != 0 {
            return false;
        }

        cmp |= 1 << char_to_index(&c);
    }

    true
}

fn find_first_n_unique(input: &str, n_unique: usize) -> Option<usize> {
    if input.len() < n_unique {
        return None;
    }

    for i in 0..=(input.len() - n_unique) {
        if is_all_unique(&input[i..(i + n_unique)]) {
            return Some(i + n_unique);
        }
    }

    None
}

fn main() {
    let data = include_str!("./input.txt");

    // Part 1
    if let Some(n) = find_first_n_unique(data, 4) {
        println!("First occurence of 4 unique: {}", n);
    }

    if let Some(n) = find_first_n_unique(data, 14) {
        println!("First occurrence of 14 unique: {}", n);
    }
}

#[cfg(test)]
mod tests {
    use crate::find_first_n_unique;

    #[test]
    fn ex_inputs_4_unique() {
        assert_eq!(find_first_n_unique("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4),Some(7));
        assert_eq!(find_first_n_unique("bvwbjplbgvbhsrlpgdmjqwftvncz", 4),Some(5));
        assert_eq!(find_first_n_unique("nppdvjthqldpwncqszvftbrmjlhg", 4),Some(6));
        assert_eq!(find_first_n_unique("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),Some(10));
        assert_eq!(find_first_n_unique("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),Some(11));
    }

    #[test]
    fn ex_inputs_14_unique() {
        assert_eq!(find_first_n_unique("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),Some(19));
        assert_eq!(find_first_n_unique("bvwbjplbgvbhsrlpgdmjqwftvncz", 14),Some(23));
        assert_eq!(find_first_n_unique("nppdvjthqldpwncqszvftbrmjlhg", 14),Some(23));
        assert_eq!(find_first_n_unique("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),Some(29));
        assert_eq!(find_first_n_unique("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),Some(26));
    }
}
