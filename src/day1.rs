use std::fs;
pub fn day1_part1() -> Option<u32> {
    let contents = fs::read_to_string("data/day1-input.txt").unwrap();
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

pub fn day1_part2() -> u32 {
    let contents = fs::read_to_string("data/day1-input.txt").unwrap();
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
    elves.iter().take(3).sum()
}
