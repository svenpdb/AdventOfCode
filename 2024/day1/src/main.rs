use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path: &str = "input";
    let input: String = fs::read_to_string(file_path).expect("Cannot read the file.");

    let res1: usize = part_one(&input);

    println!("{res1}");

    let res2: usize = part_two(&input);
    println!("{res2}");
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    input.lines().map(|line| {
        let (num1, num2) = line.split_once("   ").expect("Failed splitting line");
        (
            num1.parse::<usize>().expect("Could not parse num1"),
            num2.parse::<usize>().expect("Could not parse num1"),
        )
    }).collect()
}

fn part_one(input: &str) -> usize {
    let (mut list1, mut list2) = parse_input(input);
    list1.sort();
    list2.sort();

    let diff: Vec<usize> = list1.into_iter().zip(list2).map(|(a, b)| (a as i32 - b as i32).abs() as usize).collect();
    diff.iter().sum()
}


fn count_occ(list: Vec<usize>) -> HashMap<usize, usize> {
    let mut counts = HashMap::new();

    for &el in &list {
        let count = counts.entry(el).or_insert(0);
        *count += 1;
    }
    counts
}

fn part_two(input: &str) -> usize {
    let (list1, list2): (Vec<usize>, Vec<usize>) = parse_input(&input);
    let counts = count_occ(list2);

    let mut tot = 0;
    for &el in &list1 {
        if counts.contains_key(&el) {
            tot += el * counts[&el];
        }
    }
    tot
}