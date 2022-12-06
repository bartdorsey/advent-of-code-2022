use anyhow::Result;
use std::fs;

const WIN_SCORE: u32 = 6;
const DRAW_SCORE: u32 = 3;
const LOSE_SCORE: u32 = 0;

const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;
const SCISSORS_SCORE: u32 = 3;

const ROCK1: &str = "A";
const PAPER1: &str = "B";
const SCISSORS1: &str = "C";

const ROCK2: &str = "X";
const PAPER2: &str = "Y";
const SCISSORS2: &str = "Z";

const LOSE: &str = "X";
const DRAW: &str = "Y";
const WIN: &str = "Z";

pub fn part1() -> Result<u32> {
    let contents = fs::read_to_string("data/day2-input.txt")?;
    let rounds = contents.lines();
    let mut score = 0;
    for round in rounds {
        let moves = round.split(' ').collect::<Vec<&str>>();
        let p1 = moves[0];
        let p2 = moves[1];
        score += match p2 {
            ROCK2 => ROCK_SCORE,
            PAPER2 => PAPER_SCORE,
            SCISSORS2 => SCISSORS_SCORE,
            _ => 0,
        };
        score += match (p2, p1) {
            (ROCK2, SCISSORS1) => WIN_SCORE,
            (ROCK2, ROCK1) => DRAW_SCORE,
            (PAPER2, ROCK1) => WIN_SCORE,
            (PAPER2, PAPER1) => DRAW_SCORE,
            (SCISSORS2, PAPER1) => WIN_SCORE,
            (SCISSORS2, SCISSORS1) => DRAW_SCORE,
            _ => LOSE_SCORE,
        };
    }
    Ok(score)
}

pub fn part2() -> Result<u32> {
    let contents = fs::read_to_string("data/day2-input.txt")?;
    let rounds = contents.lines();
    let mut score = 0;
    for round in rounds {
        let moves = round.split(' ').collect::<Vec<&str>>();
        let p1 = moves[0];
        let p2 = moves[1];
        score += match p2 {
            WIN => {
                WIN_SCORE
                    + match p1 {
                        ROCK1 => PAPER_SCORE,
                        PAPER1 => SCISSORS_SCORE,
                        SCISSORS1 => ROCK_SCORE,
                        _ => 0,
                    }
            }
            LOSE => {
                LOSE_SCORE
                    + match p1 {
                        ROCK1 => SCISSORS_SCORE,
                        PAPER1 => ROCK_SCORE,
                        SCISSORS1 => PAPER_SCORE,
                        _ => 0,
                    }
            }
            DRAW => {
                DRAW_SCORE
                    + match p1 {
                        ROCK1 => ROCK_SCORE,
                        PAPER1 => PAPER_SCORE,
                        SCISSORS1 => SCISSORS_SCORE,
                        _ => 0,
                    }
            }
            _ => 0,
        };
    }
    Ok(score)
}
