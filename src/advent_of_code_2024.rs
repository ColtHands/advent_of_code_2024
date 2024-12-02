use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn read_file(path: String) -> String {
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Unable to read file");

    return contents;
}

fn parse_contents_by_line_by_word_as_integer(path: String) -> Vec<Vec<i32>> {
    let contents = read_file(path);
    let contents_parsed = contents
        .lines()
        .map(|line| {
            line
                .split_whitespace()
                .map(|word| word.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    return contents_parsed
}

pub fn day2_pt1() -> i32 {
    let contents = parse_contents_by_line_by_word_as_integer("inputs/day2.txt".to_string());

    let mut safe_count = 0;

    for line in contents {
        safe_count += if is_line_safe(&line) { 1 } else { 0 };
    }

    return safe_count;
}

pub fn day2_pt2() -> usize {
    let contents = parse_contents_by_line_by_word_as_integer("inputs/day2.txt".to_string());

    let mut safe_count = 0;

    for line in contents {
        if is_line_safe(&line) {
            safe_count += 1;
        } else {
            for i in 0..line.len() {
                let mut modified_line = line.clone();
                modified_line.remove(i);
                if is_line_safe(&modified_line) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }

    return safe_count;
}

fn is_line_safe(line: &Vec<i32>) -> bool {
    let is_decreasing = line[0] > line[1];

    for i in 1..line.len() {
        if (line[i-1] > line[i]) != is_decreasing {
            return false;
        }

        let diff = (line[i] - line[i - 1]).abs();

        if diff < 1 || diff > 3 {
            return false
        }
    }

    return true;
}

fn day1_contents() -> (Vec<i32>, Vec<i32>) {
    let contents = read_file("inputs/day1.txt".to_string());
    let contents_parsed = contents
        .split("\r\n")
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut arr1 = Vec::new();
    let mut arr2 = Vec::new();

    for pair in contents_parsed {
        arr1.push(pair[0].parse::<i32>().unwrap());
        arr2.push(pair[1].parse::<i32>().unwrap());
    }

    return (arr1, arr2);
}

fn day1_pt2() -> i32 {
    let mut count_map = HashMap::new();

    let (mut arr1, arr2) = day1_contents();

    for i in 0..arr2.len() {
        if count_map.contains_key(&arr2[i]) {
            count_map.insert(arr2[i], count_map[&arr2[i]] + 1);
        } else {
            count_map.insert(arr2[i], 1);
        }
    }

    let mut total: i32 = 0;

    for num in arr1.iter_mut() {
        if count_map.contains_key(&num) {
            total += *num * count_map[num];
        }
    }

    return total;
}

fn day1_pt1() -> i32 {
    let (mut arr1, mut arr2) = day1_contents();

    arr1.sort();
    arr2.sort();

    let mut diff = 0;

    for i in 0..arr1.len() {
        diff += (arr1[i] - arr2[i]).abs();
    }

    return diff;
}
