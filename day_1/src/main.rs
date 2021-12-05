use std::fs;

// https://adventofcode.com/2021/day/1

fn main() {
    let input_string = fs::read_to_string("input.txt")
        .unwrap_or(String::from("Error!"));
    println!("{}", part_one(&input_string));
    println!("{}", part_two(&input_string));
}

fn count_increases(numbers: Vec<u32>) -> u32 {
    let mut last: u32 = 0;
    numbers.iter().map(|x| {
        if last == 0 {
            last = *x;
            return 0;
        } else {
            if *x > last {
                last = *x;
                return 1;
            } else {
                last = *x;
                return 0;
            }
        }
    }).fold(0, |acc, x| acc + x)
}

fn part_one(input: &String) -> u32 {
    count_increases(input.lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
    )
}

fn part_two(input: &String) -> u32 {
    count_increases(input.lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()[0..]
        .windows(3)
        .map(|x| x.iter()
            .collect::<Vec<&u32>>()
            .iter()
            .fold(0, |acc, y| acc + *y)
        )
        .collect()
    )
}