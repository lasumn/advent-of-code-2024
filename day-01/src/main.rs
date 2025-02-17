use std::collections::HashMap;

fn part_1(list_1: &Vec<i32>, list_2: &Vec<i32>) {
    let differences: Vec<i32> = list_1
        .iter()
        .zip(list_2)
        .map(|(a, b)| i32::abs(a - b))
        .collect();

    let sum: i32 = differences.iter().sum();

    println!("{sum}");
}

fn part_2(list_1: &Vec<i32>, list_2: &Vec<i32>) {
    let mut num_count: HashMap<i32, i32> = HashMap::new();

    list_2.iter().for_each(|e| {
        let current_count = num_count.entry(*e).or_default();
        *current_count += 1;
    });

    let list_1_scaled: Vec<i32> = list_1
        .iter()
        .map(|e| e * num_count.get(e).unwrap_or(&0))
        .collect();

    let sum: i32 = list_1_scaled.iter().sum();

    println!("{sum}");
}

fn main() {
    let input = std::fs::read_to_string("./input.txt")
        .expect("input file missing or in the wrong location");

    let mut list_1: Vec<i32> = vec![];
    let mut list_2: Vec<i32> = vec![];

    input
        .split_whitespace()
        .enumerate()
        .for_each(|(i, e)| match i % 2 == 0 {
            true => list_1.push(e.parse().unwrap()),
            false => list_2.push(e.parse().unwrap()),
        });

    list_1.sort();
    list_2.sort();

    part_1(&list_1, &list_2);
    part_2(&list_1, &list_2);
}
