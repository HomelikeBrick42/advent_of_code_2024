const INPUT: &str = include_str!("./input.txt");

fn problem_1() {
    let (mut left, mut right): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|line| {
            let mut numbers = line.split_whitespace();
            let a: u32 = numbers.next().unwrap().parse().unwrap();
            let b: u32 = numbers.next().unwrap().parse().unwrap();
            (a, b)
        })
        .unzip();
    left.sort();
    right.sort();
    let answer: u32 = left
        .into_iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(b))
        .sum();
    println!("problem1 = {answer}");
}

fn problem_2() {
    let (left, right): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|line| {
            let mut numbers = line.split_whitespace();
            let a: u32 = numbers.next().unwrap().parse().unwrap();
            let b: u32 = numbers.next().unwrap().parse().unwrap();
            (a, b)
        })
        .unzip();
    let answer: u32 = left
        .into_iter()
        .map(|x| {
            let times = right.iter().filter(|&&y| x == y).count();
            x * times as u32
        })
        .sum();
    println!("problem2 = {answer}");
}

fn main() {
    problem_1();
    problem_2();
}
