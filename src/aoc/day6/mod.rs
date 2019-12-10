use crate::aoc::utils::Input;
use std::collections::HashMap;
use petgraph::graphmap::UnGraphMap;
use petgraph::algo::dijkstra;

pub async fn part1() -> String {
    let input = Input {
        path: "src/aoc/day6/input.txt".to_string()
    };
    let lines = input.read().await;
    let items: Vec<(String, String)> = lines.iter().map(|o| {
        let vec: Vec<String> = o.clone().split(")").map(|s| s.to_string()).collect::<Vec<String>>();
        (vec[0].clone(), vec[1].clone())
    }).collect();

    let mut map = HashMap::new();
    for (prev_name, curr_name) in &items {
        map.insert(curr_name.clone(), prev_name.clone());
    }

    let mut count = 0;
    for (key, value) in &map {
        let mut parent = &map.get(value.as_str());
        let mut local_count = 1;
        let mut item;

        loop {
            match *parent {
                Some(_) => {
                    local_count = local_count + 1;
                    let curr = parent.unwrap();

                    item = map.get(curr);
                    parent = &item;
                }
                None => {
                    break;
                }
            }
        }
        count = count + local_count;
    }

    count.to_string()
}

pub async fn part2() -> String {
    let input = Input {
        path: "src/aoc/day6/input.txt".to_string()
    };
    let lines = input.read().await;
    let items: Vec<(String, String)> = lines.iter().map(|o| {
        let vec: Vec<String> = o.clone().split(")").map(|s| s.to_string()).collect::<Vec<String>>();
        (vec[0].clone(), vec[1].clone())
    }).collect();

    let mut graph = UnGraphMap::new();
    for (prev_name, curr_name) in &items {
        if !graph.contains_node(prev_name) {
            graph.add_node(prev_name);
        }
        if !graph.contains_node(curr_name) {
            graph.add_node(curr_name);
        }
        graph.add_edge(prev_name, curr_name, 1);
    }

    let you = "YOU".to_string();
    let san = "SAN".to_string();

    let map = dijkstra(&graph, &you, Some(&san), |_| 1);
    map.get(&san).unwrap() - 2
}