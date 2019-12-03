use crate::aoc::utils::Solution;

mod aoc;

#[tokio::main]
async fn main() {
    let result = aoc::day3::part1().await;
    println!("{}", result);
}
