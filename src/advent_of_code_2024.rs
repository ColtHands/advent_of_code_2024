use std::fs::File;
use std::io::Read;

pub fn read_file(path: String) -> String {
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Unable to read file");

    return contents;
}

pub fn day1() {
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

    arr1.sort();
    arr2.sort();

    let mut diff = 0;

    for i in 0..arr1.len() {
        diff += (arr1[i] - arr2[i]).abs();
    }

    println!("{:?}", (diff));
}