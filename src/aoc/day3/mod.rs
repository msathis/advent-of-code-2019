use crate::aoc::utils::Input;
use std::collections::HashSet;

fn get_points(line: Vec<String>) -> Vec<(i32, i32)> {
    let mut points = vec![];

    let mut x = 0;
    let mut y = 0;
    for inst in line {
        let (op, size) = inst.split_at(1);
        match op {
            "R" => {
                let new_x = x + size.parse::<i32>().unwrap();
                add_points(&mut points, x + 1, y, new_x, y);
                x = new_x;
            }
            "L" => {
                let new_x = x - size.parse::<i32>().unwrap();
                add_points(&mut points, x - 1, y, new_x, y);
                x = new_x;
            }
            "U" => {
                let new_y = y + size.parse::<i32>().unwrap();
                add_points(&mut points, x, y + 1, x, new_y);
                y = new_y;
            }
            "D" => {
                let new_y = y - size.parse::<i32>().unwrap();
                add_points(&mut points, x, y - 1, x, new_y);
                y = new_y;
            }
            _ => panic!()
        }
    }
    points
}

fn add_points(points: &mut Vec<(i32, i32)>, x_start: i32, y_start: i32, x_end: i32, y_end: i32) -> &mut Vec<(i32, i32)> {
    if x_start == x_end {
        let step = if y_start > y_end { -1 } else { 1 };
        let mut y = y_start;

        while y != y_end {
            points.push((x_start, y));
            y = y + step;
        }
        points.push((x_start, y));
    } else {
        let step = if x_start > x_end { -1 } else { 1 };
        let mut x = x_start;

        while x != x_end {
            points.push((x, y_start));
            x = x + step;
        }
        points.push((x, y_start));
    }

    points
}

pub async fn part1() -> String {
    let input = Input {
        path: "src/aoc/day3/input.txt".to_string()
    };
    let lines = input.read().await;
    let line1: Vec<String> = lines.first().unwrap().split(",").map(|s| s.to_string()).collect();
    let line2: Vec<String> = lines.last().unwrap().split(",").map(|s| s.to_string()).collect();

    let points1 = get_points(line1);
    let points2 = get_points(line2);

    let set1: HashSet<(i32, i32)> = points1.iter().cloned().collect();
    let set2: HashSet<(i32, i32)> = points2.iter().cloned().collect();

    let mut min = i32::max_value();
    for (x, y) in set1.intersection(&set2) {
        let dist = i32::abs(*x) + i32::abs(*y);

        //Lets skip point (0, 0)
        if dist < min && dist > 0 {
            min = dist;
        }
    }
    min.to_string()
}

pub async fn part2() -> String {
    let input = Input {
        path: "src/aoc/day3/input.txt".to_string()
    };
    let lines = input.read().await;
    let line1: Vec<String> = lines.first().unwrap().split(",").map(|s| s.to_string()).collect();
    let line2: Vec<String> = lines.last().unwrap().split(",").map(|s| s.to_string()).collect();

    let points1 = get_points(line1);
    let points2 = get_points(line2);

    let set1: HashSet<(i32, i32)> = points1.iter().cloned().collect();
    let set2: HashSet<(i32, i32)> = points2.iter().cloned().collect();

    let mut min = usize::max_value();
    for (x, y) in set1.intersection(&set2) {
        let dist = points1.iter().position(|(x2, y2)| x == x2 && y == y2).unwrap() +
            points2.iter().position(|(x2, y2)| x == x2 && y == y2).unwrap() + 2;

        //Lets skip point (0, 0)
        if dist < min && dist > 0 {
            min = dist;
        }
    }
    min.to_string()
}