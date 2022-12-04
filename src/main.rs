#![feature(iter_array_chunks)]
use day1::{day1_part1, day1_part2};
use day2::{day2_part1, day2_part2};
use day3::{day3_part1, day3_part2};
mod day1;
mod day2;
mod day3;
fn main() {
    // Day1
    println!("Day 1");
    println!("day1 part 1 = {}", day1_part1().unwrap());
    println!("day1 part 2 = {}", day1_part2());
    // Day2
    println!("Day 2");
    println!("day2 part 1 = {}", day2_part1());
    println!("day2 part 2 = {}", day2_part2());
    // Day3
    println!("Day 3");
    println!("day3 part 1 = {}", day3_part1().unwrap());
    println!("day3 part 2 = {}", day3_part2().unwrap());
}
