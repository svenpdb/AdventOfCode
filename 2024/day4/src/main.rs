use std::fs;

fn main() {
    let file_path: &str = "input";

    let input = read_input(file_path);

    let ans1: u32 = part1(input.clone());
    println!("Part 1: {}", ans1);

    let ans2: u32 = part2(input.clone());
    println!("Part 2: {}", ans2);
}

fn read_input(file: &str) -> Vec<String> {
    let mut input = Vec::new();

    for line in fs::read_to_string(file).unwrap().lines() {
        input.push(line.to_string());
    }

    input
}

fn part1(input_array: Vec<String>) -> u32 {
    let mut total: u32 = 0; 

    total += check_horizontal(&input_array);
    total += check_vertical(&input_array);
    total += check_diagonal(&input_array);

    total
}

fn part2(input_array: Vec<String>) -> u32 {
    let mut total: u32 = 0;
    let rows = input_array.len();
    let cols = input_array[0].len();
    let mas = "MAS";
    let sam = "SAM";

    for y in 1..(rows-1) {
        for x in 1..(cols-1) {
            let center = input_array[x as usize]
                                    .chars()
                                    .nth(y as usize)
                                    .unwrap();

            if center != 'A' {
                continue
            }

            let mut diag_p: String = "".to_string();
            
            for i in -1..2 {
                let ix: usize = (x as i32 + i as i32) as usize;
                let iy: usize = (y as i32 + i as i32) as usize;
                let ch = input_array[ix]
                                    .chars()
                                    .nth(iy)
                                    .unwrap();
                diag_p.push(ch);

            }
            if (diag_p != mas) & (diag_p != sam) {
                continue
            }
            
            let mut diag_n: String = "".to_string();

            for i in -1..2 {
                let ix: usize = (x as i32 + i as i32) as usize;
                let iy: usize = (y as i32 - i as i32) as usize;
                let ch = input_array[ix]
                                    .chars()
                                    .nth(iy)
                                    .unwrap();
                diag_n.push(ch);

            }

            if (diag_n != mas) & (diag_n != sam) {
                continue
            }

            total += 1;

        }
    }

    

    total
    
}

fn find_xmas(line: &Vec<String>) -> u32 {
    let length: usize = line.len();
    let forwards: &str = "XMAS";
    let backwards: &str = "SAMX";
    let mut tot_inline: u32 = 0;

    for (index, l) in line.iter().enumerate(){
        if l != "X" {
            continue
        } else {
            if index > 2 {
                let s = &line[index-3..index+1].join("");
                if s == backwards {
                    tot_inline += 1
                }
            }
            if index < length - 3 {
                let s = &line[index..index+4].join("");
                if s == forwards {
                    tot_inline += 1
                }
            }
        }
    }

    tot_inline
}

fn check_horizontal(inp: &Vec<String>) -> u32 {
    let mut tot_hor: u32 = 0;
    
    for line_r in inp {
        let line = line_r
                        .chars()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>();
        tot_hor += find_xmas(&line);
    }
    tot_hor
}

fn check_vertical(inp: &Vec<String>) -> u32 {
    let mut tot_ver: u32 = 0;
    let length = inp[0].len();
    for col in 0..length {
        let line = inp
                        .iter()
                        .map(|x| x.chars().nth(col).unwrap().to_string())
                        .collect::<Vec<_>>();
        tot_ver += find_xmas(&line);
    }
    tot_ver
}

fn check_diagonal(inp: &Vec<String>) -> u32 {
    let mut tot_diag: u32 = 0;
    let length: i32 = inp[0].len() as i32;
    tot_diag += 0; 

    // positive diagonal
    for diag in -length..length {
        let mut line: Vec<String> = Vec::new();
        for xi in 0..length {
            for yi in 0..length {
                if xi == yi + diag {
                    let tmp = inp
                                .iter()
                                .nth(xi as usize)
                                .unwrap()
                                .chars()
                                .nth(yi as usize)
                                .unwrap()
                                .to_string();
                    line.push(tmp);

                }
            }
        }
        if line.len() > 3 {
            tot_diag += find_xmas(&line);
        }
    }

    // negative diagonal
    for diag in 0..2*length - 1 {
        let mut line: Vec<String> = Vec::new();
        for xi in 0..length {
            for yi in 0..length {
                if xi + yi == diag {
                    let tmp = inp
                                .iter()
                                .nth(xi as usize)
                                .unwrap()
                                .chars()
                                .nth(yi as usize)
                                .unwrap()
                                .to_string();
                    line.push(tmp);

                }
            }
        }
        if line.len() > 3 {
            tot_diag += find_xmas(&line);
        }
    }

    tot_diag
}


