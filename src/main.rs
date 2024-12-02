mod advent_of_code_2024;
use advent_of_code_2024::{
    // day1_pt1,
    // day1_pt2,
    day2_pt1,
    day2_pt2,
};

fn main() {
    // log!("Day 1 pt 1 result", day1_pt1());
    // log!("Day 1 pt 2 result", day1_pt2());

    log!("Day 2 pt 1 result", day2_pt1());
    log!("Day 2 pt 2 result", day2_pt2());
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        println!("{:?}", ($($arg)*));
    }
}