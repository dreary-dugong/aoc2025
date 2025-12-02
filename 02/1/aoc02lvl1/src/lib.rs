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

pub fn run(cfg: Config) -> anyhow::Result<u64> {
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

struct Range {
    low: u64,
    high: u64,
}

fn parse(input: String) -> anyhow::Result<Vec<Range>> {
    Ok(input
        .replace('\n', "")
        .split(',')
        .map(|unparsed| {
            let mut nums = unparsed.split('-');
            let low = nums
                .next()
                .expect("Bad input, missing low value in range")
                .parse()
                .expect("Bad input: low value is not a number");
            let high = nums
                .next()
                .expect("Bad input, missing high value in range")
                .parse()
                .expect("Bad input: high value is not a number");
            Range { low, high }
        })
        .collect())
}

fn process(data: Vec<Range>) -> u64 {
    data.into_iter()
        .map(|range| {
            // get a representation of the lowest invalid id
            let low_digit_count = range.low.ilog10() + 1;

            // if the digit count is even, we need to test if the first half of
            // the digits can just be doubled. They can't if the second half is
            // a higher number. In that case, add 1 to the number represented by
            // the first half and now we can double it
            let mut lowest_invalid_prefix;
            if low_digit_count % 2 == 0 {
                let low_suffix = range.low % 10_u64.pow(low_digit_count / 2);
                lowest_invalid_prefix = range.low / 10_u64.pow(low_digit_count / 2);
                if low_suffix > lowest_invalid_prefix {
                    lowest_invalid_prefix += 1;
                }
            // if the digit count is odd, we need to find the first number above
            // it with an even digit count so we can take the prefix and double that
            } else {
                lowest_invalid_prefix = 10_u64.pow((low_digit_count / 2) + 1);
            }

            // now get a a representation of the highest invalid id
            let high_digit_count = range.low.ilog10() + 1;
            let highest_invalid_prefix;
            // if the number of digits is odd, the highest invalid is 9999...
            // one less than the number of digits
            if high_digit_count % 2 == 1 {
                highest_invalid_prefix = 10_u64.pow(high_digit_count / 2 + 1) - 1;
            } else {
                let high_prefix = range.high / 10_u64.pow(high_digit_count / 2);
                let high_suffix = range.high % 10_u64.pow(high_digit_count / 2);
                // if the number of digits is even and the second half is higher,
                // we can double the first half
                if high_suffix >= high_prefix {
                    highest_invalid_prefix = high_prefix

                // if the number of digits is even and the first half is higher,
                // we can double the first half minus 1
                } else {
                    highest_invalid_prefix = high_prefix - 1;
                }
            }

            // to get all the invalid ids, we double every number between
            // the lowest invalid prefix and the highest invalid prefix
            (lowest_invalid_prefix..=highest_invalid_prefix)
                .map(double)
                .sum::<u64>()
        })
        .sum()
}

// given a number n, return the number formed by repeating n's digits after n
fn double(prefix: u64) -> u64 {
    let digit_count = prefix.ilog10() + 1;
    prefix * 10_u64.pow(digit_count) + prefix
}
