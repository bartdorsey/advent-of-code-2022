use anyhow::Result;
use std::fs;

pub fn part1() -> Result<u32> {
    let contents = fs::read_to_string("data/day7-test.txt")?;
    let lines = contents.lines();
    for line in lines {
        if line.starts_with('$') {}
    }

    Ok(0)
}

pub fn part2() -> Result<u32> {
    let contents = fs::read_to_string("data/day7-test.txt")?;
    Ok(0)
}
