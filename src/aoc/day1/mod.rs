use crate::aoc::utils::Input;
use async_trait::async_trait;

pub async fn part1() -> String {
    let input = Input {
        path: "src/aoc/day1/input.txt".to_string()
    };
    let lines = input.read().await;
    let sum = lines.iter().map(|s| s.parse::<u32>().unwrap()).map(|u| {
        (u / 3) - 2
    }).sum::<u32>();
    sum.to_string()
}

fn get_recursive_fuel(n: u32) -> u32 {
    if n <= 5 {
        return 0u32;
    }

    let curr_fuel = (n / 3 - 2);
    curr_fuel + get_recursive_fuel(curr_fuel)
}

pub async fn part2() -> String {
    let input = Input {
        path: "src/aoc/day1/input.txt".to_string()
    };
    let lines = input.read().await;
    let sum = lines.iter().map(|s| s.parse::<u32>().unwrap()).map(get_recursive_fuel).sum::<u32>();
    sum.to_string()
}