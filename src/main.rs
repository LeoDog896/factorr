use std::io::stdin;

use atty::Stream;
use clap::Parser;

fn factor(mut input: u128) -> Vec<u128> {
    let mut factors = Vec::new();

    let mut candidates = 2..;

    while input > 1 {
        let x = candidates.next().unwrap();

        while input % x == 0 {
            input /= x;

            factors.push(x);
        }
    }

    factors
}

/// Gets the prime factors for any integer. It will use stdin if none are specified.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// List of numbers to factor.
    numbers: Option<Vec<u128>>
}

fn main() {
    let cli = Cli::parse();

    if let Some(numbers) = cli.numbers {
        for input in numbers {
            let factors = factor(input);

            println!(
                "{}",
                factors
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
    } else {
        if atty::is(Stream::Stdin) {
            eprintln!("STDIN expected. Make sure to pipe something in, or run with arguments. Run factorr --help for more information.");
        } else {
            for line in stdin().lines() {
                let line = line.unwrap();

                let input = line.parse::<u128>();

                match input {
                    Ok(input) => {
                        let factors = factor(input);

                        println!(
                            "{}",
                            factors
                                .iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>()
                                .join(" ")
                        );
                    },
                    Err(_) => {
                        eprintln!("Expected a number, got {} instead.", line);
                        return;
                    }
                }
            }
        }
    }
}
