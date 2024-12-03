use std::ops::RangeInclusive;

const INPUT: &str = include_str!("./input.txt");

fn is_safe(values: Vec<u32>) -> bool {
    #[derive(Clone, Copy)]
    enum State {
        Increasing,
        Decreasing,
    }

    const SAFE_BOUNDS: RangeInclusive<u32> = 1..=3;

    let mut state = None;

    values.windows(2).all(|values| {
        let [a, b] = *values else { unreachable!() };
        match state {
            Some(State::Increasing) | None if a < b && SAFE_BOUNDS.contains(&(b - a)) => {
                state = Some(State::Increasing);
                true
            }

            Some(State::Decreasing) | None if a > b && SAFE_BOUNDS.contains(&(a - b)) => {
                state = Some(State::Decreasing);
                true
            }

            _ => false,
        }
    })
}

fn problem_1() {
    let answer = INPUT
        .lines()
        .filter(|line| {
            let values = line
                .split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect::<Vec<u32>>();
            is_safe(values)
        })
        .count();
    println!("problem1 = {answer}");
}

fn problem_2() {
    let answer = INPUT
        .lines()
        .filter(|line| {
            let values = line
                .split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect::<Vec<u32>>();

            (0..values.len()).any(|i| {
                let mut values = values.clone();
                values.remove(i);
                is_safe(values)
            })
        })
        .count();
    println!("problem2 = {answer}");
}

fn main() {
    problem_1();
    problem_2();
}
