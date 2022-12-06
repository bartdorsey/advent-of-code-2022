use anyhow::{anyhow, Result};
use std::{fs, vec};

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

pub fn part1() -> Result<String> {
    let contents = fs::read_to_string("data/day5-input.txt")?;
    let pieces = contents.split("\n\n").take(2);

    if let [dock, instructions] = pieces.collect::<Vec<&str>>()[..] {
        let mut stacks: Vec<Vec<char>> = vec![];
        let lines = dock.lines().rev().skip(1);
        for line in lines {
            // println!("New Line");
            let mut stack_index = 0;
            let mut char_index = 0;
            let chars = line.chars().collect::<Vec<char>>();
            while char_index < chars.len() {
                // dbg!(stack_index);
                // dbg!(char_index);
                let char = chars[char_index];
                // dbg!(&char);
                if char == '[' {
                    let letter = chars[char_index + 1];
                    // dbg!(letter);
                    let stack = stacks.get_mut(stack_index);
                    match stack {
                        Some(stack) => stack.push(letter),
                        None => stacks.push(vec![letter]),
                    }
                    // dbg!(&stacks);
                }
                stack_index += 1;
                char_index += 4;
            }
        }
        let mut moves: Vec<Move> = vec![];
        for instruction in instructions.lines() {
            let mut i = vec![];
            for x in instruction.split_whitespace() {
                if let Ok(parsed) = x.parse::<usize>() {
                    i.push(parsed);
                }
            }
            moves.push(Move {
                count: i[0],
                from: i[1],
                to: i[2],
            })
        }
        for m in moves {
            for _ in 0..m.count {
                if let Some(item) = stacks[m.from - 1].pop() {
                    stacks[m.to - 1].push(item);
                }
            }
        }
        let mut result = String::from("");
        for stack in stacks {
            result.push(*stack.last().unwrap());
        }
        Ok(result)
    } else {
        Err(anyhow!("Couldn't parse string"))
    }
}

pub fn part2() -> Result<String> {
    let contents = fs::read_to_string("data/day5-input.txt")?;
    let pieces = contents.split("\n\n").take(2);

    if let [dock, instructions] = pieces.collect::<Vec<&str>>()[..] {
        let mut stacks: Vec<Vec<char>> = vec![];
        let lines = dock.lines().rev().skip(1);
        for line in lines {
            // println!("New Line");
            let mut stack_index = 0;
            let mut char_index = 0;
            let chars = line.chars().collect::<Vec<char>>();
            while char_index < chars.len() {
                // dbg!(stack_index);
                // dbg!(char_index);
                let char = chars[char_index];
                // dbg!(&char);
                if char == '[' {
                    let letter = chars[char_index + 1];
                    // dbg!(letter);
                    let stack = stacks.get_mut(stack_index);
                    match stack {
                        Some(stack) => stack.push(letter),
                        None => stacks.push(vec![letter]),
                    }
                    // dbg!(&stacks);
                }
                stack_index += 1;
                char_index += 4;
            }
        }
        let mut moves: Vec<Move> = vec![];
        for instruction in instructions.lines() {
            let mut i = vec![];
            for x in instruction.split_whitespace() {
                if let Ok(parsed) = x.parse::<usize>() {
                    i.push(parsed);
                }
            }
            moves.push(Move {
                count: i[0],
                from: i[1],
                to: i[2],
            })
        }
        for m in moves {
            let mut temp: Vec<char> = vec![];
            for _ in 0..m.count {
                if let Some(item) = stacks[m.from - 1].pop() {
                    temp.push(item);
                    // stacks[m.to - 1].push(item);
                }
            }
            temp.reverse();
            stacks[m.to - 1].append(&mut temp);
        }
        let mut result: String = String::from("");
        for stack in stacks {
            result.push(*stack.last().unwrap());
        }
        Ok(result)
    } else {
        Err(anyhow!("Couldn't parse string"))
    }
}
