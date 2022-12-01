use std::fs;
fn day1() -> Option<u32> {
    let contents = fs::read_to_string("day1-input.txt").unwrap();
    let mut elves: Vec<u32> = Vec::new();

    for elf in contents.split("\n\n") {
        let mut sum = 0;
        for quantity in elf.split('\n') {
            let quantity = quantity.parse::<u32>().unwrap_or(0);
            sum += quantity;
        }
        elves.push(sum);
    }
    elves.sort();
    elves.pop()
}

fn day1_part2() -> u32 {
    let contents = fs::read_to_string("day1-input.txt").unwrap();
    let mut elves: Vec<u32> = Vec::new();

    for elf in contents.split("\n\n") {
        let mut sum = 0;
        for quantity in elf.split('\n') {
            let quantity = quantity.parse::<u32>().unwrap_or(0);
            sum += quantity;
        }
        elves.push(sum);
    }
    elves.sort();
    let mut top_three_sum = 0;
    for _ in 0..3 {
        top_three_sum += elves.pop().unwrap_or(0);
    }
    top_three_sum
}

fn main() {
    println!("day1 part 1 = {}", day1().unwrap());
    println!("day1 part 2 = {}", day1_part2());
}
