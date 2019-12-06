use crate::aoc::utils::Input;
use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;

fn number_to_map(n: i32) -> HashMap<i32, i32> {
    let mut digits: HashMap<i32, i32> = HashMap::new();
    let mut n = n;
    while n > 9 {
        let digit = n % 10;
        if digits.contains_key(&digit) {
            digits.insert(digit, digits.get(&digit).unwrap() + 1);
        } else {
            digits.insert(digit, 1);
        }
        n = n / 10;
    }

    if digits.contains_key(&n) {
        digits.insert(n, digits.get(&n).unwrap() + 1);
    } else {
        digits.insert(n, 1);
    }

    digits
}

pub async fn part1() -> String {
    let start = 146888;
    let end = 612564;
    let mut count = 0;

    for i in start..end {
        let set: HashMap<i32, i32> = number_to_map(i);

        let re = Regex::new(r"^1*2*3*4*5*6*7*8*9*?$").unwrap();

        if set.len() < i.to_string().len() && re.is_match(i.to_string().as_str()) {
            count = count + 1;
        }
    }
    count.to_string()
}

pub async fn part2() -> String {
    let start = 146810;
    let end = 612564;
    let mut count = 0;

    for i in start..=end {
        let set: HashMap<i32, i32> = number_to_map(i);

        let re = Regex::new(r"^1*2*3*4*5*6*7*8*9*?$").unwrap();

        if set.len() < i.to_string().len() && set.iter().any(|(key, val)| val.eq(&2_i32)) && re.is_match(i.to_string().as_str()) {
            count = count + 1;
        }
    }


    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn test_part1() {
        assert_eq!(part1().await, "1748".to_string());
    }

    #[test]
    async fn test_part2() {
        assert_eq!(part2().await, "1180".to_string());
    }
}