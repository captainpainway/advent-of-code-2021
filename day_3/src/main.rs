use parse_input;
use std::collections::HashMap;

// https://adventofcode.com/2021/day/3

fn main() {
    let input_string = parse_input::parse("input.txt");
    println!("{}", part_one(&input_string));
    println!("{}", part_two(&input_string));
}

fn find_most_common_bits(input: &Vec<String>) -> HashMap<usize, char> {
    let len = input[0].len();
    let mut hash_map = HashMap::new();

    for y in 0..len {
        let mut tmp_string = "".to_owned();
        for x in input {
            tmp_string = tmp_string + &((x.as_bytes()[y] as char).to_string());
        }

        let zeroes = tmp_string.matches("0").count();
        let ones = tmp_string.matches("1").count();

        if zeroes > ones {
            hash_map.insert(y, '0');
        } else if zeroes == ones {
            hash_map.insert(y, '1');
        } else {
            hash_map.insert(y, '1');
        }
    }
    hash_map
}

fn find_least_common_bits(input: &Vec<String>) -> HashMap<usize, char> {
    let len = input[0].len();
    let mut hash_map = HashMap::new();

    for y in 0..len {
        let mut tmp_string = "".to_owned();
        for x in input {
            tmp_string = tmp_string + &((x.as_bytes()[y] as char).to_string());
        }

        let zeroes = tmp_string.matches("0").count();
        let ones = tmp_string.matches("1").count();

        if zeroes < ones {
            hash_map.insert(y, '0');
        } else if zeroes == ones {
            hash_map.insert(y, '0');
        } else {
            hash_map.insert(y, '1');
        }
    }
    hash_map
}

fn part_one(input: &Vec<String>) -> isize {
    let most_common_bits = find_most_common_bits(input);

    let mut gamma: String = String::new();
    for x in 0..most_common_bits.keys().len() {
        gamma.push(most_common_bits[&x]);
    }

    let epsilon: String = gamma.chars().map(|x| match x {
        '0' => '1',
        '1' => '0',
        _ => x
    }).collect();

    let gamma_num = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_num = isize::from_str_radix(&epsilon, 2).unwrap();

    gamma_num * epsilon_num
}

fn part_two(input: &Vec<String>) -> isize {
    let input_length = input[0].len();

    let mut inputs = input.clone();
    for x in 0..input_length {
        let mut removes = Vec::new();
        let most_common_bits = find_most_common_bits(&inputs);
        for y in 0..inputs.len() {
            if inputs[y].as_bytes()[x] as char == most_common_bits[&x] {
                removes.push(y);
            }
        }
        inputs = removes.iter().rev().map(|x| inputs.remove(*x)).collect();
        if inputs.len() == 1 {
            break;
        }
    }
    let og_rating = &inputs[0];

    let mut inputs = input.clone();
    for x in 0..input_length {
        let mut removes = Vec::new();
        let most_common_bits = find_least_common_bits(&inputs);
        for y in 0..inputs.len() {
            if inputs[y].as_bytes()[x] as char == most_common_bits[&x] {
                removes.push(y);
            }
        }
        inputs = removes.iter().rev().map(|x| inputs.remove(*x)).collect();
        if inputs.len() == 1 {
            break;
        }
    }
    let co2_rating = &inputs[0];

    let og_num = isize::from_str_radix(&og_rating, 2).unwrap();
    let co2_num = isize::from_str_radix(&co2_rating, 2).unwrap();

    og_num * co2_num
}