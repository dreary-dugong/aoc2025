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
    let mut val = 50;
    let mut zero_count = 0;

    for (direction, count) in data.into_iter() {
        let mut count = count as i32;
        match direction {
            Direction::Right => {
                while count >= 100 {
                    count -= 100;
                    zero_count += 1;
                }
                val += count;
                if val >= 100 {
                    zero_count += 1;
                    val -= 100;
                }
            }
            Direction::Left => {
                while count >= 100 {
                    count -= 100;
                    zero_count += 1;
                }
                val -= count;
                if val == 0 && count != 0 {
                    zero_count += 1;
                }
                if val < 0 {
                    if (val + count) != 0 {
                        zero_count += 1;
                    }
                    val += 100;
                }
            }
        };
    }

    zero_count
}
