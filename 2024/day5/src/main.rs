use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

type Ordering = HashMap<u32, Vec<u32>>;


fn main() {
    let path: String = "test_inp".to_string();
    let (ordering, pages): (Ordering, Vec<Vec<u32>>) = parse_input(path);

    let ans1: u32 = part1(ordering.clone(), pages.clone());
    println!("{}", ans1);

    let ans2: u32 = part2(ordering, pages);
    println!("{}", ans2);
}

fn parse_input(path: String) -> (Ordering, Vec<Vec<u32>>) {
    let input: String = fs::read_to_string(path).expect("Failed reading input");

    let tmp = input.split("\n\n").map(|x| x.to_string()).collect::<Vec<String>>();
    let ordering_raw: String = tmp[0].clone();
    let pages_raw: String = tmp[1].clone();
    
    let mut ordering_hash: Ordering = HashMap::new();

    for line in ordering_raw.split("\n") {
        let pair: Vec<u32> = line.split("|")
                                .map(|x| x.parse::<u32>().expect("Could not parse ordering"))
                                .collect();

        match ordering_hash.entry(pair[0]) {
            Entry::Vacant(e) => {e.insert(vec![pair[1]]);},
            Entry::Occupied(mut e) => {e.get_mut().push(pair[1]); }
        }
    }

    let pages_iter = pages_raw.split("\n");
    let length = pages_iter.clone().count();
    let mut pages: Vec<Vec<u32>> = Vec::new();
    for line in pages_iter.take(length-1) {
        let page: Vec<u32> = line.split(",")
                                .map(|x| x.parse::<u32>().expect("could not parse pages"))
                                .collect();

        pages.push(page);
    }

    (ordering_hash, pages)
}

fn part1(ordering: Ordering, pages: Vec<Vec<u32>>) -> u32 {
    let mut checksum: u32 = 0;
    for line in pages {
        if check_correctness(ordering.clone(), line.clone()) {
            checksum += find_center(line);
        }
    }
    checksum
}

fn check_correctness(ordering: Ordering, line:  Vec<u32>) -> bool {
    for (index, page) in line.iter().enumerate() {
        if ordering.contains_key(page) {
            for previous_page in &line[0..index] {
                if ordering[page].contains(previous_page) {
                    return false
                }
            }
        }
    }
    true
}

fn find_center(line: Vec<u32>) -> u32 {
    let length: usize = line.len();
    let half: usize = (length - 1) / 2; // There are only odd-length inputs
    let mid = line.get(half).expect("could not get middle value of vector");
    *mid
}

fn part2(ordering: Ordering, pages: Vec<Vec<u32>>) -> u32 {
    let mut checksum: u32 = 0;
    for line in pages {
        if check_correctness(ordering.clone(), line.clone()) {
           continue 
        } else {
            let newline = fix_pages(ordering.clone(), line.clone());
            checksum += find_center(newline);

        }
    }
    checksum
}

fn fix_pages(ordering: Ordering, mut line: Vec<u32>) -> Vec<u32> {
    while !check_correctness(ordering.clone(), line.clone()) {
        'outer: for (index, page) in line.iter().enumerate() {
            if index == 0 {continue}
            if ordering.contains_key(page) {
                for (index2, previous_page) in (&line[0..index]).iter().enumerate() {
                    if ordering[page].contains(previous_page) {
                        let popped = line.remove(index2);
                        line.insert(index, popped);
                        break 'outer;
                    }
                }
            }
        }
    } 
    line
}
