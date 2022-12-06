use anyhow::Result;
use std::collections::BTreeSet;
use std::fs;

pub fn part1() -> Result<u32> {
    const WINDOW_SIZE: usize = 4;
    let contents = fs::read_to_string("data/day6-input.txt")?;
    let chars: Vec<char> = contents.chars().collect();
    let (count, _) = chars
        .windows(WINDOW_SIZE)
        .enumerate()
        .find(|(_i, window)| {
            let set = window.iter().collect::<BTreeSet<&char>>();
            set.len() == WINDOW_SIZE
        })
        .unwrap();
    Ok((count + WINDOW_SIZE) as u32)
}

pub fn part2() -> Result<u32> {
    const WINDOW_SIZE: usize = 14;
    let contents = fs::read_to_string("data/day6-input.txt")?;
    let chars: Vec<char> = contents.chars().collect();
    let (count, _) = chars
        .windows(WINDOW_SIZE)
        .enumerate()
        .find(|(_i, window)| {
            let set = window.iter().collect::<BTreeSet<&char>>();
            set.len() == WINDOW_SIZE
        })
        .unwrap();
    Ok((count + WINDOW_SIZE) as u32)
}
