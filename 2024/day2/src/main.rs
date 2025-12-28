use std::fs;

fn main() {
    let path = "input";
    // let path = "tmp";
    let input = fs::read_to_string(path).expect("Could not read file");

    let res1 = part_one(&input);
    println!("{res1}");

    let res2: usize = part_two(&input);
    println!("{res2}");
}

fn part_one(input: &str) -> usize {
    let mut count: usize = 0;
    for line in input.lines() {
        let vec: Vec<i32> = line
                            .split_whitespace()
                            .map(|x: &str| x.parse::<i32>().expect("failed parsing input"))
                            .collect();
        if determine_safe_p1(vec) {
            count += 1;
        }
    }
    count
}

fn part_two(input: &str) -> usize {
    let mut count: usize = 0;
    for line in input.lines() {
        let vec: Vec<i32> = line
                            .split_whitespace()
                            .map(|x: &str| x.parse::<i32>().expect("failed parsing input"))
                            .collect();
        if determine_safe_p2(vec) {
            count += 1;
        }
    }
    count
    
}

fn compute_direction(a: i32, b: i32) -> i32 {
    if a == b {
        0
    } else {    
    (a - b) / (a-b).abs()
    }
}

fn determine_safe_p1(seq: Vec<i32>) -> bool {
    let length: usize = seq.len();
    let mut safe: bool = true;
    let direction: i32 = compute_direction(seq[1],seq[0]);
    for index in 0..length-1 {
        let diff: i32 = seq[index + 1] - seq[index];
        if (diff.abs() > 3) | (diff.abs() < 1) {
            safe = false;
            break;
        }
        if direction != compute_direction(seq[index+1], seq[index]) {
            safe = false;
            break;
        }
    }
    safe
}

fn determine_safe_p2(seq: Vec<i32>) -> bool {
    let length: usize = seq.len();
    let mut safe: bool = true;

    for index in 0..length-1 {
        let diff: i32 = seq[index + 1] - seq[index];
        if (diff.abs() > 3) | (diff.abs() < 1) {
            safe = false;
        }
        if index != 0 {
            if compute_direction(seq[index], seq[index-1]) != compute_direction(seq[index+1], seq[index]) {
                safe = false;
            }
        }
    }
    if !safe {
        for index in 0..length {        
            let mut newseq: Vec<i32> = seq.clone();
            newseq.remove(index);
            safe = determine_safe_p1(newseq);
            if safe {
                break;
            }
        }

    }
    safe
}