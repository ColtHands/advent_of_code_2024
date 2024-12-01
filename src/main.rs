mod advent_of_code_2024;
use advent_of_code_2024::{
    day1_pt1,
    day1_pt2
};

fn main() {
    log!("Day 1 result", day1_pt1());
    log!("Day 1 result", day1_pt2());
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        println!("{:?}", ($($arg)*));
    }
}