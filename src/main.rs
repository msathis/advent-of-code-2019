use crate::aoc::utils::Solution;

mod aoc;

#[tokio::main]
async fn main() {
    let result = aoc::day4::part2().await;
    println!("{}", result);
}
