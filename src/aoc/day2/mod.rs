use crate::aoc::utils::Input;

fn run_computer(vec: &mut Vec<u32>, noun: u32, verb: u32) -> u32 {
    let mut index = 0;

    std::mem::replace(&mut vec[1], noun);
    std::mem::replace(&mut vec[2], verb);

    while index < vec.len() {
        match vec.get(index).unwrap() {
            1 => {
                let op1: u32 = *vec.get(*vec.get(index + 1 as usize).unwrap() as usize).unwrap() as u32;
                let op2: u32 = *vec.get(*vec.get(index + 2 as usize).unwrap() as usize).unwrap() as u32;

                let dest = vec[index + 3] as usize;
                std::mem::replace(&mut vec[dest], op1 + op2);
                index = index + 4;
            }
            2 => {
                let op1: u32 = *vec.get(*vec.get(index + 1 as usize).unwrap() as usize).unwrap() as u32;
                let op2: u32 = *vec.get(*vec.get(index + 2 as usize).unwrap() as usize).unwrap() as u32;

                let dest = vec[index + 3] as usize;
                std::mem::replace(&mut vec[dest], op1 * op2);
                index = index + 4;
            }
            99 => break,
            _ => panic!("Unexpected opcode")
        }
    }

    *vec.get(0).unwrap()
}

pub async fn part1() -> String {
    let input = Input {
        path: "src/aoc/day2/input.txt".to_string()
    };
    let lines = input.read().await;
    let mut vec: Vec<u32> = lines.first().unwrap().split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let result = run_computer(&mut vec, 12, 2);
    result.to_string()
}

pub async fn part2() -> String {
    let input = Input {
        path: "src/aoc/day2/input.txt".to_string()
    };

    let lines = input.read().await;
    let vec: Vec<u32> = lines.first().unwrap().split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    for i  in 0..100 {
        for j in 0..100 {
            let result = run_computer(&mut vec.clone(), i as u32, j as u32);
            if result == 19690720 {
                return (100 * i + j).to_string();
            }
        }
    }
    panic!("Failed to find noun and verb")
}