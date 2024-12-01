use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub fn read_file(path: String) -> String {
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Unable to read file");

    return contents;
}

pub fn day1_contents() -> (Vec<i32>, Vec<i32>) {
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

pub fn day1_pt1() -> i32 {
    let (mut arr1, mut arr2) = day1_contents();

    arr1.sort();
    arr2.sort();

    let mut diff = 0;

    for i in 0..arr1.len() {
        diff += (arr1[i] - arr2[i]).abs();
    }

    return diff;
}

pub fn day1_pt2() -> i32 {
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
