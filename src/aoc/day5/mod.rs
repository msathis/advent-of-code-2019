use crate::aoc::utils::Input;

fn split_opcode(n: i32) -> Vec<i32> {
    let mut vec = vec![];
    let mut code = n;

    let op = code % 100;
    vec.push(op);
    code = code / 100;

    while code >= 10 {
        let digit = code % 10;
        code = code / 10;
        vec.push(digit);
    }

    vec.push(code);
    vec
}

fn get_value(vec: &Vec<i32>, mode: i32, value: i32) -> i32 {
    if mode == 0 {
        return *vec.get(dbg!(value) as usize).unwrap() as i32;
    }
    value
}

fn run_computer(vec: &mut Vec<i32>, input: i32) -> i32 {
    let mut index = 0;

    let mut output = 0;

    while index < vec.len() {
        let code = split_opcode(dbg!(*vec.get(index).unwrap()));

        match code.get(0).unwrap() {
            1 => {
                let op1: i32 = get_value(&vec, *code.get(1).unwrap_or(&0), *vec.get(index + 1).unwrap());
                let op2: i32 = get_value(&vec, *code.get(2).unwrap_or(&0), *vec.get(index + 2).unwrap());

                let dest = vec[index + 3] as usize;
                vec[dest] = op1 + op2;
                index = index + 4;
            }
            2 => {
                let op1: i32 = get_value(&vec, *code.get(1).unwrap_or(&0), *vec.get(index + 1).unwrap());
                let op2: i32 = get_value(&vec, *code.get(2).unwrap_or(&0), *vec.get(index + 2).unwrap());

                let dest = vec[index + 3] as usize;
                vec[dest] = op1 * op2;
                index = index + 4;
            }
            3 => {
                let dest = vec[index + 1] as usize;
                vec[dest] = input;
                index = index + 2;
            }
            4 => {
                let op1: i32 = get_value(&vec, *code.get(1).unwrap_or(&0), *vec.get(index + 1).unwrap());
                output = op1;
                index = index + 2;
            }
            5 => {
                let op1: i32 = get_value(&vec, *code.get(1).unwrap_or(&0), *vec.get(index + 1).unwrap());
                let op2: i32 = get_value(&vec, *code.get(2).unwrap_or(&0), *vec.get(index + 2).unwrap());

                if op1 != 0 {
                    index = op2 as usize;
                } else {
                    index = index + 3;
                }
            }
            6 => {
                let op1: i32 = get_value(&vec, *code.get(1).unwrap_or(&0), *vec.get(index + 1).unwrap());
                let op2: i32 = get_value(&vec, *code.get(2).unwrap_or(&0), *vec.get(index + 2).unwrap());

                if op1 == 0 {
                    index = op2 as usize;
                } else {
                    index = index + 3;
                }
            }
            7 => {
                let op1: i32 = get_value(&vec, *code.get(1).unwrap_or(&0), *vec.get(index + 1).unwrap());
                let op2: i32 = get_value(&vec, *code.get(2).unwrap_or(&0), *vec.get(index + 2).unwrap());
                let dest = vec[index + 3] as usize;

                if op1 < op2 {
                    vec[dest] = 1;
                } else {
                    vec[dest] = 0;
                }
                index = index + 4;
            }
            8 => {
                let op1: i32 = get_value(&vec, *code.get(1).unwrap_or(&0), *vec.get(index + 1).unwrap());
                let op2: i32 = get_value(&vec, *code.get(2).unwrap_or(&0), *vec.get(index + 2).unwrap());
                let dest = vec[index + 3] as usize;

                if op1 == op2 {
                    vec[dest] = 1;
                } else {
                    vec[dest] = 0;
                }
                index = index + 4;
            }
            _ => break
        }
    }

    output
}

pub async fn part1() -> String {
    let input = Input {
        path: "src/aoc/day5/input.txt".to_string()
    };
    let lines = input.read().await;
    let mut vec: Vec<i32> = lines.first().unwrap().split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let result = run_computer(&mut vec, 1);
    result.to_string()
}

pub async fn part2() -> String {
    let input = Input {
        path: "src/aoc/day5/input.txt".to_string()
    };
    let lines = input.read().await;
    let mut vec: Vec<i32> = lines.first().unwrap().split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let result = run_computer(&mut vec, 5);
    result.to_string()
}