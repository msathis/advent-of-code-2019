use crate::aoc::utils::Input;
use std::collections::HashSet;

fn get_points(line: Vec<String>) -> HashSet<(i32, i32)>{
    let mut points = HashSet::new();

    let mut x = 0;
    let mut y = 0;
    for inst in line {
        let (op, size) = inst.split_at(1);
        match op {
            "R" => {
                let new_x = x + size.parse::<i32>().unwrap();
                add_points(&mut points, x, y, new_x, y);
                x = new_x;
            },
            "L" => {
                let new_x = x - size.parse::<i32>().unwrap();
                add_points(&mut points, new_x, y, x, y);
                x = new_x;
            },
            "U" => {
                let new_y = y + size.parse::<i32>().unwrap();
                add_points(&mut points, x, y, x, new_y);
                y = new_y;
            },
            "D" => {
                let new_y = y - size.parse::<i32>().unwrap();
                add_points(&mut points, x, new_y, x, y);
                y = new_y;
            },
            _ => panic!()
        }
    }
    points
}

fn add_points(points: &mut HashSet<(i32, i32)>, x_start: i32, y_start: i32, x_end: i32, y_end: i32) -> &mut HashSet<(i32, i32)> {

    if x_start == x_end {
        for y in y_start..y_end + 1 {
            points.insert((x_start, y));
        }
    } else {
        for x in x_start..x_end + 1 {
            points.insert((x, y_start));
        }
    }

    points
}

pub async fn part1() -> String {
    let input = Input {
        path: "src/aoc/day3/input.txt".to_string()
    };
    let lines = input.read().await;
    let mut line1: Vec<String> = lines.first().unwrap().split(",").map(|s| s.to_string()).collect();
    let mut line2: Vec<String> = lines.last().unwrap().split(",").map(|s| s.to_string()).collect();

    let points1 = get_points(line1);
    let points2 = get_points(line2);

    let mut min = i32::max_value();
    for (x, y) in points1.intersection(&points2) {
        let dist = i32::abs(*x) + i32::abs(*y);

        //Lets skip point (0, 0)
        if dist < min && dist > 0 {
            min = dist;
        }
    }
    min.to_string()
}

pub async fn part2() -> String {
    unimplemented!()
}