use std::fs;
use std::io;
use std::path::PathBuf;

extern crate clap;
use clap::Parser;

extern crate anyhow;

#[derive(Parser, Debug)]
pub struct Args {
    /// path to the input file
    #[arg(short, long)]
    input: Option<PathBuf>,
}

enum InputConfig {
    File(PathBuf),
    Stdin,
}
pub struct Config {
    input: InputConfig,
}

impl Config {
    pub fn make() -> Self {
        let args = Args::parse();
        let input = if let Some(path) = args.input {
            InputConfig::File(path)
        } else {
            InputConfig::Stdin
        };

        Config { input }
    }
}

pub fn run(cfg: Config) -> anyhow::Result<i32> {
    // figure out where to get our input from and read it into a string
    let input_string = match cfg.input {
        InputConfig::File(path) => fs::read_to_string(path)?,
        InputConfig::Stdin => {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf)?;
            buf
        }
    };

    let data = parse(input_string)?;
    let result = process(data);

    println!("{}", result);

    Ok(result)
}

enum Direction {
    Right,
    Left,
}

fn parse(input: String) -> anyhow::Result<Vec<(Direction, u32)>> {
    Ok(input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let direction = match chars.next().expect("empty line") {
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => panic!("bad input line, expected R or L"),
            };

            let count = chars
                .collect::<String>()
                .parse::<u32>()
                .expect("invalid turn count, expected a number");

            (direction, count)
        })
        .collect())
}

fn process(data: Vec<(Direction, u32)>) -> i32 {
    let mut dial = 50;
    let mut zero_count = 0;

    for (direction, count) in data.into_iter() {
        let mut count = count as i32;

        // count full rotations
        zero_count += count / 100;
        count %= 100;

        // if the count is 0, nothing happens
        // we handle that edge case here to simplify logic later
        if count == 0 {
            continue;
        }

        // account for the difference between going left and going right
        match direction {
            Direction::Right => {
                // if we make a full rotation by adding, count it and
                // ensure the dial stays under 100
                dial += count;
                zero_count += dial / 100;
                dial %= 100;
            }
            Direction::Left => {
                // if the dial was already at 0, don't double count it
                // otherwise count it if we rotate past 0
                if dial != 0 && count >= dial {
                    zero_count += 1;
                }

                // python lets you do this by modding a negative
                // maybe rust has an easier way? Wrapping sub with arbirary max_int?
                dial -= count;
                if dial < 0 {
                    dial += 100;
                }
            }
        };
    }

    zero_count
}
