const INPUT: &str = include_str!("./input.txt");

fn problem_1() {
    let answer: u32 = INPUT
        .match_indices("mul(")
        .filter_map(|(index, found)| {
            let mut str = &INPUT[index + found.len()..];

            let a_length = str.chars().take_while(char::is_ascii_digit).count();
            let a_str;
            (a_str, str) = str.split_at(a_length);
            let a: u32 = a_str.parse().ok()?;

            if str.as_bytes()[0] != b',' {
                return None;
            }
            str = &str[1..];

            let b_length = str.chars().take_while(char::is_ascii_digit).count();
            let b_str;
            (b_str, str) = str.split_at(b_length);
            let b: u32 = b_str.parse().ok()?;

            if str.as_bytes()[0] != b')' {
                return None;
            }

            Some(a * b)
        })
        .sum();
    println!("problem1 = {answer}");
}

fn problem_2() {
    let mut answer = 0u32;
    let mut str = INPUT;
    let mut sum_it = true;
    loop {
        if str.is_empty() {
            break;
        }

        if str.starts_with("mul(") {
            str = &str[4..];
            if let Some(value) = (|| {
                let a_length = str.chars().take_while(char::is_ascii_digit).count();
                let a_str;
                (a_str, str) = str.split_at(a_length);
                let a: u32 = a_str.parse().ok()?;

                if str.as_bytes()[0] != b',' {
                    return None;
                }
                str = &str[1..];

                let b_length = str.chars().take_while(char::is_ascii_digit).count();
                let b_str;
                (b_str, str) = str.split_at(b_length);
                let b: u32 = b_str.parse().ok()?;

                if str.as_bytes()[0] != b')' {
                    return None;
                }
                str = &str[1..];

                Some(a * b)
            })() {
                if sum_it {
                    answer += value;
                }
            }
        } else if str.starts_with("do()") {
            str = &str[4..];
            sum_it = true;
        } else if str.starts_with("don't()") {
            str = &str[6..];
            sum_it = false;
        } else {
            str = &str[1..];
        }
    }
    println!("problem2 = {answer}");
}

fn main() {
    problem_1();
    problem_2();
}
